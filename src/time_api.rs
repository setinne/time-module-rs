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
//! 13. 日志回调
//! 14. Context 句柄模式（v0.2.16）

use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::{Arc, Mutex, Once};
use std::panic::UnwindSafe;
use std::sync::atomic::{AtomicI32, AtomicI64, AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

use crate::error::TimeErrorCode;
use crate::time::{calc, config, tz, dst};
use crate::time::calc::{FullTime, FullTimeNs, CalendarType};


/// 有效时区偏移范围：UTC-12 到 UTC+14（-43200 秒 到 50400 秒）
/// 注：UTC+13 和 UTC+14 为政治选择时区（如基里巴斯、托克劳）
pub const MIN_VALID_OFFSET: i32 = -43200;   // UTC-12
pub const MAX_VALID_OFFSET: i32 = 50400;    // UTC+14

/// 常用时区偏移常量（秒）
pub const OFFSET_UTC_MINUS_12: i32 = -43200;
pub const OFFSET_UTC_MINUS_11: i32 = -39600;
pub const OFFSET_UTC_MINUS_10: i32 = -36000;
pub const OFFSET_UTC_MINUS_9:  i32 = -32400;
pub const OFFSET_UTC_MINUS_8:  i32 = -28800;
pub const OFFSET_UTC_MINUS_7:  i32 = -25200;
pub const OFFSET_UTC_MINUS_6:  i32 = -21600;
pub const OFFSET_UTC_MINUS_5:  i32 = -18000;
pub const OFFSET_UTC_MINUS_4:  i32 = -14400;
pub const OFFSET_UTC_MINUS_3:  i32 = -10800;
pub const OFFSET_UTC_MINUS_2:  i32 = -7200;
pub const OFFSET_UTC_MINUS_1:  i32 = -3600;
pub const OFFSET_UTC_0:        i32 = 0;
pub const OFFSET_UTC_PLUS_1:   i32 = 3600;
pub const OFFSET_UTC_PLUS_2:   i32 = 7200;
pub const OFFSET_UTC_PLUS_3:   i32 = 10800;
pub const OFFSET_UTC_PLUS_4:   i32 = 14400;
pub const OFFSET_UTC_PLUS_5:   i32 = 18000;
pub const OFFSET_UTC_PLUS_6:   i32 = 21600;
pub const OFFSET_UTC_PLUS_7:   i32 = 25200;
pub const OFFSET_UTC_PLUS_8:   i32 = 28800;
pub const OFFSET_UTC_PLUS_9:   i32 = 32400;
pub const OFFSET_UTC_PLUS_10:  i32 = 36000;
pub const OFFSET_UTC_PLUS_11:  i32 = 39600;
pub const OFFSET_UTC_PLUS_12:  i32 = 43200;
pub const OFFSET_UTC_PLUS_13:  i32 = 46800;  // 托克劳、萨摩亚（DST）
pub const OFFSET_UTC_PLUS_14:  i32 = 50400;  // 基里巴斯莱恩群岛

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

/// 闰秒处理模式
#[repr(C)]
pub enum LeapSecondMode {
    Ignore = 0,
    Smear = 1,
    Reject = 2,
}

/// 日志等级
#[repr(C)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
}

// ============================================================================
// 全局状态（为保持向后兼容，保留原有静态变量，但新代码优先使用句柄）
// ============================================================================

static LAST_ERROR: AtomicI32 = AtomicI32::new(0);

// 原有全局配置（在迁移到句柄模式期间仍使用，后续逐步废弃）
static GLOBAL_TIMEZONE_OFFSET: AtomicI32 = AtomicI32::new(0);
static GLOBAL_AUTO_DST_ENABLED: AtomicI32 = AtomicI32::new(1);
static GLOBAL_AUTO_SYNC_ENABLED: AtomicI32 = AtomicI32::new(1);
static GLOBAL_SYNC_INTERVAL: AtomicI64 = AtomicI64::new(3600);
static LEAP_MODE: AtomicI32 = AtomicI32::new(LeapSecondMode::Ignore as i32);
static LEAP_SMEAR_START: AtomicI64 = AtomicI64::new(0);
static LEAP_SMEAR_DURATION: AtomicI64 = AtomicI64::new(86400);
static LOG_LEVEL: AtomicI32 = AtomicI32::new(LogLevel::Info as i32);

