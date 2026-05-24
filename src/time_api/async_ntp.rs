//! 异步 NTP 同步 API (v0.2.17)

use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::error::TimeErrorCode;

/// 异步 NTP 同步回调类型
/// success: 1=成功, 0=失败
/// offset_ms: 同步后的时间偏移（毫秒），失败时为 0
/// user_data: 用户自定义数据指针
type NtpAsyncCallback = extern "C" fn(success: i32, offset_ms: i64, user_data: *mut std::ffi::c_void);

pub static ASYNC_TASK_COUNT: AtomicUsize = AtomicUsize::new(0);

/// 启动异步 NTP 同步（非阻塞）
/// 返回 0 表示成功启动异步任务，非 0 表示错误码
/// 同步完成后调用 callback
#[no_mangle]
pub extern "C" fn api_ForceResyncAsync(
    _handle: *mut crate::time_api::handle::TimeModuleHandle,
    callback: Option<NtpAsyncCallback>,
    user_data: *mut std::ffi::c_void,
) -> i32 {
    let cb = match callback {
        Some(cb) => cb,
        None => return TimeErrorCode::InvalidParam as i32,
    };

    let user_data_ptr = user_data as usize;

    let result = thread::Builder::new()
        .name("ntp-async".to_string())
        .spawn(move || {
            let success = crate::time::core::ntp::force_resync();
            let offset_ms = if success {
                if let Some((cached_sec, cached_us)) = crate::time::core::ntp::get_cached_utc_time() {
                    let system_sec = crate::time::core::local::get_system_time_ns().0;
                    (cached_sec as i64 - system_sec as i64) * 1000 + (cached_us as i64 / 1000)
                } else {
                    0
                }
            } else {
                0
            };
            let user_data = user_data_ptr as *mut std::ffi::c_void;
            cb(success as i32, offset_ms, user_data);
            ASYNC_TASK_COUNT.fetch_sub(1, Ordering::SeqCst);
        });
    
    match result {
        Ok(_handle) => {
            ASYNC_TASK_COUNT.fetch_add(1, Ordering::SeqCst);
            TimeErrorCode::Success as i32
        }
        Err(_) => TimeErrorCode::AsyncTaskFailed as i32,
    }
}