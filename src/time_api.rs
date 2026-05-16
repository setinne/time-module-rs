// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 对外 C 接口
//!
//! 模块结构：
//! 1. 导入与类型定义
//! 2. 版本信息
//! 3. 错误处理辅助
//! 4. 历法设置
//! 5. 核心时间获取（微秒/纳秒）
//! 6. 安全字符串格式化
//! 7. 时区/DST 设置与查询
//! 8. NTP 同步控制
//! 9. 工具函数（版本字符串、错误字符串、关闭等）
//! 10. 星期函数
//! 11. Unix 时间戳
//! 12. 日期工具

use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::atomic::{AtomicI32, Ordering};
use std::panic::UnwindSafe;
use std::sync::Mutex;

use crate::error::TimeErrorCode;
use crate::time::{calc, config, tz, dst};
use crate::time::calc::{FullTime, FullTimeNs, CalendarType};

// ============================================================================
// 类型定义
// ============================================================================

/// NTP 同步状态
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NTPStatus {
    NotStarted = 0,
    Syncing = 1,
    Synced = 2,
    OffsetLarge = 3,
}

// ============================================================================
// 全局状态
// ============================================================================

static LAST_ERROR: AtomicI32 = AtomicI32::new(0);

// ============================================================================
// 辅助函数
// ============================================================================

fn result_to_i32(result: Result<(), TimeErrorCode>) -> i32 {
    match result {
        Ok(()) => TimeErrorCode::Success as i32,
        Err(e) => e as i32,
    }
}

fn safe_catch<T, F>(f: F, default: T) -> T
where
    F: FnOnce() -> T + UnwindSafe,
{
    std::panic::catch_unwind(f).unwrap_or(default)
}

