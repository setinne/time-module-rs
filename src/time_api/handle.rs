//! 句柄模式（多实例支持）

use std::sync::{Arc, Mutex, Once};
use crate::time_api::types::LogCallback;
use crate::time_api::globals::{
    GLOBAL_TIMEZONE_OFFSET, GLOBAL_AUTO_DST_ENABLED, GLOBAL_AUTO_SYNC_ENABLED,
    GLOBAL_SYNC_INTERVAL, LEAP_MODE, LEAP_SMEAR_START, LEAP_SMEAR_DURATION,
    LOG_LEVEL, LOG_CALLBACK,
};

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
            leap_mode: crate::time_api::types::LeapSecondMode::Ignore as i32,
            leap_smear_start: 0,
            leap_smear_duration: 86400,
            log_callback: None,
            log_level: crate::time_api::types::LogLevel::Info as i32,
        }
    }
}

/// 对外句柄（不透明指针）
#[repr(C)]
pub struct TimeModuleHandle {
    pub(super) inner: Arc<Mutex<TimeModuleInner>>,
}

// 确保句柄可以在线程间安全传递
unsafe impl Send for TimeModuleHandle {}
unsafe impl Sync for TimeModuleHandle {}

// 使用 Once + static mut 实现安全的延迟初始化
static DEFAULT_MODULE_INIT: Once = Once::new();
static mut DEFAULT_MODULE_PTR: *mut TimeModuleHandle = std::ptr::null_mut();

pub fn get_default_handle() -> *mut TimeModuleHandle {
    DEFAULT_MODULE_INIT.call_once(|| {
        let handle = api_CreateModule();
        // 从现有全局状态同步初始值
        if let Some(h) = unsafe { handle.as_mut() } {
            let mut inner = h.inner.lock().unwrap();
            inner.timezone_offset = GLOBAL_TIMEZONE_OFFSET.load(std::sync::atomic::Ordering::Acquire);
            inner.auto_dst_enabled = GLOBAL_AUTO_DST_ENABLED.load(std::sync::atomic::Ordering::Acquire) != 0;
            inner.auto_sync_enabled = GLOBAL_AUTO_SYNC_ENABLED.load(std::sync::atomic::Ordering::Acquire) != 0;
            inner.sync_interval_secs = GLOBAL_SYNC_INTERVAL.load(std::sync::atomic::Ordering::Acquire) as u64;
            inner.leap_mode = LEAP_MODE.load(std::sync::atomic::Ordering::Acquire);
            inner.leap_smear_start = LEAP_SMEAR_START.load(std::sync::atomic::Ordering::Acquire);
            inner.leap_smear_duration = LEAP_SMEAR_DURATION.load(std::sync::atomic::Ordering::Acquire);
            inner.log_level = LOG_LEVEL.load(std::sync::atomic::Ordering::Acquire);
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