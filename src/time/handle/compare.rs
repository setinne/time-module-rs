// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html

// 比较系统时间与 NTP 时间是否在阈值内
use crate::time::defines::SYNC_THRESHOLD_SECONDS;
use crate::time::git::ntp::get_cached_utc_time;
use crate::time::git::local::get_system_time_utc;

fn abs_diff_u64(a: u64, b: u64) -> u64 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

pub fn check_time_accuracy() -> bool {
    let (sys_sec, _) = get_system_time_utc();
    match get_cached_utc_time() {
        Some((ntp_sec, _)) => abs_diff_u64(ntp_sec, sys_sec) <= SYNC_THRESHOLD_SECONDS,
        None => false,
    }
}