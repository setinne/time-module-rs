// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html

// 运行时配置：时区偏移和自动同步开关（原子变量）
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

static TIMEZONE_OFFSET: AtomicI32 = AtomicI32::new(28800);
static AUTO_SYNC_ENABLED: AtomicBool = AtomicBool::new(true);
static AUTO_DST_ENABLED: AtomicBool = AtomicBool::new(true);

pub fn set_timezone_offset(sec: i32) -> Result<(), ()> {
    if sec < -50400 || sec > 50400 {
        return Err(());
    }
    TIMEZONE_OFFSET.store(sec, Ordering::Release);
    Ok(())
}

pub fn get_timezone_offset() -> i32 {
    TIMEZONE_OFFSET.load(Ordering::Acquire)
}

pub fn is_auto_sync_enabled() -> bool {
    AUTO_SYNC_ENABLED.load(Ordering::Acquire)
}

pub fn set_auto_sync_enabled(enabled: bool) {
    AUTO_SYNC_ENABLED.store(enabled, Ordering::Release);
}

pub fn is_auto_dst_enabled() -> bool {
    AUTO_DST_ENABLED.load(Ordering::Acquire)
}

pub fn set_auto_dst_enabled(enabled: bool) {
    AUTO_DST_ENABLED.store(enabled, Ordering::Release);
}