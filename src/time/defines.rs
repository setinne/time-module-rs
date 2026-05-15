// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html

// 常量定义
use std::sync::atomic::{AtomicU64, Ordering};

pub const SYNC_THRESHOLD_SECONDS: u64 = 60;
pub const DEFAULT_NTP_UPDATE_INTERVAL_SEC: u64 = 3600;

static NTP_UPDATE_INTERVAL_SEC: AtomicU64 = AtomicU64::new(DEFAULT_NTP_UPDATE_INTERVAL_SEC);

pub fn get_ntp_update_interval() -> u64 {
    let val = NTP_UPDATE_INTERVAL_SEC.load(Ordering::Acquire);
    if val < 10 { DEFAULT_NTP_UPDATE_INTERVAL_SEC } else { val }
}

pub fn set_ntp_update_interval(sec: u64) {
    let val = if sec < 10 { DEFAULT_NTP_UPDATE_INTERVAL_SEC } else { sec };
    NTP_UPDATE_INTERVAL_SEC.store(val, Ordering::Release);
}