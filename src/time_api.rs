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

use crate::time::calc::{FullTime, FullTimeNs, CalendarType};
use crate::error::TimeErrorCode;
use crate::time::{calc, config, tz, dst};

// ---------- 版本信息 ----------
pub const VERSION_MAJOR: i32 = 0;
pub const VERSION_MINOR: i32 = 2;
pub const VERSION_PATCH: i32 = 7;

/// 获取 DLL 版本号
/// 返回值格式: 0xMMmmpp (主版本.次版本.补丁)
/// 例如 0x000207 表示 0.2.7
#[no_mangle]
pub extern "C" fn api_GetVersion() -> i32 {
    (VERSION_MAJOR << 16) | (VERSION_MINOR << 8) | VERSION_PATCH
}

/// 获取版本字符串（返回静态字符串，无需释放）
#[no_mangle]
pub extern "C" fn api_GetVersionString() -> *const c_char {
    // 返回静态字符串，避免内存泄漏
    concat!("0.2.7", "\0").as_ptr() as *const c_char
}

static LAST_ERROR: AtomicI32 = AtomicI32::new(0);

fn result_to_i32(result: Result<(), TimeErrorCode>) -> i32 {
    match result {
        Ok(()) => TimeErrorCode::Success as i32,
        Err(e) => e as i32,
    }
}

/// 设置历法类型（公历/儒略历）
/// @param type: 0=公历（默认），1=儒略历
#[no_mangle]
pub extern "C" fn api_SetCalendarType(cal_type: i32) {
    match cal_type {
        1 => crate::time::calc::set_calendar_type(CalendarType::Julian),
        _ => crate::time::calc::set_calendar_type(CalendarType::Gregorian),
    }
}

/// 获取当前历法类型
/// @return 0=公历，1=儒略历
#[no_mangle]
pub extern "C" fn api_GetCalendarType() -> i32 {
    crate::time::calc::get_calendar_type() as i32
}


// ---------- Getters ----------
#[no_mangle]
pub extern "C" fn api_GetLocalTime() -> FullTime {
    std::panic::catch_unwind(|| {
        let (sec, us) = calc::get_calibrated_local_time();
        calc::utc_to_fulltime(sec, us, config::get_timezone_offset())
    }).unwrap_or(FullTime {
        year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 0, ms: 0, us: 0 
    })
}

#[no_mangle]
pub extern "C" fn api_GetNetworkTime() -> FullTime {
    std::panic::catch_unwind(|| {
        match crate::time::core::ntp::get_cached_utc_time() {
            Some((sec, us)) => calc::utc_to_fulltime(sec, us, config::get_timezone_offset()),
            None => FullTime { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 0, ms: 0, us: 0 },
        }
    }).unwrap_or(FullTime {
        year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 0, ms: 0, us: 0 
    })
}

#[no_mangle]
pub extern "C" fn api_GetFormattedTime() -> *const c_char {
    std::panic::catch_unwind(|| {
        let ft = api_GetLocalTime();
        let s = format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
            ft.year, ft.month, ft.day, ft.hour, ft.minute, ft.second, ft.ms);
        CString::new(s).unwrap_or_default().into_raw()
    }).unwrap_or_else(|_| {
        CString::new("Error: Internal panic").unwrap_or_default().into_raw()
    })
}

#[no_mangle]
pub extern "C" fn api_FreeString(ptr: *mut c_char) {
    if ptr.is_null() { return; }
    unsafe { drop(CString::from_raw(ptr)); }
}

