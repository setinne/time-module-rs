//! 版本信息 API

use std::ffi::CString;
use std::os::raw::c_char;
use crate::time_api::consts::{VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH};

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