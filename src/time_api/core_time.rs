//! 核心时间获取 API（微秒/纳秒）

use crate::time::calc::{FullTime, FullTimeNs, utc_to_fulltime, utc_to_fulltime_ns};
use crate::time::calc::get_calibrated_local_time;
use crate::time_api::helpers::safe_catch;
use crate::time_api::globals::GLOBAL_TIMEZONE_OFFSET;
use crate::time_api::handle::{TimeModuleHandle, get_default_handle};
use crate::time_api::leap_second::get_smear_offset;

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
            let (sec, us) = get_calibrated_local_time();
            let tz_offset = if handle.is_null() {
                GLOBAL_TIMEZONE_OFFSET.load(std::sync::atomic::Ordering::Acquire)
            } else {
                let inner = unsafe { &*handle }.inner.lock().unwrap();
                inner.timezone_offset
            };
            utc_to_fulltime(sec, us, tz_offset)
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
                GLOBAL_TIMEZONE_OFFSET.load(std::sync::atomic::Ordering::Acquire)
            } else {
                let inner = unsafe { &*handle }.inner.lock().unwrap();
                inner.timezone_offset
            };
            let total_secs = secs + base_offset as i64;

            let ft = utc_to_fulltime_ns(total_secs, ns);
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
                    GLOBAL_TIMEZONE_OFFSET.load(std::sync::atomic::Ordering::Acquire)
                } else {
                    let inner = unsafe { &*handle }.inner.lock().unwrap();
                    inner.timezone_offset
                };
                utc_to_fulltime(sec, us, tz_offset)
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