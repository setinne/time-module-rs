//! NTP 同步控制 API

use crate::error::TimeErrorCode;
use crate::time::{calc, config};
use crate::time::core::ntp;
use crate::time_api::types::NTPStatus;
use crate::time_api::globals::{GLOBAL_AUTO_SYNC_ENABLED, GLOBAL_SYNC_INTERVAL};
use crate::time_api::handle::{TimeModuleHandle, get_default_handle};
use crate::time_api::helpers::safe_catch;

/// 强制同步 NTP（旧版，返回 bool）
#[deprecated(since = "0.2.7", note = "Use api_ForceResyncEx instead")]
#[no_mangle]
pub extern "C" fn api_ForceResync() -> bool {
    ntp::force_resync()
}

/// 强制同步 NTP（返回错误码）- 使用默认模块
#[no_mangle]
pub extern "C" fn api_ForceResyncEx() -> i32 {
    if ntp::force_resync() {
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
        GLOBAL_AUTO_SYNC_ENABLED.store(val, std::sync::atomic::Ordering::Release);
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
        GLOBAL_SYNC_INTERVAL.store(secs as i64, std::sync::atomic::Ordering::Release);
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

// ----------------------------------------------------------------------------
// NTP 状态查询（Ex 系列）
// ----------------------------------------------------------------------------

#[deprecated(since = "0.2.9", note = "Use api_IsNTPSyncedEx instead")]
#[no_mangle]
pub extern "C" fn api_IsNTPSynced() -> bool {
    api_IsNTPSyncedEx() != 0
}

#[no_mangle]
pub extern "C" fn api_IsNTPSyncedEx() -> i32 {
    safe_catch(
        || {
            if ntp::is_ntp_available() {
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
            if ntp::is_ntp_available() {
                1
            } else {
                0
            }
        },
        0,
    )
}