// 新版日志回调（增强版，包含文件/行号/时间戳）
type LogCallback = extern "C" fn(level: i32, file: *const c_char, line: u32, timestamp: u64, msg: *const c_char);
static mut LOG_CALLBACK: Option<LogCallback> = None;

// ============================================================================
// 句柄模式内部结构（v0.2.16）
// ============================================================================

/// 内部模块状态（所有原本全局的状态都移到这里）
pub struct TimeModuleInner {
    pub timezone_offset: i32,
    pub auto_dst_enabled: bool,
    pub auto_sync_enabled: bool,
    pub sync_interval_secs: u64,
    pub leap_mode: i32,
    pub leap_smear_start: i64,
    pub leap_smear_duration: i64,
    pub log_callback: Option<LogCallback>,
    pub log_level: i32,
}

impl Default for TimeModuleInner {
    fn default() -> Self {
        Self {
            timezone_offset: 0,
            auto_dst_enabled: true,
            auto_sync_enabled: true,
            sync_interval_secs: 3600,
            leap_mode: LeapSecondMode::Ignore as i32,
            leap_smear_start: 0,
            leap_smear_duration: 86400,
            log_callback: None,
            log_level: LogLevel::Info as i32,
        }
    }
}

/// 对外句柄（不透明指针）
#[repr(C)]
pub struct TimeModuleHandle {
    inner: Arc<Mutex<TimeModuleInner>>,
}

// 确保句柄可以在线程间安全传递
unsafe impl Send for TimeModuleHandle {}
unsafe impl Sync for TimeModuleHandle {}

// 使用 Once + static mut 实现安全的延迟初始化
static DEFAULT_MODULE_INIT: Once = Once::new();
static mut DEFAULT_MODULE_PTR: *mut TimeModuleHandle = std::ptr::null_mut();

fn get_default_handle() -> *mut TimeModuleHandle {
    DEFAULT_MODULE_INIT.call_once(|| {
        let handle = api_CreateModule();
        // 从现有全局状态同步初始值
        if let Some(h) = unsafe { handle.as_mut() } {
            let mut inner = h.inner.lock().unwrap();
            inner.timezone_offset = GLOBAL_TIMEZONE_OFFSET.load(Ordering::Acquire);
            inner.auto_dst_enabled = GLOBAL_AUTO_DST_ENABLED.load(Ordering::Acquire) != 0;
            inner.auto_sync_enabled = GLOBAL_AUTO_SYNC_ENABLED.load(Ordering::Acquire) != 0;
            inner.sync_interval_secs = GLOBAL_SYNC_INTERVAL.load(Ordering::Acquire) as u64;
            inner.leap_mode = LEAP_MODE.load(Ordering::Acquire);
            inner.leap_smear_start = LEAP_SMEAR_START.load(Ordering::Acquire);
            inner.leap_smear_duration = LEAP_SMEAR_DURATION.load(Ordering::Acquire);
            inner.log_level = LOG_LEVEL.load(Ordering::Acquire);
            unsafe {
                inner.log_callback = LOG_CALLBACK;
            }
        }
        unsafe {
            DEFAULT_MODULE_PTR = handle;
        }
    });
    unsafe { DEFAULT_MODULE_PTR }
}

/// 创建新模块实例
#[no_mangle]
pub extern "C" fn api_CreateModule() -> *mut TimeModuleHandle {
    let inner = TimeModuleInner::default();
    let handle = Box::new(TimeModuleHandle {
        inner: Arc::new(Mutex::new(inner)),
    });
    Box::into_raw(handle)
}

/// 销毁模块实例
#[no_mangle]
pub extern "C" fn api_DestroyModule(handle: *mut TimeModuleHandle) {
    if !handle.is_null() {
        unsafe { drop(Box::from_raw(handle)); }
    }
}

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
pub const VERSION_PATCH: i32 = 19;

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
// 闰秒控制（句柄版本 + 旧版兼容）
// ============================================================================


