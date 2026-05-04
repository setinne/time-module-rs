// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html

// 对外 C 接口：输出时间，接收配置输入
use std::ffi::CString;
use std::os::raw::c_char;
use crate::error::TimeErrorCode;
use crate::time::handle;
use crate::time::config;
use crate::time::tz_country;
use crate::time::tz_name;
use crate::time::handle::formatting::FullTime;

// ---------- 错误处理辅助函数 ----------
fn result_to_i32(result: Result<(), TimeErrorCode>) -> i32 {
    match result {
        Ok(()) => TimeErrorCode::Success as i32,
        Err(e) => e as i32,
    }
}

// ---------- 输出 ----------
#[no_mangle]
pub extern "C" fn api_GetLocalTime() -> FullTime {
    let (sec, us) = handle::get_calibrated_local_time();
    handle::formatting::utc_to_fulltime(sec, us, config::get_timezone_offset())
}

#[no_mangle]
pub extern "C" fn api_GetNetworkTime() -> FullTime {
    match crate::time::git::ntp::get_cached_utc_time() {
        Some((sec, us)) => handle::formatting::utc_to_fulltime(sec, us, config::get_timezone_offset()),
        None => FullTime { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 0, ms: 0, us: 0 },
    }
}

#[no_mangle]
pub extern "C" fn api_GetFormattedTime() -> *const c_char {
    let ft = api_GetLocalTime();
    let s = format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
        ft.year, ft.month, ft.day, ft.hour, ft.minute, ft.second, ft.ms
    );
    CString::new(s).unwrap_or_default().into_raw()
}

#[no_mangle]
pub extern "C" fn api_FreeString(ptr: *mut c_char) {
    if ptr.is_null() { return; }
    unsafe { drop(CString::from_raw(ptr)); }
}

#[no_mangle]
pub extern "C" fn api_IsNTPSynced() -> bool {
    crate::time::git::ntp::is_ntp_available()
}

// ---------- 带错误码返回的输入 ----------
#[no_mangle]
pub extern "C" fn api_SetTimezoneByLocation(lon: f64, lat: f64, code: *const c_char) -> i32 {
    let result = (|| {
        let country = if code.is_null() { None } else {
            unsafe {
                match std::ffi::CStr::from_ptr(code).to_str() {
                    Ok(s) => Some(s),
                    Err(_) => return Err(TimeErrorCode::InvalidParam),
                }
            }
        };
        let offset = tz_country::offset_from_location(lon, lat, country);
        config::set_timezone_offset(offset).map_err(|_| TimeErrorCode::InvalidParam)
    })();
    result_to_i32(result)
}

#[no_mangle]
pub extern "C" fn api_SetTimezoneOffset(sec: i32) -> i32 {
    let result = config::set_timezone_offset(sec).map_err(|_| TimeErrorCode::InvalidParam);
    result_to_i32(result)
}

#[no_mangle]
pub extern "C" fn api_SetTimezoneByName(name: *const c_char) -> i32 {
    let result = (|| {
        if name.is_null() {
            return Err(TimeErrorCode::InvalidParam);
        }
        let name_str = unsafe {
            std::ffi::CStr::from_ptr(name).to_str().map_err(|_| TimeErrorCode::InvalidParam)?
        };
        let offset = tz_name::get_offset_by_name(name_str).ok_or(TimeErrorCode::InvalidParam)?;
        config::set_timezone_offset(offset).map_err(|_| TimeErrorCode::InvalidParam)
    })();
    result_to_i32(result)
}

#[no_mangle]
pub extern "C" fn api_ForceResync() -> bool {
    crate::time::git::ntp::force_resync()
}

#[no_mangle]
pub extern "C" fn api_ForceResyncEx() -> i32 {
    if crate::time::git::ntp::force_resync() {
        TimeErrorCode::Success as i32
    } else {
        TimeErrorCode::NtpTimeout as i32
    }
}

#[no_mangle]
pub extern "C" fn api_SetAutoSyncEnabled(enabled: bool) {
    config::set_auto_sync_enabled(enabled);
}

#[no_mangle]
pub extern "C" fn api_GetTimezoneOffset() -> i32 {
    config::get_timezone_offset()
}

// ---------- 新增：获取错误码描述 ----------
#[no_mangle]
pub extern "C" fn api_GetErrorString(code: i32) -> *const c_char {
    let err = match code {
        0 => TimeErrorCode::Success,
        1 => TimeErrorCode::InvalidParam,
        2 => TimeErrorCode::NtpTimeout,
        3 => TimeErrorCode::NoNtpServer,
        4 => TimeErrorCode::Timeout,
        5 => TimeErrorCode::NotSynced,
        6 => TimeErrorCode::FileNotFound,
        7 => TimeErrorCode::ParseError,
        _ => TimeErrorCode::InvalidParam,
    };
    CString::new(err.as_str()).unwrap_or_default().into_raw()
}

// ---------- 新增：获取最后错误（如果有全局错误状态需要维护）----------
// 如果需要记录最后错误，可以添加一个全局变量
use std::sync::atomic::{AtomicI32, Ordering};
static LAST_ERROR: AtomicI32 = AtomicI32::new(0);

#[no_mangle]
pub extern "C" fn api_SetLastError(code: i32) {
    LAST_ERROR.store(code, Ordering::Release);
}

#[no_mangle]
pub extern "C" fn api_GetLastError() -> i32 {
    LAST_ERROR.load(Ordering::Acquire)
}