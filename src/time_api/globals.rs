//! 静态全局变量

use std::sync::atomic::{AtomicI32, AtomicI64};

use crate::time_api::types::{LogLevel, LogCallback, LeapSecondMode};

// 错误码
pub static LAST_ERROR: AtomicI32 = AtomicI32::new(0);

// 原有全局配置（在迁移到句柄模式期间仍使用，后续逐步废弃）
pub static GLOBAL_TIMEZONE_OFFSET: AtomicI32 = AtomicI32::new(0);
pub static GLOBAL_AUTO_DST_ENABLED: AtomicI32 = AtomicI32::new(1);
pub static GLOBAL_AUTO_SYNC_ENABLED: AtomicI32 = AtomicI32::new(1);
pub static GLOBAL_SYNC_INTERVAL: AtomicI64 = AtomicI64::new(3600);
pub static LEAP_MODE: AtomicI32 = AtomicI32::new(LeapSecondMode::Ignore as i32);
pub static LEAP_SMEAR_START: AtomicI64 = AtomicI64::new(0);
pub static LEAP_SMEAR_DURATION: AtomicI64 = AtomicI64::new(86400);
pub static LOG_LEVEL: AtomicI32 = AtomicI32::new(LogLevel::Info as i32);

// 新版日志回调（增强版，包含文件/行号/时间戳）
pub static mut LOG_CALLBACK: Option<LogCallback> = None;