fn set_leap_mode_on_handle(handle: *mut TimeModuleHandle, mode: i32) {
    if handle.is_null() {
        LEAP_MODE.store(mode, Ordering::Release);
        if mode == LeapSecondMode::Smear as i32 {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            LEAP_SMEAR_START.store(now, Ordering::Release);
        } else {
            LEAP_SMEAR_START.store(0, Ordering::Release);
        }
        return;
    }
    let mut inner = unsafe { &*handle }.inner.lock().unwrap();
    inner.leap_mode = mode;
    if mode == LeapSecondMode::Smear as i32 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        inner.leap_smear_start = now;
    } else {
        inner.leap_smear_start = 0;
    }
}

/// 设置闰秒处理模式（使用默认模块）
#[no_mangle]
pub extern "C" fn api_SetLeapSecondMode(mode: i32) {
    set_leap_mode_on_handle(get_default_handle(), mode);
}

/// 设置闰秒处理模式（指定模块）
#[no_mangle]
pub extern "C" fn api_SetLeapSecondModeWithModule(handle: *mut TimeModuleHandle, mode: i32) {
    set_leap_mode_on_handle(handle, mode);
}

/// 获取当前 smearing 偏移（秒）
fn get_smear_offset(handle: *mut TimeModuleHandle) -> f64 {
    let mode = if handle.is_null() {
        LEAP_MODE.load(Ordering::Acquire)
    } else {
        let inner = unsafe { &*handle }.inner.lock().unwrap();
        inner.leap_mode
    };
    if mode != LeapSecondMode::Smear as i32 {
        return 0.0;
    }
    let start = if handle.is_null() {
        LEAP_SMEAR_START.load(Ordering::Acquire)
    } else {
        let inner = unsafe { &*handle }.inner.lock().unwrap();
        inner.leap_smear_start
    };
    if start == 0 {
        return 0.0;
    }
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let elapsed = now - start;
    let duration = if handle.is_null() {
        LEAP_SMEAR_DURATION.load(Ordering::Acquire)
    } else {
        let inner = unsafe { &*handle }.inner.lock().unwrap();
        inner.leap_smear_duration
    };
    if elapsed >= duration {
        return 0.0;
    }
    1.0 * (elapsed as f64) / (duration as f64)
}

// ============================================================================
// 核心时间获取
// ============================================================================

/// 获取经校准的本地时间（微秒精度）- 使用默认模块
#[no_mangle]
pub extern "C" fn api_GetLocalTime() -> FullTime {
    api_GetLocalTimeWithModule(get_default_handle())
}

