//! 类型定义

use std::ffi::c_char;

/// NTP 同步状态
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NTPStatus {
    NotStarted = 0,
    Syncing = 1,
    Synced = 2,
    OffsetLarge = 3,
}

/// 闰秒处理模式
#[repr(C)]
pub enum LeapSecondMode {
    Ignore = 0,
    Smear = 1,
    Reject = 2,
}

/// 日志等级
#[repr(C)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
}

/// 日志回调类型（增强版，包含文件/行号/时间戳）
pub type LogCallback = extern "C" fn(level: i32, file: *const c_char, line: u32, timestamp: u64, msg: *const c_char);