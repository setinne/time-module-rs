// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html

//! 全局配置管理

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

// 全局配置）
static TIMEZONE_OFFSET: AtomicI32 = AtomicI32::new(28800);
#[allow(dead_code)]
static AUTO_DST_ENABLED: AtomicBool = AtomicBool::new(true);
static AUTO_SYNC_ENABLED: AtomicBool = AtomicBool::new(true);

/// 设置时区偏移
pub fn set_timezone_offset(sec: i32) -> Result<(), ()> {
    if sec < -43200 || sec > 50400 {
        return Err(());
    }
    TIMEZONE_OFFSET.store(sec, Ordering::Release);
    Ok(())
}

/// 获取时区偏移
pub fn get_timezone_offset() -> i32 {
    TIMEZONE_OFFSET.load(Ordering::Acquire)
}

/// 检查自动同步是否启用
pub fn is_auto_sync_enabled() -> bool {
    AUTO_SYNC_ENABLED.load(Ordering::Acquire)
}

/// 设置自动同步启用状态
pub fn set_auto_sync_enabled(enabled: bool) {
    AUTO_SYNC_ENABLED.store(enabled, Ordering::Release);
}

/// 检查自动 DST 是否启用
pub fn is_auto_dst_enabled() -> bool {
    AUTO_DST_ENABLED.load(Ordering::Acquire)
}

/// 设置自动 DST 启用状态
pub fn set_auto_dst_enabled(enabled: bool) {
    AUTO_DST_ENABLED.store(enabled, Ordering::Release);
}