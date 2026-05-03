// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html

// 将 UTC 时间戳加上时区偏移，转换为本地 FullTime 结构
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FullTime {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
    pub ms: i32,
    pub us: i32,
}

pub fn utc_to_fulltime(utc_sec: u64, utc_us: i32, tz_offset: i32) -> FullTime {
    let total_secs = utc_sec as i64 + tz_offset as i64;
    let total_secs = if total_secs < 0 { 0 } else { total_secs as u64 };
    const SECS_PER_DAY: u64 = 86400;
    const SECS_PER_HOUR: u64 = 3600;
    const SECS_PER_MIN: u64 = 60;

    let secs_in_day = total_secs % SECS_PER_DAY;
    let hour = (secs_in_day / SECS_PER_HOUR) as i32;
    let minute = ((secs_in_day % SECS_PER_HOUR) / SECS_PER_MIN) as i32;
    let second = (secs_in_day % SECS_PER_MIN) as i32;

    let mut days = (total_secs / SECS_PER_DAY) as i32;
    let mut year = 1970;
    loop {
        let leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
        let days_in_year = if leap { 366 } else { 365 };
        if days < days_in_year { break; }
        days -= days_in_year;
        year += 1;
    }
    let leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    let month_days = if leap {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut month = 0;
    let mut rem = days;
    while rem >= month_days[month] {
        rem -= month_days[month];
        month += 1;
    }
    FullTime {
        year,
        month: (month + 1) as i32,
        day: (rem + 1) as i32,
        hour, minute, second,
        ms: utc_us / 1000,
        us: utc_us,
    }
}