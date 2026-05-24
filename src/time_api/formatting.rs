//! 字符串格式化 API

use std::ffi::CString;
use std::os::raw::c_char;
use crate::error::TimeErrorCode;
use crate::time_api::globals::LAST_ERROR;
use crate::time_api::core_time::api_GetLocalTime;

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
        LAST_ERROR.store(TimeErrorCode::InvalidParam as i32, std::sync::atomic::Ordering::Release);
        return -1;
    }
    if buf_size <= 0 {
        LAST_ERROR.store(TimeErrorCode::BufferTooSmall as i32, std::sync::atomic::Ordering::Release);
        return -1;
    }

    let ft = api_GetLocalTime();
    let s = format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
        ft.year, ft.month, ft.day, ft.hour, ft.minute, ft.second, ft.ms
    );

    let bytes = s.as_bytes();
    if bytes.len() + 1 > buf_size as usize {
        LAST_ERROR.store(TimeErrorCode::BufferTooSmall as i32, std::sync::atomic::Ordering::Release);
        return -1;
    }

    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, bytes.len());
        *buf.add(bytes.len()) = 0;
    }
    bytes.len() as i32
}