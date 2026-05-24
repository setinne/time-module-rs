//! 闰秒控制 API

use std::sync::atomic::Ordering;
use crate::time_api::types::LeapSecondMode;
use crate::time_api::globals::{LEAP_MODE, LEAP_SMEAR_START, LEAP_SMEAR_DURATION};
use crate::time_api::handle::{TimeModuleHandle, get_default_handle};

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
pub fn get_smear_offset(handle: *mut TimeModuleHandle) -> f64 {
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