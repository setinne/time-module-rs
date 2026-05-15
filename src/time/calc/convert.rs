// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 时间戳转换核心逻辑

use super::jd::{julian_day_to_unix_secs, unix_secs_to_julian_day, jd_to_gregorian};
use super::calendar::{CalendarType, gregorian_to_julian};
use super::FullTime;

static mut CALENDAR_TYPE: CalendarType = CalendarType::Gregorian;

pub fn set_calendar_type(cal_type: CalendarType) {
    unsafe { CALENDAR_TYPE = cal_type; }
}

pub fn get_calendar_type() -> CalendarType {
    unsafe { CALENDAR_TYPE }
}

/// Unix 时间戳（秒+纳秒）→ 公历/儒略历 FullTime（微秒精度）
pub fn utc_to_fulltime_ns(secs: i64, ns: i32) -> FullTime {
    let jd = unix_secs_to_julian_day(secs);
    let (year, month, day) = jd_to_gregorian(jd);

    let (year, month, day) = match get_calendar_type() {
        CalendarType::Julian => gregorian_to_julian(year, month, day),
        CalendarType::Gregorian => (year, month, day),
    };

    let day_start = julian_day_to_unix_secs(jd);
    let mut remaining = secs - day_start;
    remaining = remaining.rem_euclid(86400); // [0, 86400)

    let hour = (remaining / 3600) as i32;
    let minute = ((remaining % 3600) / 60) as i32;
    let second = (remaining % 60) as i32;

    let ms = ns / 1_000_000;
    let us = (ns % 1_000_000) / 1_000;

    FullTime {
        year,
        month,
        day,
        hour,
        minute,
        second,
        ms,
        us,
    }
}

/// Unix 时间戳（秒+微秒，带时区偏移）→ FullTime（保持兼容）
pub fn utc_to_fulltime(secs: u64, us: i32, tz_offset: i32) -> FullTime {
    let total_secs = secs as i64 + tz_offset as i64;
    let ns = us * 1000;
    utc_to_fulltime_ns(total_secs, ns)
}