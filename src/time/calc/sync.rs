// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 时间同步与校准

use crate::time::defines::SYNC_THRESHOLD_SECONDS;
use crate::time::core::ntp::get_cached_utc_time;
use crate::time::core::local::get_system_time_utc;

fn abs_diff_u64(a: u64, b: u64) -> u64 {
    if a > b { a - b } else { b - a }
}

/// 检查系统时间是否与 NTP 时间同步
pub fn is_time_synced() -> bool {
    let (sys_sec, _) = get_system_time_utc();
    match get_cached_utc_time() {
        Some((ntp_sec, _)) => abs_diff_u64(ntp_sec, sys_sec) <= SYNC_THRESHOLD_SECONDS,
        None => false,
    }
}

/// 获取经校准的本地时间
pub fn get_calibrated_local_time() -> (u64, i32) {
    let (sys_sec, sys_us) = get_system_time_utc();
    if let Some((ntp_sec, ntp_us)) = get_cached_utc_time() {
        (ntp_sec, ntp_us)
    } else {
        (sys_sec, sys_us)
    }
}

pub fn check_time_accuracy() -> bool {
    is_time_synced()
}