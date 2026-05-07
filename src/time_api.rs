// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 对外 C 接口


use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::atomic::{AtomicI32, Ordering};

use crate::error::TimeErrorCode;
use crate::time::{calc, config, tz, dst};
use crate::time::calc::FullTime;

static LAST_ERROR: AtomicI32 = AtomicI32::new(0);

fn result_to_i32(result: Result<(), TimeErrorCode>) -> i32 {
    match result {
        Ok(()) => TimeErrorCode::Success as i32,
        Err(e) => e as i32,
    }
}


// ---------- Getters ----------
#[no_mangle]
pub extern "C" fn api_GetLocalTime() -> FullTime {
    let (sec, us) = calc::get_calibrated_local_time();
    calc::utc_to_fulltime(sec, us, config::get_timezone_offset())
}

#[no_mangle]
pub extern "C" fn api_GetNetworkTime() -> FullTime {
    match crate::time::core::ntp::get_cached_utc_time() {
        Some((sec, us)) => calc::utc_to_fulltime(sec, us, config::get_timezone_offset()),
        None => FullTime { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 0, ms: 0, us: 0 },
    }
}

#[no_mangle]
pub extern "C" fn api_GetFormattedTime() -> *const c_char {
    let ft = api_GetLocalTime();
    let s = format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
        ft.year, ft.month, ft.day, ft.hour, ft.minute, ft.second, ft.ms);
    CString::new(s).unwrap_or_default().into_raw()
}

#[no_mangle]
pub extern "C" fn api_FreeString(ptr: *mut c_char) {
    if ptr.is_null() { return; }
    unsafe { drop(CString::from_raw(ptr)); }
}

#[no_mangle]
pub extern "C" fn api_IsNTPSynced() -> bool {
    crate::time::core::ntp::is_ntp_available()
}

#[no_mangle]
pub extern "C" fn api_GetTimezoneOffset() -> i32 {
    config::get_timezone_offset()
}

#[no_mangle]
pub extern "C" fn api_GetLastError() -> i32 {
    LAST_ERROR.load(Ordering::Acquire)
}

// 新增：设置 DST 后端
#[no_mangle]
pub extern "C" fn api_SetDSTBackend(backend: i32) {
    match backend {
        1 => crate::time::dst::set_backend(crate::time::dst::DSTBackend::SystemAPI),
        _ => crate::time::dst::set_backend(crate::time::dst::DSTBackend::RuleTable),
    }
}

// 新增：获取当前 DST 后端
#[no_mangle]
pub extern "C" fn api_GetDSTBackend() -> i32 {
    crate::time::dst::get_backend() as i32
}

// 新增：获取系统时区信息
#[no_mangle]
pub extern "C" fn api_GetSystemTimezoneOffset() -> i32 {
    crate::time::dst::get_system_current_bias()
}

// 新增：判断系统是否处于 DST
#[no_mangle]
pub extern "C" fn api_IsSystemDST() -> bool {
    crate::time::dst::is_system_dst()
}

// ---------- Setters ----------
#[no_mangle]
pub extern "C" fn api_SetTimezoneByLocation(lon: f64, lat: f64, code: *const c_char) -> i32 {
    let result = (|| {
        let country = if code.is_null() { None } else {
            Some(unsafe {
                std::ffi::CStr::from_ptr(code).to_str()
                    .map_err(|_| TimeErrorCode::InvalidParam)?
            })
        };
        let base_offset = tz::offset_from_location(lon, lat, country)?;
        
        let (sec, us) = calc::get_calibrated_local_time();
        let local = calc::utc_to_fulltime(sec, us, base_offset);
        
        let final_offset = if let Some(c) = country {
            if dst::is_dst(&local, c) {
                base_offset + dst::get_dst_offset(c)
            } else {
                base_offset
            }
        } else {
            base_offset
        };
        
        config::set_timezone_offset(final_offset).map_err(|_| TimeErrorCode::InvalidParam)
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
        if name.is_null() { return Err(TimeErrorCode::InvalidParam); }
        let name_str = unsafe {
            std::ffi::CStr::from_ptr(name).to_str().map_err(|_| TimeErrorCode::InvalidParam)?
        };
        let offset = tz::get_offset_by_name(name_str).ok_or(TimeErrorCode::InvalidParam)?;
        config::set_timezone_offset(offset).map_err(|_| TimeErrorCode::InvalidParam)
    })();
    result_to_i32(result)
}

#[no_mangle]
pub extern "C" fn api_SetAutoSyncEnabled(enabled: bool) {
    config::set_auto_sync_enabled(enabled);
}

#[no_mangle]
pub extern "C" fn api_SetLastError(code: i32) {
    LAST_ERROR.store(code, Ordering::Release);
}

// ---------- Sync ----------
#[no_mangle]
pub extern "C" fn api_ForceResync() -> bool {
    crate::time::core::ntp::force_resync()
}

#[no_mangle]
pub extern "C" fn api_ForceResyncEx() -> i32 {
    if crate::time::core::ntp::force_resync() {
        TimeErrorCode::Success as i32
    } else {
        TimeErrorCode::NtpTimeout as i32
    }
}

#[no_mangle]
pub extern "C" fn api_IsDST(country: *const c_char) -> bool {
    let time = api_GetLocalTime();
    let country_str = unsafe {
        if country.is_null() { "" }
        else { std::ffi::CStr::from_ptr(country).to_str().unwrap_or("") }
    };
    crate::time::dst::is_dst(&time, country_str)
}

#[no_mangle]
pub extern "C" fn api_GetDSTOffset(country: *const c_char) -> i32 {
    let country_str = unsafe {
        if country.is_null() { "" }
        else { std::ffi::CStr::from_ptr(country).to_str().unwrap_or("") }
    };
    crate::time::dst::get_dst_offset(country_str)
}

#[no_mangle]
pub extern "C" fn api_SetAutoDST(enabled: bool) {
    crate::time::config::set_auto_dst_enabled(enabled);
}
// ---------- Error ----------
#[no_mangle]
pub extern "C" fn api_GetErrorString(code: i32) -> *const c_char {
    crate::error_string::get_error_string(code)
}