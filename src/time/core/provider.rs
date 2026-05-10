// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html

// 组合系统时间和网络时间
use super::local::get_system_time_utc;
use super::ntp::get_cached_utc_time;

pub fn get_full_time_data() -> (u64, i32, Option<u64>) {
    let (sec, us) = get_system_time_utc();
    (sec, us, get_cached_utc_time().map(|(s,_)| s))
}