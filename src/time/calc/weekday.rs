// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 星期几计算

use super::jd::gregorian_to_jd;

/// 计算星期几（0-6，0=Sunday, 1=Monday, ..., 6=Saturday）
pub fn weekday(year: i32, month: i32, day: i32) -> i32 {
    let jd = gregorian_to_jd(year, month, day);
    // 已知 1970-01-01 (JD 2440588) 是 Thursday (4 in 0-6 if Sunday=0? Let's test)
    // 实际 1970-01-01 是 Thursday。若定义 0=Sunday，则 Thursday=4。
    // 公式: (jd + 偏移) % 7
    // 我们希望 (2440588 + offset) % 7 == 4
    // 计算 2440588 % 7 = 2440588 mod 7 = ? 2440588 / 7 = 348655 remainder 3? 验证：7*348655=2440585，余3。
    // 所以 (3 + offset) % 7 == 4 => offset = 1。
    let offset = 1;
    ((jd + offset) % 7) as i32
}

/// 计算星期几（1-7，1=Monday, 7=Sunday）ISO 标准
pub fn weekday_iso(year: i32, month: i32, day: i32) -> i32 {
    let w = weekday(year, month, day);
    if w == 0 { 7 } else { w }
}

/// 获取英文星期名称（全称）
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

/// 获取中文星期名称
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
    fn test_weekday() {
        // 1970-01-01 星期四
        assert_eq!(weekday(1970, 1, 1), 4);
        assert_eq!(weekday_iso(1970, 1, 1), 4);
        assert_eq!(weekday_name(1970, 1, 1), "Thursday");
        assert_eq!(weekday_name_zh(1970, 1, 1), "星期四");
        // 2024-05-15 星期三 (实际2024-05-15是周三)
        assert_eq!(weekday(2024, 5, 15), 3);
    }
}