/// 判断是否为有效日期
fn is_valid_date(year: i32, month: i32, day: i32) -> bool {
    if month < 1 || month > 12 {
        return false;
    }
    let days_in_month = match month {
        2 => {
            let leap = api_IsLeapYear(year);
            if leap { 29 } else { 28 }
        }
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    day >= 1 && day <= days_in_month
}

// ============================================================================
// 版本信息
// ============================================================================

pub const VERSION_MAJOR: i32 = 0;
pub const VERSION_MINOR: i32 = 2;
pub const VERSION_PATCH: i32 = 13;

/// 获取 DLL 版本号（编码为 0xMMmmpp）
#[no_mangle]
pub extern "C" fn api_GetVersion() -> i32 {
    (VERSION_MAJOR << 16) | (VERSION_MINOR << 8) | VERSION_PATCH
}

/// 获取版本字符串（动态分配，需调用 api_FreeString 释放）
#[no_mangle]
pub extern "C" fn api_GetVersionString() -> *const c_char {
    let s = format!("{}.{}.{}", VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH);
    CString::new(s).unwrap_or_default().into_raw()
}

// ============================================================================
// 错误处理
// ============================================================================

#[no_mangle]
pub extern "C" fn api_GetLastError() -> i32 {
    LAST_ERROR.load(Ordering::Acquire)
}

#[no_mangle]
pub extern "C" fn api_SetLastError(code: i32) {
    LAST_ERROR.store(code, Ordering::Release);
}

/// 获取错误码描述（动态分配，需调用 api_FreeString 释放）
#[no_mangle]
pub extern "C" fn api_GetErrorString(code: i32) -> *const c_char {
    crate::error_string::get_error_string(code)
}

/// 释放由 `api_GetFormattedTime`、`api_GetVersionString`、`api_GetErrorString` 返回的字符串
#[no_mangle]
pub extern "C" fn api_FreeString(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe { drop(CString::from_raw(ptr)); }
}

// ============================================================================
// 历法设置
// ============================================================================

/// 设置历法类型（0=公历，1=儒略历）
#[no_mangle]
pub extern "C" fn api_SetCalendarType(cal_type: i32) {
    match cal_type {
        1 => calc::set_calendar_type(CalendarType::Julian),
        _ => calc::set_calendar_type(CalendarType::Gregorian),
    }
}

/// 获取当前历法类型（0=公历，1=儒略历）
#[no_mangle]
pub extern "C" fn api_GetCalendarType() -> i32 {
    calc::get_calendar_type() as i32
}

// ============================================================================
// 核心时间获取
// ============================================================================

/// 获取经校准的本地时间（微秒精度）
#[no_mangle]
pub extern "C" fn api_GetLocalTime() -> FullTime {
    safe_catch(
        || {
            let (sec, us) = calc::get_calibrated_local_time();
            calc::utc_to_fulltime(sec, us, config::get_timezone_offset())
        },
        FullTime {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
            ms: 0,
            us: 0,
        },
    )
}

/// 获取经校准的本地时间（纳秒精度）
#[no_mangle]
pub extern "C" fn api_GetLocalTimeNs() -> FullTimeNs {
    safe_catch(
        || {
            let (secs, ns) = crate::time::core::local::get_system_time_ns();
            let base_offset = config::get_timezone_offset();
            let total_secs = secs + base_offset as i64;

            let ft = calc::utc_to_fulltime_ns(total_secs, ns);
            FullTimeNs {
                year: ft.year,
                month: ft.month,
                day: ft.day,
                hour: ft.hour,
                minute: ft.minute,
                second: ft.second,
                ns,
            }
        },
        FullTimeNs {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
            ns: 0,
        },
    )
}

/// 获取 NTP 网络时间（微秒精度，不可用时返回全 0）
#[no_mangle]
pub extern "C" fn api_GetNetworkTime() -> FullTime {
    safe_catch(
        || match crate::time::core::ntp::get_cached_utc_time() {
            Some((sec, us)) => calc::utc_to_fulltime(sec, us, config::get_timezone_offset()),
            None => FullTime {
                year: 0,
                month: 0,
                day: 0,
                hour: 0,
                minute: 0,
                second: 0,
                ms: 0,
                us: 0,
            },
        },
        FullTime {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
            ms: 0,
            us: 0,
        },
    )
}

/// 获取格式化的时间字符串（动态分配，需调用 api_FreeString 释放）
#[no_mangle]
pub extern "C" fn api_GetFormattedTime() -> *const c_char {
    safe_catch(
        || {
            let ft = api_GetLocalTime();
            let s = format!(
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
                ft.year, ft.month, ft.day, ft.hour, ft.minute, ft.second, ft.ms
            );
            CString::new(s).unwrap_or_default().into_raw()
        },
        CString::new("Error: Internal panic").unwrap_or_default().into_raw(),
    )
}

/// 安全版本的格式化时间（调用者提供缓冲区）
/// 返回实际写入的字节数（不含 null），失败返回 -1
#[no_mangle]
pub extern "C" fn api_GetFormattedTimeBuf(buf: *mut u8, buf_size: i32) -> i32 {
    if buf.is_null() {
        LAST_ERROR.store(TimeErrorCode::InvalidParam as i32, Ordering::Release);
        return -1;  
    }
    if buf_size <= 0 {
        LAST_ERROR.store(TimeErrorCode::BufferTooSmall as i32, Ordering::Release);
        return -1;
    }

    let ft = api_GetLocalTime();
    let s = format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
        ft.year, ft.month, ft.day, ft.hour, ft.minute, ft.second, ft.ms
    );

    let bytes = s.as_bytes();
    if bytes.len() + 1 > buf_size as usize {
        LAST_ERROR.store(TimeErrorCode::BufferTooSmall as i32, Ordering::Release);
        return -1;
    }

    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, bytes.len());
        *buf.add(bytes.len()) = 0;
    }
    bytes.len() as i32
}

#[repr(C)]
pub enum LeapSecondMode {
    Ignore = 0,
    Smear = 1,
    Reject = 2,
}

static LEAP_MODE: AtomicI32 = AtomicI32::new(LeapSecondMode::Ignore as i32);

#[no_mangle]
pub extern "C" fn api_SetLeapSecondMode(mode: i32) {
    LEAP_MODE.store(mode, Ordering::Release);
}

// ============================================================================
// 时区与 DST 设置
// ============================================================================

/// 获取当前时区偏移（秒）
#[no_mangle]
pub extern "C" fn api_GetTimezoneOffset() -> i32 {
    safe_catch(|| config::get_timezone_offset(), 0)
}

/// 设置时区偏移（秒）
#[no_mangle]
pub extern "C" fn api_SetTimezoneOffset(sec: i32) -> i32 {
    let result = config::set_timezone_offset(sec).map_err(|_| TimeErrorCode::InvalidParam);
    result_to_i32(result)
}

