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
/// 支持负数天数（1970 年之前的日期）
pub fn days_to_ymd(days: i64) -> (i32, i32, i32) {
    if days >= 0 {
        // 正数天数：从 1970 年开始向后推算
        let mut year = 1970;
        let mut remaining = days;
        
        loop {
            let days_in_year = if is_leap_year(year) { 366 } else { 365 };
            if remaining < days_in_year {
                break;
            }
            remaining -= days_in_year;
            year += 1;
        }
        
        let day_of_year = remaining as i32 + 1;
        let (month, day) = day_of_year_to_month_day(year, day_of_year);
        (year, month, day)
    } else {
        // 负数天数：从 1969 年开始向前推算
        let mut year = 1969;
        let mut remaining = days;  // 负值
        
        loop {
            let days_in_year = if is_leap_year(year) { 366 } else { 365 };
            // 当 remaining + days_in_year < 0 时，说明还要往前
            // 如果等于 0，说明正好是这一年的第一天
            if remaining + days_in_year < 0 {
                remaining += days_in_year;
                year -= 1;
            } else {
                break;
            }
        }
        
        // 此时 remaining 是负数，范围在 -days_in_year .. 0
        // 需要转换为该年中的第几天
        let days_in_this_year = if is_leap_year(year) { 366 } else { 365 };
        let day_of_year = (days_in_this_year as i64 + remaining) as i32 + 1;
        let (month, day) = day_of_year_to_month_day(year, day_of_year);
        (year, month, day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_days_to_ymd_epoch() {
        let (y, m, d) = days_to_ymd(0);
        assert_eq!((y, m, d), (1970, 1, 1));
    }
    
    #[test]
    fn test_days_to_ymd_positive() {
        let (y, m, d) = days_to_ymd(1);
        assert_eq!((y, m, d), (1970, 1, 2));
        
        let (y, m, d) = days_to_ymd(31);
        assert_eq!((y, m, d), (1970, 2, 1));
    }
    
    #[test]
    fn test_days_to_ymd_negative() {
        let (y, m, d) = days_to_ymd(-1);
        assert_eq!((y, m, d), (1969, 12, 31));
        
        let (y, m, d) = days_to_ymd(-2);
        assert_eq!((y, m, d), (1969, 12, 30));
        
        let (y, m, d) = days_to_ymd(-365);
        assert_eq!((y, m, d), (1969, 1, 1));
    }
    
    #[test]
    fn test_days_to_ymd_leap_year() {
        let (y, m, d) = days_to_ymd(365 + 365 + 31 + 28);
        assert_eq!((y, m, d), (1972, 2, 29));
    }
}
