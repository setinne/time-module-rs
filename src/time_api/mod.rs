

//! 对外 C 接口模块（拆分为多个子模块）
//!
//! 模块结构：
//! - types: 公共类型定义
//! - consts: 常量定义
//! - globals: 静态全局变量
//! - helpers: 辅助函数
//! - handle: 句柄模式（多实例支持）
//! - version: 版本信息
//! - error_handling: 错误处理
//! - calendar: 历法设置
//! - leap_second: 闰秒控制
//! - core_time: 核心时间获取
//! - formatting: 字符串格式化
//! - timezone_dst: 时区与 DST
//! - ntp: NTP 同步控制
//! - weekday: 星期函数
//! - unix_timestamp: Unix 时间戳
//! - date_utils: 日期工具
//! - logging: 日志回调
//! - shutdown: 关闭与清理
//! - async_ntp: 异步 NTP 同步

mod types;
mod consts;
mod globals;
mod helpers;
mod handle;
mod version;
mod error_handling;
mod calendar;
mod leap_second;
mod core_time;
mod formatting;
mod timezone_dst;
mod ntp;
mod weekday;
mod unix_timestamp;
mod date_utils;
mod logging;
mod shutdown;
mod async_ntp;

// 重新导出所有公开 API（可选，便于外部使用）
pub use types::*;
pub use consts::*;
pub use handle::*;
pub use version::*;
pub use error_handling::*;
pub use calendar::*;
pub use leap_second::*;
pub use core_time::*;
pub use formatting::*;
pub use timezone_dst::*;
pub use ntp::*;
pub use weekday::*;
pub use unix_timestamp::*;
pub use date_utils::*;
pub use logging::*;
pub use shutdown::*;
pub use async_ntp::*;