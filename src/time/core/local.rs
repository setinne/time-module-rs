// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 获取系统 UTC 时间及单调时钟

use std::sync::Once;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

/// 获取系统 UTC 时间（纳秒精度）
pub fn get_system_time_ns() -> (i64, i32) {
    let d = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    (d.as_secs() as i64, d.subsec_nanos() as i32)
}

/// 获取系统 UTC 时间（微秒精度，保持兼容）
pub fn get_system_time_us() -> (u64, i32) {
    let d = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    (d.as_secs(), d.subsec_micros() as i32)
}

/// 获取系统 UTC 时间（微秒精度，原名保持兼容）
pub fn get_system_time_utc() -> (u64, i32) {
    get_system_time_us()
}

static MONO_ONCE: Once = Once::new();
static mut MONO_START: Option<Instant> = None;

/// 获取单调时钟时间（秒，浮点）
pub fn monotonic_secs() -> f64 {
    MONO_ONCE.call_once(|| {
        unsafe {
            MONO_START = Some(Instant::now());
        }
    });
    let start = unsafe { MONO_START.as_ref().unwrap() };
    start.elapsed().as_secs_f64()
}