//! 星期函数 API

use std::ffi::CString;
use std::os::raw::c_char;
use crate::error::TimeErrorCode;
use crate::time::calc::weekday as calc_weekday;
use crate::time::calc::weekday_iso;
use crate::time::calc::weekday_name;
use crate::time::calc::weekday_name_zh;
use crate::time_api::globals::LAST_ERROR;
use crate::time_api::helpers::is_valid_date;

/// 获取星期几（0=星期日, 1=星期一, ..., 6=星期六）
#[no_mangle]
pub extern "C" fn api_GetWeekday(year: i32, month: i32, day: i32) -> i32 {
    if !is_valid_date(year, month, day) {
        LAST_ERROR.store(TimeErrorCode::InvalidDate as i32, std::sync::atomic::Ordering::Release);
        return -1;
    }
    calc_weekday(year, month, day)
}

/// 获取星期几（1=星期一, 2=星期二, ..., 7=星期日）
#[no_mangle]
pub extern "C" fn api_GetWeekdayISO(year: i32, month: i32, day: i32) -> i32 {
    if !is_valid_date(year, month, day) {
        LAST_ERROR.store(TimeErrorCode::InvalidDate as i32, std::sync::atomic::Ordering::Release);
        return -1;
    }
    weekday_iso(year, month, day)
}

/// 获取英文星期名称（动态分配，需调用 api_FreeString 释放）
#[no_mangle]
pub extern "C" fn api_GetWeekdayName(year: i32, month: i32, day: i32) -> *const c_char {
    if !is_valid_date(year, month, day) {
        LAST_ERROR.store(TimeErrorCode::InvalidDate as i32, std::sync::atomic::Ordering::Release);
        return std::ptr::null();
    }
    let name = weekday_name(year, month, day);
    CString::new(name).unwrap_or_default().into_raw()
}

/// 获取中文星期名称（动态分配，需调用 api_FreeString 释放）
#[no_mangle]
pub extern "C" fn api_GetWeekdayNameZh(year: i32, month: i32, day: i32) -> *const c_char {
    if !is_valid_date(year, month, day) {
        LAST_ERROR.store(TimeErrorCode::InvalidDate as i32, std::sync::atomic::Ordering::Release);
        return std::ptr::null();
    }
    let name = weekday_name_zh(year, month, day);
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
        LAST_ERROR.store(TimeErrorCode::InvalidDate as i32, std::sync::atomic::Ordering::Release);
        return -1;
    }
    if buf.is_null() {
        LAST_ERROR.store(TimeErrorCode::InvalidParam as i32, std::sync::atomic::Ordering::Release);
        return -1;
    }
    if buf_size <= 0 {
        LAST_ERROR.store(TimeErrorCode::BufferTooSmall as i32, std::sync::atomic::Ordering::Release);
        return -1;
    }
    let name = weekday_name(year, month, day);
    let bytes = name.as_bytes();
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
        LAST_ERROR.store(TimeErrorCode::InvalidDate as i32, std::sync::atomic::Ordering::Release);
        return -1;
    }
    if buf.is_null() {
        LAST_ERROR.store(TimeErrorCode::InvalidParam as i32, std::sync::atomic::Ordering::Release);
        return -1;
    }
    if buf_size <= 0 {
        LAST_ERROR.store(TimeErrorCode::BufferTooSmall as i32, std::sync::atomic::Ordering::Release);
        return -1;
    }
    let name = weekday_name_zh(year, month, day);
    let bytes = name.as_bytes();
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