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

/// 将 Unix 时间戳（秒+微秒）转换为日期时间结构
/// 支持负数时间戳（1970年之前的日期）
pub fn utc_to_fulltime(utc_sec: u64, utc_us: i32, tz_offset: i32) -> FullTime {
    // 转换为带时区偏移的秒数（可能是负数）
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
    
    // 计算天数（向下取整）
    let days = if timestamp >= 0 {
        timestamp / SECS_PER_DAY
    } else {
        // 负数时间戳：-1 秒应该对应 1969-12-31 23:59:59
        (timestamp - (SECS_PER_DAY - 1)) / SECS_PER_DAY
    };
    
    // 计算剩余秒数（确保在 0..86400 范围内）
    let remaining_secs = timestamp - days * SECS_PER_DAY;
    let remaining_secs = if remaining_secs < 0 {
        remaining_secs + SECS_PER_DAY
    } else {
        remaining_secs
    };
    
    let hour = (remaining_secs / SECS_PER_HOUR) as i32;
    let minute = ((remaining_secs % SECS_PER_HOUR) / SECS_PER_MIN) as i32;
    let second = (remaining_secs % SECS_PER_MIN) as i32;
    
    // 将天数转换为年月日
    let (year, month, day) = days_to_ymd(days);
    
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

/// 将从 Unix 纪元（1970-01-01）开始的天数转换为年月日
fn days_to_ymd(days: i64) -> (i32, i32, i32) {
    // 处理负数天数（1970年之前）
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
        
        // 现在 remaining_days 是负数或零，表示从 year 年 1 月 1 日开始的天数偏移
        let day_of_year = (remaining_days + days_in_year(year)) as i32 + 1;
        let (month, day) = day_of_year_to_month_day(year, day_of_year);
        return (year, month, day);
    }
    
    // 正数天数（1970年及以后）
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

/// 判断是否为闰年
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// 获取指定年份的天数
fn days_in_year(year: i32) -> i64 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// 将一年中的第几天转换为月份和日期
fn day_of_year_to_month_day(year: i32, mut day_of_year: i32) -> (i32, i32) {
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_utc_to_fulltime_positive() {
        // 1970-01-01 00:00:00 UTC+8
        let ft = utc_to_fulltime(0, 0, 28800);
        assert_eq!(ft.year, 1970);
        assert_eq!(ft.month, 1);
        assert_eq!(ft.day, 1);
        assert_eq!(ft.hour, 8);
        assert_eq!(ft.minute, 0);
        assert_eq!(ft.second, 0);
    }
    
    #[test]
    fn test_utc_to_fulltime_positive_edge() {
        // 1970-01-01 00:00:00 UTC-2 应该等于 1969-12-31 22:00:00
        let ft = utc_to_fulltime(0, 0, -7200);
        assert_eq!(ft.year, 1969);
        assert_eq!(ft.month, 12);
        assert_eq!(ft.day, 31);
        assert_eq!(ft.hour, 22);
        assert_eq!(ft.minute, 0);
        assert_eq!(ft.second, 0);
    }
    
    #[test]
    fn test_utc_to_fulltime_negative() {
        // UTC 0 + (-28800) = -28800 秒 = 1969-12-31 16:00:00
        let ft = utc_to_fulltime(0, 0, -28800);
        assert_eq!(ft.year, 1969);
        assert_eq!(ft.month, 12);
        assert_eq!(ft.day, 31);
        assert_eq!(ft.hour, 16);
        assert_eq!(ft.minute, 0);
        assert_eq!(ft.second, 0);
    }
    
    #[test]
    fn test_specific_timestamp() {
        // 2024-06-15 12:30:45 UTC
        // 时间戳: 1718454645
        let ft = utc_to_fulltime(1718454645, 123456, 0);
        assert_eq!(ft.year, 2024);
        assert_eq!(ft.month, 6);
        assert_eq!(ft.day, 15);
        assert_eq!(ft.hour, 12);
        assert_eq!(ft.minute, 30);
        assert_eq!(ft.second, 45);
        assert_eq!(ft.ms, 123);
        assert_eq!(ft.us, 123456);
    }
    
    #[test]
    fn test_leap_year() {
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2020));
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2021));
    }
    
    #[test]
    fn test_day_of_year_conversion() {
        // 2020年是闰年，2月29日应该是第60天
        let (month, day) = day_of_year_to_month_day(2020, 60);
        assert_eq!(month, 2);
        assert_eq!(day, 29);
        
        // 2021年不是闰年，3月1日是第60天
        let (month, day) = day_of_year_to_month_day(2021, 60);
        assert_eq!(month, 3);
        assert_eq!(day, 1);
    }
    
    #[test]
    fn test_various_timestamps() {
        // 测试 2000-01-01 00:00:00 UTC
        let timestamp = 946684800;
        let ft = utc_to_fulltime(timestamp, 0, 0);
        assert_eq!(ft.year, 2000);
        assert_eq!(ft.month, 1);
        assert_eq!(ft.day, 1);
        
        // 测试 2038-01-19 03:14:07 UTC (32位时间戳溢出边界)
        let timestamp = 2147483647;
        let ft = utc_to_fulltime(timestamp, 0, 0);
        assert_eq!(ft.year, 2038);
        assert_eq!(ft.month, 1);
        assert_eq!(ft.day, 19);
        assert_eq!(ft.hour, 3);
        assert_eq!(ft.minute, 14);
        assert_eq!(ft.second, 7);
    }
}