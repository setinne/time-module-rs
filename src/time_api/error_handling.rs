//! 错误处理 API

use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::atomic::Ordering;
use crate::time_api::globals::LAST_ERROR;

/// 获取最后发生的错误码
#[no_mangle]
pub extern "C" fn api_GetLastError() -> i32 {
    LAST_ERROR.load(Ordering::Acquire)
}

/// 设置错误码
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