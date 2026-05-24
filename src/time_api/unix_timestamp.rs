//! Unix 时间戳 API

/// 获取当前 Unix 时间戳（秒）
#[no_mangle]
pub extern "C" fn api_GetUnixTimestamp() -> i64 {
    let (secs, _) = crate::time::core::local::get_system_time_ns();
    secs
}

/// 获取当前 Unix 时间戳（毫秒）
#[no_mangle]
pub extern "C" fn api_GetUnixTimestampMs() -> i64 {
    let (secs, ns) = crate::time::core::local::get_system_time_ns();
    secs * 1000 + (ns / 1_000_000) as i64
}

/// 获取当前 Unix 时间戳（微秒）
#[no_mangle]
pub extern "C" fn api_GetUnixTimestampUs() -> i64 {
    let (secs, ns) = crate::time::core::local::get_system_time_ns();
    secs * 1_000_000 + (ns / 1_000) as i64
}

/// 获取当前 Unix 时间戳（纳秒）
#[no_mangle]
pub extern "C" fn api_GetUnixTimestampNs() -> i64 {
    let (secs, ns) = crate::time::core::local::get_system_time_ns();
    secs * 1_000_000_000 + ns as i64
}