/// 通过名称设置时区（如 "UTC+8"）
#[no_mangle]
pub extern "C" fn api_SetTimezoneByName(name: *const c_char) -> i32 {
    let result = safe_catch(
        || {
            if name.is_null() {
                return Err(TimeErrorCode::InvalidParam);
            }
            let name_str = unsafe {
                std::ffi::CStr::from_ptr(name)
                    .to_str()
                    .map_err(|_| TimeErrorCode::InvalidParam)?
            };
            let offset = tz::get_offset_by_name(name_str).ok_or(TimeErrorCode::InvalidParam)?;
            config::set_timezone_offset(offset).map_err(|_| TimeErrorCode::InvalidParam)
        },
        Err(TimeErrorCode::InternalPanic),
    );
    result_to_i32(result)
}

/// 通过经纬度设置时区（默认自动应用 DST）
#[no_mangle]
pub extern "C" fn api_SetTimezoneByLocation(lon: f64, lat: f64, code: *const c_char) -> i32 {
    api_SetTimezoneByLocationEx(lon, lat, code, 1)
}

/// 通过经纬度设置时区，并指定是否应用 DST（apply_dst: 1=应用, 0=不应用）
#[no_mangle]
pub extern "C" fn api_SetTimezoneByLocationEx(
    lon: f64,
    lat: f64,
    code: *const c_char,
    apply_dst: i32,
) -> i32 {
    let result = safe_catch(
        || {
            let country = if code.is_null() {
                None
            } else {
                Some(unsafe {
                    std::ffi::CStr::from_ptr(code)
                        .to_str()
                        .map_err(|_| TimeErrorCode::InvalidParam)?
                })
            };

            let base_offset = tz::offset_from_location(lon, lat, country)?;

            let final_offset = if apply_dst != 0 {
                if let Some(c) = country {
                    let (sec, us) = calc::get_calibrated_local_time();
                    let local_prelim = calc::utc_to_fulltime(sec, us, base_offset);
                    if dst::is_dst(&local_prelim, c) {
                        base_offset + dst::get_dst_offset(c)
                    } else {
                        base_offset
                    }
                } else {
                    base_offset
                }
            } else {
                base_offset
            };

            config::set_timezone_offset(final_offset).map_err(|_| TimeErrorCode::InvalidParam)
        },
        Err(TimeErrorCode::InternalPanic),
    );

    match result {
        Ok(()) => TimeErrorCode::Success as i32,
        Err(e) => {
            if e == TimeErrorCode::InternalPanic {
                LAST_ERROR.store(TimeErrorCode::InternalPanic as i32, Ordering::Release);
            }
            e as i32
        }
    }
}

/// 获取基础时区偏移（不含 DST），失败返回 -1
#[no_mangle]
pub extern "C" fn api_GetBaseOffsetByLocation(lon: f64, lat: f64, code: *const c_char) -> i32 {
    let country = if code.is_null() { None } else {
        unsafe { std::ffi::CStr::from_ptr(code).to_str().ok() }
    };
    match tz::offset_from_location(lon, lat, country) {
        Ok(offset) => offset,
        Err(e) => {
            LAST_ERROR.store(e as i32, Ordering::Release);
            -1
        }
    }
}
// ============================================================================
// DST 查询与控制
// ============================================================================

/// 获取指定国家的 DST 偏移（秒）
#[no_mangle]
pub extern "C" fn api_GetDSTOffset(country: *const c_char) -> i32 {
    safe_catch(
        || {
            let country_str = unsafe {
                if country.is_null() {
                    ""
                } else {
                    std::ffi::CStr::from_ptr(country).to_str().unwrap_or("")
                }
            };
            dst::get_dst_offset(country_str)
        },
        0,
    )
}

/// 启用/禁用自动 DST
#[no_mangle]
pub extern "C" fn api_SetAutoDST(enabled: bool) {
    config::set_auto_dst_enabled(enabled);
}

/// 设置 DST 后端（0=规则表，1=系统 API）
#[no_mangle]
pub extern "C" fn api_SetDSTBackend(backend: i32) {
    match backend {
        1 => dst::set_backend(dst::DSTBackend::SystemAPI),
        _ => dst::set_backend(dst::DSTBackend::RuleTable),
    }
}

/// 获取当前 DST 后端
#[no_mangle]
pub extern "C" fn api_GetDSTBackend() -> i32 {
    dst::get_backend() as i32
}

/// 获取系统完整时区偏移（含 DST）
#[no_mangle]
pub extern "C" fn api_GetSystemTimezoneOffset() -> i32 {
    dst::get_system_current_bias()
}

