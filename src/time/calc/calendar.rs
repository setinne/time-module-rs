// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 公历与儒略历转换

use super::jd::*;

/// 历法类型
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalendarType {
    Gregorian = 0,  // 公历（默认）
    Julian = 1,     // 儒略历
}

/// 公历 → 儒略历日期
/// 规则：公历日期减去 10 天（1582 年 10 月 15 日 → 1582 年 10 月 5 日）
pub fn gregorian_to_julian(year: i32, month: i32, day: i32) -> (i32, i32, i32) {
    // 使用儒略日作为中间转换
    let jd = gregorian_to_jd(year, month, day);
    jd_to_julian(jd)
}

/// 儒略历 → 公历日期
/// 规则：儒略历日期加上 10 天（1582 年 10 月 5 日 → 1582 年 10 月 15 日）
pub fn julian_to_gregorian(year: i32, month: i32, day: i32) -> (i32, i32, i32) {
    let jd = julian_to_jd(year, month, day);
    jd_to_gregorian(jd)
}

/// 儒略日 → 儒略历日期
pub fn jd_to_julian(jd: i64) -> (i32, i32, i32) {
    // 使用 Fliegel-Van Flandern 算法的变体（针对儒略历）
    let jd = jd + 1401;
    let r = jd;
    let g = r / 1461;
    let dg = r % 1461;
    let a = (dg / 365 + 1) * 3 / 4;
    let da = dg - a * 365;
    let y = g * 4 + a;
    let m = (da * 100 + 52) / 3060;
    let d = da - (m * 306 + 5) / 10 + 1;
    let year = y + (m + 2) / 12;
    let month = (m + 2) % 12 + 1;
    
    (year as i32, month as i32, d as i32)
}

/// 儒略历 → 儒略日
pub fn julian_to_jd(year: i32, month: i32, day: i32) -> i64 {
    let y = if month <= 2 { year - 1 } else { year };
    let m = if month <= 2 { month + 12 } else { month };
    
    (36525 * (y + 4712) / 100) as i64
        + (306 * (m + 1) / 10) as i64
        + day as i64
        - 45
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gregorian_to_julian() {
        // 1582-10-15 公历 → 1582-10-05 儒略历
        let (y, m, d) = gregorian_to_julian(1582, 10, 15);
        assert_eq!((y, m, d), (1582, 10, 5));
        
        // 2026-05-09 公历 → 2026-04-26 儒略历
        let (y, m, d) = gregorian_to_julian(2026, 5, 9);
        assert_eq!((y, m, d), (2026, 4, 26));
    }

    #[test]
    fn test_julian_to_gregorian() {
        // 1582-10-05 儒略历 → 1582-10-15 公历
        let (y, m, d) = julian_to_gregorian(1582, 10, 5);
        assert_eq!((y, m, d), (1582, 10, 15));
        
        // 2026-04-26 儒略历 → 2026-05-09 公历
        let (y, m, d) = julian_to_gregorian(2026, 4, 26);
        assert_eq!((y, m, d), (2026, 5, 9));
    }
}