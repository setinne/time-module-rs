// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 公历与儒略历转换

use super::jd::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalendarType {
    Gregorian = 0,
    Julian = 1,
}

/// 公历 → 儒略历日期
pub fn gregorian_to_julian(year: i32, month: i32, day: i32) -> (i32, i32, i32) {
    let jd = gregorian_to_jd(year, month, day);
    jd_to_julian(jd)
}

/// 儒略历 → 公历日期
pub fn julian_to_gregorian(year: i32, month: i32, day: i32) -> (i32, i32, i32) {
    let jd = julian_to_jd(year, month, day);
    jd_to_gregorian(jd)
}

/// 儒略历日期 → 儒略日
fn julian_to_jd(year: i32, month: i32, day: i32) -> i64 {
    let a = (14 - month) / 12;
    let y = (year + 4800 - a) as i64;
    let m = (month + 12 * a - 3) as i64;
    
    day as i64 
        + (153 * m + 2) / 5 
        + 365 * y 
        + y / 4 
        - 32083  // 儒略历不修正格里历改革
}

/// 儒略日 → 儒略历日期
fn jd_to_julian(jd: i64) -> (i32, i32, i32) {
    let a = jd + 32082;
    let b = (4 * a + 3) / 1461;
    let c = a - (1461 * b) / 4;
    let d = (5 * c + 2) / 153;
    
    let day = (c - (153 * d + 2) / 5 + 1) as i32;
    let month = (d + 3 - 12 * (d / 10)) as i32;
    let year = (b - 4800 + d / 10) as i32;
    
    (year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gregorian_to_julian() {
        assert_eq!(gregorian_to_julian(1582, 10, 15), (1582, 10, 5));
        assert_eq!(gregorian_to_julian(2026, 5, 10), (2026, 4, 27));
    }

    #[test]
    fn test_julian_to_gregorian() {
        assert_eq!(julian_to_gregorian(1582, 10, 5), (1582, 10, 15));
        assert_eq!(julian_to_gregorian(2026, 4, 27), (2026, 5, 10));
    }
    
    #[test]
    fn test_roundtrip() {
        let test_dates = [
            (1582, 10, 15),
            (1970, 1, 1),
            (2000, 1, 1),
            (2026, 5, 10),
        ];
        
        for (y, m, d) in test_dates {
            let (jy, jm, jd) = gregorian_to_julian(y, m, d);
            let (gy, gm, gd) = julian_to_gregorian(jy, jm, jd);
            assert_eq!((y, m, d), (gy, gm, gd));
        }
    }
}