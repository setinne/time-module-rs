//! 关闭与清理 API

use std::thread;
use std::time::Duration;
use std::sync::atomic::Ordering;
use crate::time_api::async_ntp::ASYNC_TASK_COUNT;

/// 关闭 DLL，停止所有后台线程。卸载前调用以确保干净退出。
#[no_mangle]
pub extern "C" fn api_Shutdown() {
    // 等待所有异步 NTP 任务完成（最多等待 5 秒）
    for _ in 0..50 {
        if ASYNC_TASK_COUNT.load(Ordering::SeqCst) == 0 {
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
    crate::time::core::ntp::shutdown();
}