/// 获取经校准的本地时间（微秒精度）- 指定模块
#[no_mangle]
pub extern "C" fn api_GetLocalTimeWithModule(handle: *mut TimeModuleHandle) -> FullTime {
    let mut ft = safe_catch(
        || {
            let (sec, us) = calc::get_calibrated_local_time();
            let tz_offset = if handle.is_null() {
                GLOBAL_TIMEZONE_OFFSET.load(Ordering::Acquire)
            } else {
                let inner = unsafe { &*handle }.inner.lock().unwrap();
                inner.timezone_offset
            };
            calc::utc_to_fulltime(sec, us, tz_offset)
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
    );

    let smear = get_smear_offset(handle);
    if smear.abs() > 1e-6 {
        let total_us = ft.us as f64 + smear * 1_000_000.0;
        let extra_sec = (total_us / 1_000_000.0) as i32;
        ft.us = (total_us as i32).rem_euclid(1_000_000);
        ft.second += extra_sec;

        while ft.second >= 60 {
            ft.second -= 60;
            ft.minute += 1;
            if ft.minute >= 60 {
                ft.minute -= 60;
                ft.hour += 1;
                if ft.hour >= 24 {
                    ft.hour -= 24;
                }
            }
        }
    }

    ft
}

/// 获取经校准的本地时间（纳秒精度）- 使用默认模块
#[no_mangle]
pub extern "C" fn api_GetLocalTimeNs() -> FullTimeNs {
    api_GetLocalTimeNsWithModule(get_default_handle())
}

/// 获取经校准的本地时间（纳秒精度）- 指定模块
#[no_mangle]
pub extern "C" fn api_GetLocalTimeNsWithModule(handle: *mut TimeModuleHandle) -> FullTimeNs {
    let mut ft = safe_catch(
        || {
            let (secs, ns) = crate::time::core::local::get_system_time_ns();
            let base_offset = if handle.is_null() {
                GLOBAL_TIMEZONE_OFFSET.load(Ordering::Acquire)
            } else {
                let inner = unsafe { &*handle }.inner.lock().unwrap();
                inner.timezone_offset
            };
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
    );

    let smear = get_smear_offset(handle);
    if smear.abs() > 1e-6 {
        let total_ns = ft.ns as f64 + smear * 1_000_000_000.0;
        let extra_sec = (total_ns / 1_000_000_000.0) as i32;
        let new_ns = (total_ns as i64).rem_euclid(1_000_000_000);
        ft.ns = new_ns as i32;
        ft.second += extra_sec;

        while ft.second >= 60 {
            ft.second -= 60;
            ft.minute += 1;
            if ft.minute >= 60 {
                ft.minute -= 60;
                ft.hour += 1;
                if ft.hour >= 24 {
                    ft.hour -= 24;
                }
            }
        }
    }

    ft
}

/// 获取 NTP 网络时间（微秒精度，不可用时返回全 0）- 使用默认模块
#[no_mangle]
pub extern "C" fn api_GetNetworkTime() -> FullTime {
    api_GetNetworkTimeWithModule(get_default_handle())
}

/// 获取 NTP 网络时间（微秒精度，不可用时返回全 0）- 指定模块
#[no_mangle]
pub extern "C" fn api_GetNetworkTimeWithModule(handle: *mut TimeModuleHandle) -> FullTime {
    safe_catch(
        || match crate::time::core::ntp::get_cached_utc_time() {
            Some((sec, us)) => {
                let tz_offset = if handle.is_null() {
                    GLOBAL_TIMEZONE_OFFSET.load(Ordering::Acquire)
                } else {
                    let inner = unsafe { &*handle }.inner.lock().unwrap();
                    inner.timezone_offset
                };
                calc::utc_to_fulltime(sec, us, tz_offset)
            }
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

/// 获取格式化的时间字符串（动态分配，需调用 api_FreeString 释放）- 使用默认模块
#[no_mangle]
pub extern "C" fn api_GetFormattedTime() -> *const c_char {
    let ft = api_GetLocalTime();
    let s = format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
        ft.year, ft.month, ft.day, ft.hour, ft.minute, ft.second, ft.ms
    );
    CString::new(s).unwrap_or_default().into_raw()
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

// ============================================================================
// 时区与 DST 设置（句柄版本 + 旧版兼容）
// ============================================================================

/// 获取当前时区偏移（秒）- 使用默认模块
#[no_mangle]
pub extern "C" fn api_GetTimezoneOffset() -> i32 {
    GLOBAL_TIMEZONE_OFFSET.load(Ordering::Acquire)
}

/// 获取当前时区偏移（秒）- 指定模块
#[no_mangle]
pub extern "C" fn api_GetTimezoneOffsetWithModule(handle: *mut TimeModuleHandle) -> i32 {
    if handle.is_null() {
        return api_GetTimezoneOffset();
    }
    let inner = unsafe { &*handle }.inner.lock().unwrap();
    inner.timezone_offset
}

/// 设置时区偏移（秒）- 使用默认模块
#[no_mangle]
pub extern "C" fn api_SetTimezoneOffset(sec: i32) -> i32 {
    api_SetTimezoneOffsetWithModule(get_default_handle(), sec)
}

#[no_mangle]
pub extern "C" fn api_SetTimezoneOffsetWithModule(handle: *mut TimeModuleHandle, sec: i32) -> i32 {
    if sec < MIN_VALID_OFFSET || sec > MAX_VALID_OFFSET {
        LAST_ERROR.store(TimeErrorCode::TimezoneOffsetOutOfRange as i32, Ordering::Release);
        return TimeErrorCode::TimezoneOffsetOutOfRange as i32;
    }
    if handle.is_null() {
        GLOBAL_TIMEZONE_OFFSET.store(sec, Ordering::Release);
        return TimeErrorCode::Success as i32;
    }
    let mut inner = unsafe { &*handle }.inner.lock().unwrap();
    inner.timezone_offset = sec;
    TimeErrorCode::Success as i32
}

/// 通过名称设置时区（如 "UTC+8"）- 使用默认模块
#[no_mangle]
pub extern "C" fn api_SetTimezoneByName(name: *const c_char) -> i32 {
    api_SetTimezoneByNameWithModule(get_default_handle(), name)
}

/// 通过名称设置时区（如 "UTC+8"）- 指定模块
#[no_mangle]
pub extern "C" fn api_SetTimezoneByNameWithModule(handle: *mut TimeModuleHandle, name: *const c_char) -> i32 {
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
            let offset = tz::get_offset_by_name(name_str).ok_or(TimeErrorCode::TimezoneNameNotFound)?;
            if offset < MIN_VALID_OFFSET || offset > MAX_VALID_OFFSET {
                return Err(TimeErrorCode::TimezoneOffsetOutOfRange);
            }
            api_SetTimezoneOffsetWithModule(handle, offset);
            Ok(())
        },
        Err(TimeErrorCode::InternalPanic),
    );
    result_to_i32(result)
}

/// 通过经纬度设置时区（默认自动应用 DST）- 使用默认模块
#[no_mangle]
pub extern "C" fn api_SetTimezoneByLocation(lon: f64, lat: f64, code: *const c_char) -> i32 {
    api_SetTimezoneByLocationEx(lon, lat, code, 1)
}

/// 通过经纬度设置时区，并指定是否应用 DST（apply_dst: 1=应用, 0=不应用）- 使用默认模块
#[no_mangle]
pub extern "C" fn api_SetTimezoneByLocationEx(lon: f64, lat: f64, code: *const c_char, apply_dst: i32) -> i32 {
    api_SetTimezoneByLocationExWithModule(get_default_handle(), lon, lat, code, apply_dst)
}

/// 通过经纬度设置时区，并指定是否应用 DST（指定模块）
#[no_mangle]
pub extern "C" fn api_SetTimezoneByLocationExWithModule(
    handle: *mut TimeModuleHandle,
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

            api_SetTimezoneOffsetWithModule(handle, final_offset);
            Ok(())
        },
        Err(TimeErrorCode::InternalPanic),
    );

    result_to_i32(result)
}

/// 获取基础时区偏移（不含 DST），失败返回 -1 - 使用默认模块
#[no_mangle]
pub extern "C" fn api_GetBaseOffsetByLocation(lon: f64, lat: f64, code: *const c_char) -> i32 {
    let country = if code.is_null() {
        None
    } else {
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
// DST 查询与控制（句柄版本 + 旧版兼容）
// ============================================================================

/// 获取指定国家的 DST 偏移（秒）
#[no_mangle]
pub extern "C" fn api_GetDSTOffset(country: *const c_char) -> i32 {
    safe_catch(
        || {
            let country_str = unsafe {
                if country.is_null() {
                    return TimeErrorCode::DstRuleNotFound as i32;
                } else {
                    std::ffi::CStr::from_ptr(country).to_str().unwrap_or("")
                }
            };
            let offset = dst::get_dst_offset(country_str);
            if offset == 0 {
                // 注意：0 可能是有效偏移（无 DST），也可能是错误
                // 需要更好的判断，这里简化
            }
            offset
        },
        0,
    )
}

/// 启用/禁用自动 DST - 使用默认模块
#[no_mangle]
pub extern "C" fn api_SetAutoDST(enabled: bool) {
    api_SetAutoDSTWithModule(get_default_handle(), enabled);
}

/// 启用/禁用自动 DST - 指定模块
#[no_mangle]
pub extern "C" fn api_SetAutoDSTWithModule(handle: *mut TimeModuleHandle, enabled: bool) {
    let val = if enabled { 1 } else { 0 };
    if handle.is_null() {
        GLOBAL_AUTO_DST_ENABLED.store(val, Ordering::Release);
        return;
    }
    let mut inner = unsafe { &*handle }.inner.lock().unwrap();
    inner.auto_dst_enabled = enabled;
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

/// 检查时区偏移是否有效（有效范围：-43200 到 50400 秒，即 UTC-12 到 UTC+14）
#[no_mangle]
pub extern "C" fn api_IsValidTimezoneOffsetEx(sec: i32) -> i32 {
    if sec >= MIN_VALID_OFFSET && sec <= MAX_VALID_OFFSET { 1 } else { 0 }
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

/// 强制同步 NTP（返回错误码）- 使用默认模块
#[no_mangle]
pub extern "C" fn api_ForceResyncEx() -> i32 {
    if crate::time::core::ntp::force_resync() {
        TimeErrorCode::Success as i32
    } else {
        TimeErrorCode::NtpTimeout as i32
    }
}

/// 启用/禁用自动 NTP 同步 - 使用默认模块
#[no_mangle]
pub extern "C" fn api_SetAutoSyncEnabled(enabled: bool) {
    api_SetAutoSyncEnabledWithModule(get_default_handle(), enabled);
}

/// 启用/禁用自动 NTP 同步 - 指定模块
#[no_mangle]
pub extern "C" fn api_SetAutoSyncEnabledWithModule(handle: *mut TimeModuleHandle, enabled: bool) {
    let val = if enabled { 1 } else { 0 };
    if handle.is_null() {
        GLOBAL_AUTO_SYNC_ENABLED.store(val, Ordering::Release);
        return;
    }
    let mut inner = unsafe { &*handle }.inner.lock().unwrap();
    inner.auto_sync_enabled = enabled;
    // 注意：实际启用/禁用后台线程需要全局控制，这里仅记录状态
    config::set_auto_sync_enabled(enabled);
}

/// 设置 NTP 自动同步间隔（秒），最小 10 秒，默认 3600 - 使用默认模块
#[no_mangle]
pub extern "C" fn api_SetSyncInterval(seconds: u32) {
    api_SetSyncIntervalWithModule(get_default_handle(), seconds);
}

/// 设置 NTP 自动同步间隔（秒），最小 10 秒，默认 3600 - 指定模块
#[no_mangle]
pub extern "C" fn api_SetSyncIntervalWithModule(handle: *mut TimeModuleHandle, seconds: u32) {
    let secs = seconds.max(10) as u64;
    if handle.is_null() {
        GLOBAL_SYNC_INTERVAL.store(secs as i64, Ordering::Release);
        crate::time::defines::set_ntp_update_interval(secs);
        return;
    }
    let mut inner = unsafe { &*handle }.inner.lock().unwrap();
    inner.sync_interval_secs = secs;
    // 注意：实际同步间隔的修改需要通知后台线程，此处仅记录
    crate::time::defines::set_ntp_update_interval(secs);
}

/// 获取当前 NTP 同步间隔（秒）- 使用默认模块
#[no_mangle]
pub extern "C" fn api_GetSyncInterval() -> u32 {
    crate::time::defines::get_ntp_update_interval() as u32
}

/// 获取当前 NTP 同步间隔（秒）- 指定模块
#[no_mangle]
pub extern "C" fn api_GetSyncIntervalWithModule(handle: *mut TimeModuleHandle) -> u32 {
    if handle.is_null() {
        return api_GetSyncInterval();
    }
    let inner = unsafe { &*handle }.inner.lock().unwrap();
    inner.sync_interval_secs as u32
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
pub extern "C" fn api_GetWeekdayNameBuf(
    year: i32,
    month: i32,
    day: i32,
    buf: *mut u8,
    buf_size: i32,
) -> i32 {
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
pub extern "C" fn api_GetWeekdayNameZhBuf(
    year: i32,
    month: i32,
    day: i32,
    buf: *mut u8,
    buf_size: i32,
) -> i32 {
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
        total += if m == 2 && is_leap {
            29
        } else {
            days_in_month[(m - 1) as usize]
        };
    }
    total + day
}

// ============================================================================
// 日志回调（增强版，包含文件/行号/时间戳）
// ============================================================================

/// 注册日志回调函数（增强版）- 使用默认模块
#[no_mangle]
pub extern "C" fn api_SetLogCallback(callback: Option<LogCallback>) {
    api_SetLogCallbackWithModule(get_default_handle(), callback);
}

/// 注册日志回调函数（增强版）- 指定模块
#[no_mangle]
pub extern "C" fn api_SetLogCallbackWithModule(handle: *mut TimeModuleHandle, callback: Option<LogCallback>) {
    if handle.is_null() {
        unsafe { LOG_CALLBACK = callback; }
        return;
    }
    let mut inner = unsafe { &*handle }.inner.lock().unwrap();
    inner.log_callback = callback;
}

/// 设置日志最低输出级别（0=Debug,1=Info,2=Warning,3=Error）- 使用默认模块
#[no_mangle]
pub extern "C" fn api_SetLogLevel(level: i32) {
    api_SetLogLevelWithModule(get_default_handle(), level);
}

/// 设置日志最低输出级别（0=Debug,1=Info,2=Warning,3=Error）- 指定模块
#[no_mangle]
pub extern "C" fn api_SetLogLevelWithModule(handle: *mut TimeModuleHandle, level: i32) {
    let level = level.clamp(0, 3);
    if handle.is_null() {
        LOG_LEVEL.store(level, Ordering::Release);
        return;
    }
    let mut inner = unsafe { &*handle }.inner.lock().unwrap();
    inner.log_level = level;
}

/// 内部日志函数（供 DLL 内部使用）
#[allow(dead_code)]
pub(crate) fn log(level: LogLevel, file: &'static str, line: u32, msg: &str) {
    // 优先使用默认模块的回调
    let callback = unsafe { LOG_CALLBACK };
    let current_level = LOG_LEVEL.load(Ordering::Acquire);
    if (level as i32) < current_level {
        return;
    }
    if let Some(cb) = callback {
        let c_file = CString::new(file).unwrap_or_default();
        let c_msg = CString::new(msg).unwrap_or_default();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        cb(level as i32, c_file.as_ptr(), line, timestamp, c_msg.as_ptr());
    }
}

// ============================================================================
// 关闭与清理
// ============================================================================

/// 关闭 DLL，停止所有后台线程。卸载前调用以确保干净退出。
#[no_mangle]
pub extern "C" fn api_Shutdown() {
    // 等待所有异步 NTP 任务完成（最多等待 5 秒）
    for _ in 0..50 {
        if ASYNC_TASK_COUNT.load(Ordering::SeqCst) == 0 {
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
    crate::time::core::ntp::shutdown();
}

// ============================================================================
// 异步 NTP 同步（v0.2.17）
// ============================================================================


/// 异步 NTP 同步回调类型
/// success: 1=成功, 0=失败
/// offset_ms: 同步后的时间偏移（毫秒），失败时为 0
/// user_data: 用户自定义数据指针
type NtpAsyncCallback = extern "C" fn(success: i32, offset_ms: i64, user_data: *mut std::ffi::c_void);

static ASYNC_TASK_COUNT: AtomicUsize = AtomicUsize::new(0);

/// 启动异步 NTP 同步（非阻塞）
/// 返回 0 表示成功启动异步任务，非 0 表示错误码
/// 同步完成后调用 callback
#[no_mangle]
pub extern "C" fn api_ForceResyncAsync(
    _handle: *mut TimeModuleHandle,
    callback: Option<NtpAsyncCallback>,
    user_data: *mut std::ffi::c_void,
) -> i32 {
    let cb = match callback {
        Some(cb) => cb,
        None => return TimeErrorCode::InvalidParam as i32,
    };

    let user_data_ptr = user_data as usize;

    let result = thread::Builder::new()
        .name("ntp-async".to_string())
        .spawn(move || {
            let success = crate::time::core::ntp::force_resync();
            let offset_ms = if success {
                if let Some((cached_sec, cached_us)) = crate::time::core::ntp::get_cached_utc_time() {
                    let system_sec = crate::time::core::local::get_system_time_ns().0;
                    (cached_sec as i64 - system_sec as i64) * 1000 + (cached_us as i64 / 1000)
                } else {
                    0
                }
            } else {
                0
            };
            let user_data = user_data_ptr as *mut std::ffi::c_void;
            cb(success as i32, offset_ms, user_data);
            ASYNC_TASK_COUNT.fetch_sub(1, Ordering::SeqCst);
        });
    
    match result {
        Ok(_handle) => {
            ASYNC_TASK_COUNT.fetch_add(1, Ordering::SeqCst);
            TimeErrorCode::Success as i32
        }
        Err(_) => TimeErrorCode::AsyncTaskFailed as i32,
    }
}