#[no_mangle]
pub extern "C" fn api_IsNTPSynced() -> bool {
    std::panic::catch_unwind(|| {
        crate::time::core::ntp::is_ntp_available()
    }).unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn api_GetTimezoneOffset() -> i32 {
    std::panic::catch_unwind(|| {
        config::get_timezone_offset()
    }).unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn api_GetLastError() -> i32 {
    LAST_ERROR.load(Ordering::Acquire)
}

#[no_mangle]
pub extern "C" fn api_GetLocalTimeNs() -> FullTimeNs {
    std::panic::catch_unwind(|| {
        let (secs, ns) = crate::time::core::local::get_system_time_ns();
        let base_offset = config::get_timezone_offset();
        let total_secs = secs + base_offset as i64;
        
        let ft = crate::time::calc::utc_to_fulltime_ns(total_secs, ns);
        // 修复：正确计算纳秒
        FullTimeNs {
            year: ft.year,
            month: ft.month,
            day: ft.day,
            hour: ft.hour,
            minute: ft.minute,
            second: ft.second,
            ns: ns,  // 直接使用原始纳秒值
        }
    }).unwrap_or(FullTimeNs {
        year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 0, ns: 0 
    })
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
    let result = std::panic::catch_unwind(|| {
        let country = if code.is_null() { None } else {
            Some(unsafe {
                std::ffi::CStr::from_ptr(code).to_str()
                    .map_err(|_| TimeErrorCode::InvalidParam)?
            })
        };
        
        // 首先获取 UTC 时间
        let base_offset = tz::offset_from_location(lon, lat, country)?;
        
        // 获取当前 UTC 时间，然后检查 DST
        let (utc_sec, utc_us) = calc::get_calibrated_local_time();
        
        // 先用基础偏移转换时间，判断 DST
        let local_prelim = calc::utc_to_fulltime(utc_sec, utc_us, base_offset);
        
        let final_offset = if let Some(c) = country {
            if dst::is_dst(&local_prelim, c) {
                base_offset + dst::get_dst_offset(c)
            } else {
                base_offset
            }
        } else {
            base_offset
        };
        
        config::set_timezone_offset(final_offset).map_err(|_| TimeErrorCode::InvalidParam)
    });
    
    match result {
        Ok(Ok(())) => TimeErrorCode::Success as i32,
        Ok(Err(e)) => e as i32,
        Err(_) => {
            LAST_ERROR.store(TimeErrorCode::InternalPanic as i32, Ordering::Release);
            TimeErrorCode::InternalPanic as i32
        }
    }
}

#[no_mangle]
pub extern "C" fn api_SetTimezoneOffset(sec: i32) -> i32 {
    let result = config::set_timezone_offset(sec).map_err(|_| TimeErrorCode::InvalidParam);
    result_to_i32(result)
}

#[no_mangle]
pub extern "C" fn api_SetTimezoneByName(name: *const c_char) -> i32 {
    let result = std::panic::catch_unwind(|| {
        if name.is_null() { return Err(TimeErrorCode::InvalidParam); }
        let name_str = unsafe {
            std::ffi::CStr::from_ptr(name).to_str().map_err(|_| TimeErrorCode::InvalidParam)?
        };
        let offset = tz::get_offset_by_name(name_str).ok_or(TimeErrorCode::InvalidParam)?;
        config::set_timezone_offset(offset).map_err(|_| TimeErrorCode::InvalidParam)
    });
    
    match result {
        Ok(Ok(())) => TimeErrorCode::Success as i32,
        Ok(Err(e)) => e as i32,
        Err(_) => TimeErrorCode::InternalPanic as i32
    }
}

#[no_mangle]
pub extern "C" fn api_SetAutoSyncEnabled(enabled: bool) {
    config::set_auto_sync_enabled(enabled);
}

#[no_mangle]
pub extern "C" fn api_SetLastError(code: i32) {
    LAST_ERROR.store(code, Ordering::Release);
}
/// 关闭 DLL，停止所有后台线程
/// 在卸载 DLL 前调用，确保干净退出
#[no_mangle]
pub extern "C" fn api_Shutdown() {
    crate::time::core::ntp::shutdown();
}

// ---------- Sync ----------
#[deprecated(since = "0.2.7", note = "Use api_ForceResyncEx() instead")]
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
    std::panic::catch_unwind(|| {
        let time = api_GetLocalTime();
        let country_str = unsafe {
            if country.is_null() { "" }
            else { std::ffi::CStr::from_ptr(country).to_str().unwrap_or("") }
        };
        crate::time::dst::is_dst(&time, country_str)
    }).unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn api_GetDSTOffset(country: *const c_char) -> i32 {
    std::panic::catch_unwind(|| {
        let country_str = unsafe {
            if country.is_null() { "" }
            else { std::ffi::CStr::from_ptr(country).to_str().unwrap_or("") }
        };
        crate::time::dst::get_dst_offset(country_str)
    }).unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn api_SetAutoDST(enabled: bool) {
    crate::time::config::set_auto_dst_enabled(enabled);
}

// ---------- 查询函数 ----------
/// 检查指定国家是否有 DST 规则
#[no_mangle]
pub extern "C" fn api_IsDSTAvailable(country: *const c_char) -> bool {
    std::panic::catch_unwind(|| {
        let country_str = unsafe {
            if country.is_null() { "" }
            else { std::ffi::CStr::from_ptr(country).to_str().unwrap_or("") }
        };
        crate::time::dst::get_rule(country_str).is_some()
    }).unwrap_or(false)
}

/// 检查 NTP 网络时间是否可用
#[no_mangle]
pub extern "C" fn api_IsNetworkTimeAvailable() -> bool {
    std::panic::catch_unwind(|| {
        crate::time::core::ntp::is_ntp_available()
    }).unwrap_or(false)
}

/// 检查时区偏移是否有效
#[no_mangle]
pub extern "C" fn api_IsValidTimezoneOffset(sec: i32) -> bool {
    sec >= -50400 && sec <= 50400
}

// ---------- Error ----------
#[no_mangle]
pub extern "C" fn api_GetErrorString(code: i32) -> *const c_char {
    crate::error_string::get_error_string(code)
}