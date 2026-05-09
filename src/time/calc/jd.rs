// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 儒略日与公历互转（Fliegel-Van Flandern 算法）
//! 支持范围：公元前 4714 年至公元后 9999 年

/// 公历 → 儒略日 (Julian Day Number)
/// proleptic Gregorian calendar（向前无限推演公历）
pub fn gregorian_to_jd(year: i32, month: i32, day: i32) -> i64 {
    let y = if month <= 2 { year - 1 } else { year };
    let m = if month <= 2 { month + 12 } else { month };
    
    let a = y / 100;
    let b = a / 4;
    let c = 2 - a + b;
    
    (36525 * (y + 4716) / 100) as i64
        + (306 * (m + 1) / 10) as i64
        + day as i64
        + c as i64
        - 1524
}

/// 儒略日 → 公历 (proleptic Gregorian)
/// 直接数学计算，无循环
pub fn jd_to_gregorian(jd: i64) -> (i32, i32, i32) {
    let jd = jd + 1402;
    let r = jd;
    let g = r / 146097;
    let dg = r % 146097;
    let c = (dg / 36524 + 1) * 3 / 4;
    let dc = dg - c * 36524;
    let b = dc / 1461;
    let db = dc % 1461;
    let a = (db / 365 + 1) * 3 / 4;
    let da = db - a * 365;
    let y = g * 400 + c * 100 + b * 4 + a;
    let m = (da * 100 + 52) / 3060;
    let d = da - (m * 306 + 5) / 10 + 1;
    let year = y + (m + 2) / 12;
    let month = (m + 2) % 12 + 1;
    
    (year as i32, month as i32, d as i32)
}

/// 儒略日 → Unix 时间戳秒数
pub const JULIAN_DAY_EPOCH: i64 = 2440588;  // 1970-01-01 的儒略日

pub fn julian_day_to_unix_secs(jd: i64) -> i64 {
    (jd - JULIAN_DAY_EPOCH) * 86400
}

pub fn unix_secs_to_julian_day(secs: i64) -> i64 {
    JULIAN_DAY_EPOCH + secs / 86400
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gregorian_to_jd() {
        // 1970-01-01
        let jd = gregorian_to_jd(1970, 1, 1);
        assert_eq!(jd, 2440588);
        
        // 2000-01-01
        let jd = gregorian_to_jd(2000, 1, 1);
        assert_eq!(jd, 2451545);
        
        // 2026-05-09
        let jd = gregorian_to_jd(2026, 5, 9);
        assert_eq!(jd, 2460883);
    }

    #[test]
    fn test_jd_to_gregorian() {
        let (y, m, d) = jd_to_gregorian(2440588);
        assert_eq!((y, m, d), (1970, 1, 1));
        
        let (y, m, d) = jd_to_gregorian(2451545);
        assert_eq!((y, m, d), (2000, 1, 1));
        
        let (y, m, d) = jd_to_gregorian(2460883);
        assert_eq!((y, m, d), (2026, 5, 9));
    }

    #[test]
    fn test_extended_years() {
        // 公元前 10000-01-01
        let jd = gregorian_to_jd(-10000, 1, 1);
        assert_eq!(jd, -104015299);
        
        let (y, m, d) = jd_to_gregorian(jd);
        assert_eq!((y, m, d), (-10000, 1, 1));
        
        // 公元 20000-12-31
        let jd = gregorian_to_jd(20000, 12, 31);
        assert_eq!(jd, 5454388);
        
        let (y, m, d) = jd_to_gregorian(jd);
        assert_eq!((y, m, d), (20000, 12, 31));
    }
}