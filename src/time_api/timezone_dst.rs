//! 时区与 DST 设置/查询 API

use std::sync::atomic::Ordering;

use crate::error::TimeErrorCode;
use crate::time::{calc, tz, dst};
use crate::time::calc::utc_to_fulltime;
use crate::time_api::consts::{MIN_VALID_OFFSET, MAX_VALID_OFFSET};
use crate::time_api::globals::{
    GLOBAL_TIMEZONE_OFFSET, GLOBAL_AUTO_DST_ENABLED, LAST_ERROR
};
use crate::time_api::handle::{TimeModuleHandle, get_default_handle};
use crate::time_api::helpers::{result_to_i32, safe_catch};
use crate::time_api::core_time::api_GetLocalTime;

// ----------------------------------------------------------------------------
// 时区偏移获取/设置
// ----------------------------------------------------------------------------

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

/// 设置时区偏移（秒）- 指定模块
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
pub extern "C" fn api_SetTimezoneByName(name: *const std::os::raw::c_char) -> i32 {
    api_SetTimezoneByNameWithModule(get_default_handle(), name)
}

/// 通过名称设置时区（如 "UTC+8"）- 指定模块
#[no_mangle]
pub extern "C" fn api_SetTimezoneByNameWithModule(handle: *mut TimeModuleHandle, name: *const std::os::raw::c_char) -> i32 {
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
pub extern "C" fn api_SetTimezoneByLocation(lon: f64, lat: f64, code: *const std::os::raw::c_char) -> i32 {
    api_SetTimezoneByLocationEx(lon, lat, code, 1)
}

/// 通过经纬度设置时区，并指定是否应用 DST（apply_dst: 1=应用, 0=不应用）- 使用默认模块
#[no_mangle]
pub extern "C" fn api_SetTimezoneByLocationEx(lon: f64, lat: f64, code: *const std::os::raw::c_char, apply_dst: i32) -> i32 {
    api_SetTimezoneByLocationExWithModule(get_default_handle(), lon, lat, code, apply_dst)
}

/// 通过经纬度设置时区，并指定是否应用 DST（指定模块）
#[no_mangle]
pub extern "C" fn api_SetTimezoneByLocationExWithModule(
    handle: *mut TimeModuleHandle,
    lon: f64,
    lat: f64,
    code: *const std::os::raw::c_char,
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
                    let local_prelim = utc_to_fulltime(sec, us, base_offset);
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
pub extern "C" fn api_GetBaseOffsetByLocation(lon: f64, lat: f64, code: *const std::os::raw::c_char) -> i32 {
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

// ----------------------------------------------------------------------------
// DST 查询与控制
// ----------------------------------------------------------------------------

/// 获取指定国家的 DST 偏移（秒）
#[no_mangle]
pub extern "C" fn api_GetDSTOffset(country: *const std::os::raw::c_char) -> i32 {
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

// ----------------------------------------------------------------------------
// DST 布尔查询（Ex 系列返回 int 0/1）
// ----------------------------------------------------------------------------

#[deprecated(since = "0.2.9", note = "Use api_IsDSTEx instead")]
#[no_mangle]
pub extern "C" fn api_IsDST(country: *const std::os::raw::c_char) -> bool {
    api_IsDSTEx(country) != 0
}

#[no_mangle]
pub extern "C" fn api_IsDSTEx(country: *const std::os::raw::c_char) -> i32 {
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
pub extern "C" fn api_IsDSTAvailable(country: *const std::os::raw::c_char) -> bool {
    api_IsDSTAvailableEx(country) != 0
}

#[no_mangle]
pub extern "C" fn api_IsDSTAvailableEx(country: *const std::os::raw::c_char) -> i32 {
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