// ============================================================================
// DST 布尔查询（Ex 系列返回 int 0/1，跨语言安全）
// ============================================================================

#[deprecated(since = "0.2.9", note = "Use api_IsDSTEx instead")]
#[no_mangle]
pub extern "C" fn api_IsDST(country: *const c_char) -> bool {
    api_IsDSTEx(country) != 0
}

#[no_mangle]
pub extern "C" fn api_IsDSTEx(country: *const c_char) -> i32 {
    safe_catch(
        || {
            let time = api_GetLocalTime();
            let country_str = unsafe {
                if country.is_null() {
                    ""
                } else {
                    std::ffi::CStr::from_ptr(country).to_str().unwrap_or("")
                }
            };
            if dst::is_dst(&time, country_str) {
                1
            } else {
                0
            }
        },
        0,
    )
}

#[deprecated(since = "0.2.9", note = "Use api_IsSystemDSTEx instead")]
#[no_mangle]
pub extern "C" fn api_IsSystemDST() -> bool {
    api_IsSystemDSTEx() != 0
}

#[no_mangle]
pub extern "C" fn api_IsSystemDSTEx() -> i32 {
    if dst::is_system_dst() { 1 } else { 0 }
}

#[deprecated(since = "0.2.9", note = "Use api_IsDSTAvailableEx instead")]
#[no_mangle]
pub extern "C" fn api_IsDSTAvailable(country: *const c_char) -> bool {
    api_IsDSTAvailableEx(country) != 0
}

#[no_mangle]
pub extern "C" fn api_IsDSTAvailableEx(country: *const c_char) -> i32 {
    safe_catch(
        || {
            let country_str = unsafe {
                if country.is_null() {
                    ""
                } else {
                    std::ffi::CStr::from_ptr(country).to_str().unwrap_or("")
                }
            };
            if dst::get_rule(country_str).is_some() {
                1
            } else {
                0
            }
        },
        0,
    )
}

#[deprecated(since = "0.2.9", note = "Use api_IsValidTimezoneOffsetEx instead")]
#[no_mangle]
pub extern "C" fn api_IsValidTimezoneOffset(sec: i32) -> bool {
    api_IsValidTimezoneOffsetEx(sec) != 0
}

#[no_mangle]
pub extern "C" fn api_IsValidTimezoneOffsetEx(sec: i32) -> i32 {
    if sec >= -50400 && sec <= 50400 { 1 } else { 0 }
}

// ============================================================================
// NTP 同步控制
// ============================================================================

/// 强制同步 NTP（旧版，返回 bool）
#[deprecated(since = "0.2.7", note = "Use api_ForceResyncEx instead")]
#[no_mangle]
pub extern "C" fn api_ForceResync() -> bool {
    crate::time::core::ntp::force_resync()
}

/// 强制同步 NTP（返回错误码）
#[no_mangle]
pub extern "C" fn api_ForceResyncEx() -> i32 {
    if crate::time::core::ntp::force_resync() {
        TimeErrorCode::Success as i32
    } else {
        TimeErrorCode::NtpTimeout as i32
    }
}

/// 启用/禁用自动 NTP 同步
#[no_mangle]
pub extern "C" fn api_SetAutoSyncEnabled(enabled: bool) {
    config::set_auto_sync_enabled(enabled);
}

/// 设置 NTP 自动同步间隔（秒），最小 10 秒，默认 3600
#[no_mangle]
pub extern "C" fn api_SetSyncInterval(seconds: u32) {
    crate::time::defines::set_ntp_update_interval(seconds as u64);
}

/// 获取当前 NTP 同步间隔（秒）
#[no_mangle]
pub extern "C" fn api_GetSyncInterval() -> u32 {
    crate::time::defines::get_ntp_update_interval() as u32
}

/// 获取 NTP 同步状态（0=未启动, 1=同步中, 2=已同步, 3=偏移过大）
#[no_mangle]
pub extern "C" fn api_GetNTPStatus() -> i32 {
    use crate::time::core::ntp;
    if !ntp::is_started() {
        return NTPStatus::NotStarted as i32;
    }
    if !ntp::is_ntp_available() {
        return NTPStatus::Syncing as i32;
    }
    if calc::is_time_synced() {
        NTPStatus::Synced as i32
    } else {
        NTPStatus::OffsetLarge as i32
    }
}

// ============================================================================
// NTP 状态查询（Ex 系列）
// ============================================================================

