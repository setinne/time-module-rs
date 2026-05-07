// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! DST 规则数据结构

use std::collections::HashMap;
use std::sync::Once;

static mut DST_RULES: *mut HashMap<String, DstRule> = std::ptr::null_mut();
static INIT_ONCE: Once = Once::new();

/// DST 规则
#[derive(Debug, Clone)]
pub struct DstRule {
    pub country: String,
    pub start_month: u8,
    pub start_week: i8,
    pub start_dow: u8,
    pub start_hour: u8,
    pub end_month: u8,
    pub end_week: i8,
    pub end_dow: u8,
    pub end_hour: u8,
    pub offset_sec: i32,
    pub break_start_month: u8,
    pub break_start_day: u8,
    pub break_start_hour: u8,
    pub break_end_month: u8,
    pub break_end_day: u8,
    pub break_end_hour: u8,
}

impl DstRule {
    pub fn is_enabled(&self) -> bool {
        self.start_month != 0 && self.end_month != 0
    }
}

fn parse_week(week_str: &str) -> i8 {
    match week_str {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        "D" => 4,
        "E" => 5,
        "0" => 5,
        _ => week_str.parse().unwrap_or(0),
    }
}

fn parse_dow(dow_str: &str) -> u8 {
    match dow_str {
        "SUN" => 1,
        "MON" => 2,
        "TUE" => 3,
        "WED" => 4,
        "THU" => 5,
        "FRI" => 6,
        "SAT" => 7,
        _ => dow_str.parse().unwrap_or(0),
    }
}

pub fn parse_rule_line(line: &str) -> Option<DstRule> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 16 {
        return None;
    }

    Some(DstRule {
        country: parts[0].to_string(),
        start_month: parts[1].parse().unwrap_or(0),
        start_week: parse_week(parts[2]),
        start_dow: parse_dow(parts[3]),
        start_hour: parts[4].parse().unwrap_or(0),
        end_month: parts[5].parse().unwrap_or(0),
        end_week: parse_week(parts[6]),
        end_dow: parse_dow(parts[7]),
        end_hour: parts[8].parse().unwrap_or(0),
        offset_sec: parts[9].parse().unwrap_or(3600),
        break_start_month: parts[10].parse().unwrap_or(0),
        break_start_day: parts[11].parse().unwrap_or(0),
        break_start_hour: parts[12].parse().unwrap_or(0),
        break_end_month: parts[13].parse().unwrap_or(0),
        break_end_day: parts[14].parse().unwrap_or(0),
        break_end_hour: parts[15].parse().unwrap_or(0),
    })
}

fn get_rules_map() -> &'static mut HashMap<String, DstRule> {
    unsafe {
        INIT_ONCE.call_once(|| {
            let lines = crate::resources::get_dst_rules_lines();
            let mut map = HashMap::new();
            for line in lines {
                if line.starts_with('#') || line.trim().is_empty() {
                    continue;
                }
                if let Some(rule) = parse_rule_line(&line) {
                    map.insert(rule.country.clone(), rule);
                }
            }
            DST_RULES = Box::into_raw(Box::new(map));
        });
        &mut *DST_RULES
    }
}

pub fn get_rule(country: &str) -> Option<&'static DstRule> {
    get_rules_map().get(country)
}