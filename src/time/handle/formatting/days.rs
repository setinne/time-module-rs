// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 日期计算（天数转年月日）

/// 判断是否为闰年
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// 获取指定年份的天数
pub fn days_in_year(year: i32) -> i64 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// 将一年中的第几天转换为月份和日期
pub fn day_of_year_to_month_day(year: i32, mut day_of_year: i32) -> (i32, i32) {
    let month_days = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1;
    for (i, &days_in_month) in month_days.iter().enumerate() {
        if day_of_year <= days_in_month {
            month = (i + 1) as i32;
            break;
        }
        day_of_year -= days_in_month;
    }
    (month, day_of_year)
}

/// 将从 Unix 纪元（1970-01-01）开始的天数转换为年月日
pub fn days_to_ymd(days: i64) -> (i32, i32, i32) {
    if days < 0 {
        let mut year = 1969;
        let mut remaining_days = days;

        while remaining_days < 0 {
            let days_in_year = if is_leap_year(year) { 366 } else { 365 };
            if remaining_days + days_in_year <= 0 {
                remaining_days += days_in_year;
                year -= 1;
            } else {
                break;
            }
        }

        let day_of_year = (remaining_days + days_in_year(year)) as i32 + 1;
        let (month, day) = day_of_year_to_month_day(year, day_of_year);
        return (year, month, day);
    }

    let mut year = 1970;
    let mut remaining_days = days;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }

    let day_of_year = remaining_days as i32 + 1;
    let (month, day) = day_of_year_to_month_day(year, day_of_year);
    (year, month, day)
}