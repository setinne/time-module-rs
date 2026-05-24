//! 日志回调 API

use std::ffi::CString;

use std::sync::atomic::Ordering;
use crate::time_api::types::{LogLevel, LogCallback};
use crate::time_api::globals::{LOG_CALLBACK, LOG_LEVEL};
use crate::time_api::handle::{TimeModuleHandle, get_default_handle};

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