// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! DST 判断逻辑

use super::rule::{DstRule, get_rule};
use crate::time::handle::formatting::FullTime;

/// 判断是否处于夏令时
/// 无规则的国家返回 false（不启用 DST）
pub fn is_dst_by_rules(time: &FullTime, country: &str) -> bool {
    let rule = match get_rule(country) {
        Some(r) => r,
        None => return false,
    };
    
    if !rule.is_enabled() {
        return false;
    }
    
    let start = get_boundary_seconds(time.year, rule, true);
    let end = get_boundary_seconds(time.year, rule, false);
    let current = day_of_year_seconds(time);
    
    if start < end {
        current >= start && current < end
    } else {
        current >= start || current < end
    }
}

/// 获取 DST 偏移（秒）
/// 无规则的国家返回 0
pub fn get_dst_offset_by_rules(country: &str) -> i32 {
    get_rule(country)
        .map(|r| r.offset_sec)
        .unwrap_or(0)
}

/// 计算边界秒数
fn get_boundary_seconds(year: i32, rule: &DstRule, is_start: bool) -> i32 {
    let (month, week, dow, hour) = if is_start {
        (rule.start_month, rule.start_week, rule.start_dow, rule.start_hour)
    } else {
        (rule.end_month, rule.end_week, rule.end_dow, rule.end_hour)
    };
    
    if month == 0 {
        return if is_start { 0 } else { i32::MAX };
    }
    
    let day = get_specific_weekday(year, month, week, dow);
    day_of_year_seconds_for_date(year, month, day, hour)
}

/// 获取某月第n个星期几的日期
fn get_specific_weekday(year: i32, month: u8, week: i8, dow: u8) -> u8 {
    let first_dow = weekday_of_date(year, month, 1) as i32;
    let days_in_month = days_in_month(month, is_leap_year(year));
    
    if week == 5 {
        let last_day = days_in_month as i32;
        let last_dow = weekday_of_date(year, month, last_day as u8) as i32;
        let diff = (last_dow - dow as i32 + 7) % 7;
        (last_day - diff) as u8
    } else {
        let target = 1 + (week as i32 - 1) * 7 + ((dow as i32 - first_dow + 7) % 7);
        target.min(days_in_month as i32) as u8
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_in_month(month: u8, leap: bool) -> u8 {
    match month {
        2 => if leap { 29 } else { 28 },
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

fn weekday_of_date(year: i32, month: u8, day: u8) -> u8 {
    let mut y = year;
    let mut m = month as i32;
    if m < 3 {
        m += 12;
        y -= 1;
    }
    let k = y % 100;
    let j = y / 100;
    let h = (day as i32 + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;
    ((h + 6) % 7 + 1) as u8
}

fn day_of_year_seconds_for_date(year: i32, month: u8, day: u8, hour: u8) -> i32 {
    let mut days = 0;
    for m in 1..month {
        days += days_in_month(m, is_leap_year(year)) as i32;
    }
    days += (day - 1) as i32;
    days * 86400 + (hour as i32) * 3600
}

fn day_of_year_seconds(time: &FullTime) -> i32 {
    day_of_year_seconds_for_date(time.year, time.month as u8, time.day as u8, time.hour as u8)
}