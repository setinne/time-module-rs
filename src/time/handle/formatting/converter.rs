// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 时间戳转换核心逻辑
use super::{FullTime, days::days_to_ymd};

/// 将 Unix 时间戳（秒+微秒）转换为日期时间结构
/// 支持负数时间戳（1970年之前的日期）
pub fn utc_to_fulltime(utc_sec: u64, utc_us: i32, tz_offset: i32) -> FullTime {
    let total_secs = utc_sec as i64 + tz_offset as i64;
    let us = utc_us.max(0);
    let ms = us / 1000;
    unix_timestamp_to_fulltime(total_secs, us, ms)
}

/// 将 Unix 时间戳（可能为负数）转换为 FullTime
fn unix_timestamp_to_fulltime(timestamp: i64, us: i32, ms: i32) -> FullTime {
    const SECS_PER_DAY: i64 = 86400;
    const SECS_PER_HOUR: i64 = 3600;
    const SECS_PER_MIN: i64 = 60;

    let days = if timestamp >= 0 {
        timestamp / SECS_PER_DAY
    } else {
        (timestamp - (SECS_PER_DAY - 1)) / SECS_PER_DAY
    };

    let mut remaining_secs = timestamp - days * SECS_PER_DAY;
    if remaining_secs < 0 {
        remaining_secs += SECS_PER_DAY;
    }

    let hour = (remaining_secs / SECS_PER_HOUR) as i32;
    let minute = ((remaining_secs % SECS_PER_HOUR) / SECS_PER_MIN) as i32;
    let second = (remaining_secs % SECS_PER_MIN) as i32;
    let (year, month, day) = days_to_ymd(days);

    FullTime { year, month, day, hour, minute, second, ms, us }
}