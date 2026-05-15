// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 星期几计算，基于儒略日，支持 proleptic Gregorian 日历（包括公元前年份）

use super::jd::{gregorian_to_jd, julian_to_jd};
use super::get_calendar_type;
use super::CalendarType;

pub fn weekday(year: i32, month: i32, day: i32) -> i32 {
    let jd = match get_calendar_type() {
        CalendarType::Gregorian => gregorian_to_jd(year, month, day),
        CalendarType::Julian => julian_to_jd(year, month, day),
    };
    let offset = 1;
    ((jd + offset).rem_euclid(7)) as i32
}

pub fn weekday_iso(year: i32, month: i32, day: i32) -> i32 {
    let w = weekday(year, month, day);
    if w == 0 { 7 } else { w }
}

pub fn weekday_name(year: i32, month: i32, day: i32) -> &'static str {
    match weekday(year, month, day) {
        0 => "Sunday",
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        _ => unreachable!(),
    }
}

pub fn weekday_name_zh(year: i32, month: i32, day: i32) -> &'static str {
    match weekday(year, month, day) {
        0 => "星期日",
        1 => "星期一",
        2 => "星期二",
        3 => "星期三",
        4 => "星期四",
        5 => "星期五",
        6 => "星期六",
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weekday_known() {
        assert_eq!(weekday(1970, 1, 1), 4);
        assert_eq!(weekday(2000, 1, 1), 6);
        assert_eq!(weekday(2024, 5, 15), 3);
    }

    #[test]
    fn test_weekday_bc() {
        // 公元前年份，不应 panic，且返回值应在 0-6 之间
        let w = weekday(-4713, 1, 1);
        assert!((0..=6).contains(&w));
        let w2 = weekday(-1, 12, 31);
        assert!((0..=6).contains(&w2));
    }
}