#[deprecated(since = "0.2.9", note = "Use api_IsNTPSyncedEx instead")]
#[no_mangle]
pub extern "C" fn api_IsNTPSynced() -> bool {
    api_IsNTPSyncedEx() != 0
}

#[no_mangle]
pub extern "C" fn api_IsNTPSyncedEx() -> i32 {
    safe_catch(
        || {
            if crate::time::core::ntp::is_ntp_available() {
                1
            } else {
                0
            }
        },
        0,
    )
}

#[deprecated(since = "0.2.9", note = "Use api_IsNetworkTimeAvailableEx instead")]
#[no_mangle]
pub extern "C" fn api_IsNetworkTimeAvailable() -> bool {
    api_IsNetworkTimeAvailableEx() != 0
}

#[no_mangle]
pub extern "C" fn api_IsNetworkTimeAvailableEx() -> i32 {
    safe_catch(
        || {
            if crate::time::core::ntp::is_ntp_available() {
                1
            } else {
                0
            }
        },
        0,
    )
}

// ============================================================================
// 星期函数
// ============================================================================

/// 获取星期几（0=星期日, 1=星期一, ..., 6=星期六）
#[no_mangle]
pub extern "C" fn api_GetWeekday(year: i32, month: i32, day: i32) -> i32 {
    if !is_valid_date(year, month, day) {
        LAST_ERROR.store(TimeErrorCode::InvalidDate as i32, Ordering::Release);
        return -1;
    }
    crate::time::calc::weekday(year, month, day)
}

/// 获取星期几（1=星期一, 2=星期二, ..., 7=星期日）
#[no_mangle]
pub extern "C" fn api_GetWeekdayISO(year: i32, month: i32, day: i32) -> i32 {
    if !is_valid_date(year, month, day) {
        LAST_ERROR.store(TimeErrorCode::InvalidDate as i32, Ordering::Release);
        return -1;
    }
    crate::time::calc::weekday_iso(year, month, day)
}

/// 获取英文星期名称（动态分配，需调用 api_FreeString 释放）
#[no_mangle]
pub extern "C" fn api_GetWeekdayName(year: i32, month: i32, day: i32) -> *const c_char {
    if !is_valid_date(year, month, day) {
        LAST_ERROR.store(TimeErrorCode::InvalidDate as i32, Ordering::Release);
        return std::ptr::null();
    }
    let name = crate::time::calc::weekday_name(year, month, day);
    CString::new(name).unwrap_or_default().into_raw()
}

/// 获取中文星期名称（动态分配，需调用 api_FreeString 释放）
#[no_mangle]
pub extern "C" fn api_GetWeekdayNameZh(year: i32, month: i32, day: i32) -> *const c_char {
    if !is_valid_date(year, month, day) {
        LAST_ERROR.store(TimeErrorCode::InvalidDate as i32, Ordering::Release);
        return std::ptr::null();
    }
    let name = crate::time::calc::weekday_name_zh(year, month, day);
    CString::new(name).unwrap_or_default().into_raw()
}

/// 安全版本：获取英文星期名称到调用者缓冲区
/// 返回实际写入字节数（不含null），失败返回 -1
#[no_mangle]
pub extern "C" fn api_GetWeekdayNameBuf(year: i32, month: i32, day: i32, buf: *mut u8, buf_size: i32) -> i32 {
    if !is_valid_date(year, month, day) {
        LAST_ERROR.store(TimeErrorCode::InvalidDate as i32, Ordering::Release);
        return -1;
    }
    if buf.is_null() {
        LAST_ERROR.store(TimeErrorCode::InvalidParam as i32, Ordering::Release);
        return -1;
    }
    if buf_size <= 0 {
        LAST_ERROR.store(TimeErrorCode::BufferTooSmall as i32, Ordering::Release);
        return -1;
    }
    let name = crate::time::calc::weekday_name(year, month, day);
    let bytes = name.as_bytes();
    if bytes.len() + 1 > buf_size as usize {
        LAST_ERROR.store(TimeErrorCode::BufferTooSmall as i32, Ordering::Release);
        return -1;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, bytes.len());
        *buf.add(bytes.len()) = 0;
    }
    bytes.len() as i32
}

/// 安全版本：获取中文星期名称到调用者缓冲区
/// 返回实际写入字节数（不含null），失败返回 -1
#[no_mangle]
pub extern "C" fn api_GetWeekdayNameZhBuf(year: i32, month: i32, day: i32, buf: *mut u8, buf_size: i32) -> i32 {
    if !is_valid_date(year, month, day) {
        LAST_ERROR.store(TimeErrorCode::InvalidDate as i32, Ordering::Release);
        return -1;
    }
    if buf.is_null() {
        LAST_ERROR.store(TimeErrorCode::InvalidParam as i32, Ordering::Release);
        return -1;
    }
    if buf_size <= 0 {
        LAST_ERROR.store(TimeErrorCode::BufferTooSmall as i32, Ordering::Release);
        return -1;
    }
    let name = crate::time::calc::weekday_name_zh(year, month, day);
    let bytes = name.as_bytes();
    if bytes.len() + 1 > buf_size as usize {
        LAST_ERROR.store(TimeErrorCode::BufferTooSmall as i32, Ordering::Release);
        return -1;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, bytes.len());
        *buf.add(bytes.len()) = 0;
    }
    bytes.len() as i32
}

// ============================================================================
// Unix 时间戳
// ============================================================================

/// 获取当前 Unix 时间戳（秒）
#[no_mangle]
pub extern "C" fn api_GetUnixTimestamp() -> i64 {
    let (secs, _) = crate::time::core::local::get_system_time_ns();
    secs
}

/// 获取当前 Unix 时间戳（毫秒）
#[no_mangle]
pub extern "C" fn api_GetUnixTimestampMs() -> i64 {
    let (secs, ns) = crate::time::core::local::get_system_time_ns();
    secs * 1000 + (ns / 1_000_000) as i64
}

/// 获取当前 Unix 时间戳（微秒）
#[no_mangle]
pub extern "C" fn api_GetUnixTimestampUs() -> i64 {
    let (secs, ns) = crate::time::core::local::get_system_time_ns();
    secs * 1_000_000 + (ns / 1_000) as i64
}

/// 获取当前 Unix 时间戳（纳秒）
#[no_mangle]
pub extern "C" fn api_GetUnixTimestampNs() -> i64 {
    let (secs, ns) = crate::time::core::local::get_system_time_ns();
    secs * 1_000_000_000 + ns as i64
}

// ============================================================================
// 日期工具
// ============================================================================

/// 判断是否为闰年
#[no_mangle]
pub extern "C" fn api_IsLeapYear(year: i32) -> bool {
    (year.rem_euclid(4) == 0 && year.rem_euclid(100) != 0) || year.rem_euclid(400) == 0
}

/// 判断是否为闰年（Ex 版本，返回 1/0）
#[no_mangle]
pub extern "C" fn api_IsLeapYearEx(year: i32) -> i32 {
    if api_IsLeapYear(year) { 1 } else { 0 }
}

/// 获取指定日期在一年中的第几天（1-366），失败返回 -1
#[no_mangle]
pub extern "C" fn api_DayOfYear(year: i32, month: i32, day: i32) -> i32 {
    if !is_valid_date(year, month, day) {
        LAST_ERROR.store(TimeErrorCode::InvalidDate as i32, Ordering::Release);
        return -1;
    }
    let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let is_leap = api_IsLeapYear(year);
    let mut total = 0;
    for m in 1..month {
        total += if m == 2 && is_leap { 29 } else { days_in_month[(m - 1) as usize] };
    }
    total + day
}

// ============================================================================
// 日志回调
// ============================================================================

// 日志等级
#[repr(C)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
}

// 日志回调类型
type LogCallback = extern "C" fn(level: i32, msg: *const c_char);

static mut LOG_CALLBACK: Option<LogCallback> = None;
static LOG_MUTEX: Mutex<()> = Mutex::new(());

#[no_mangle]
pub extern "C" fn api_SetLogCallback(callback: Option<LogCallback>) {
    let _lock = LOG_MUTEX.lock();
    unsafe { LOG_CALLBACK = callback; }
}

// 内部函数，在需要日志的地方调用
fn log(level: LogLevel, msg: &str) {
    let _lock = LOG_MUTEX.lock();
    unsafe {
        if let Some(cb) = LOG_CALLBACK {
            let c_msg = CString::new(msg).unwrap_or_default();
            cb(level as i32, c_msg.as_ptr());
        }
    }
}

// ============================================================================
// 关闭与清理
// ============================================================================

/// 关闭 DLL，停止所有后台线程。卸载前调用以确保干净退出。
#[no_mangle]
pub extern "C" fn api_Shutdown() {
    crate::time::core::ntp::shutdown();
}