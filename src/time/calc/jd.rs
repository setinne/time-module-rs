// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 儒略日与公历互转（基于 Fliegel-Van Flandern 算法）
//! 支持范围：公元前 4713 年至公元后 9999 年（proleptic Gregorian）

/// 公历 → 儒略日（Julian Day Number）
pub fn gregorian_to_jd(year: i32, month: i32, day: i32) -> i64 {
    let a = (14 - month) / 12;
    let y = (year + 4800 - a) as i64;
    let m = (month + 12 * a - 3) as i64;
    day as i64 + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045
}

/// 儒略日 → 公历
pub fn jd_to_gregorian(jd: i64) -> (i32, i32, i32) {
    let a = jd + 32044;
    let b = (4 * a + 3) / 146097;
    let c = a - (146097 * b) / 4;
    let d = (4 * c + 3) / 1461;
    let e = c - (1461 * d) / 4;
    let m = (5 * e + 2) / 153;
    let day = (e - (153 * m + 2) / 5 + 1) as i32;
    let month = (m + 3 - 12 * (m / 10)) as i32;
    let year = (100 * b + d - 4800 + m / 10) as i32;
    (year, month, day)
}

/// Unix 纪元（1970-01-01）的儒略日
pub const JULIAN_DAY_EPOCH: i64 = 2440588;

/// 儒略日 → Unix 时间戳（秒）
pub fn julian_day_to_unix_secs(jd: i64) -> i64 {
    (jd - JULIAN_DAY_EPOCH) * 86400
}

/// Unix 时间戳（秒）→ 儒略日
pub fn unix_secs_to_julian_day(secs: i64) -> i64 {
    // 使用欧几里得除法正确处理负数
    JULIAN_DAY_EPOCH + secs.div_euclid(86400)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epoch() {
        assert_eq!(gregorian_to_jd(1970, 1, 1), 2440588);
        assert_eq!(jd_to_gregorian(2440588), (1970, 1, 1));
    }

    #[test]
    fn test_known_dates() {
        assert_eq!(gregorian_to_jd(2000, 1, 1), 2451545);
        assert_eq!(jd_to_gregorian(2451545), (2000, 1, 1));
    }

    #[test]
    fn test_leap_day() {
        let jd = gregorian_to_jd(2000, 2, 29);
        assert_eq!(jd, 2451604);
        assert_eq!(jd_to_gregorian(jd), (2000, 2, 29));
    }

    #[test]
    fn test_negative_unix() {
        let secs = -86400; // 1969-12-31 UTC
        let jd = unix_secs_to_julian_day(secs);
        assert_eq!(jd, 2440587);
        let (y, m, d) = jd_to_gregorian(jd);
        assert_eq!((y, m, d), (1969, 12, 31));
    }

    #[test]
    fn test_roundtrip_extreme() {
        let cases = [
            (-4713, 1, 1),
            (-1, 12, 31),
            (1, 1, 1),
            (1582, 10, 15),
            (1970, 1, 1),
            (9999, 12, 31),
        ];
        for (y, m, d) in cases {
            let jd = gregorian_to_jd(y, m, d);
            let (y2, m2, d2) = jd_to_gregorian(jd);
            assert_eq!((y, m, d), (y2, m2, d2), "Failed at {}-{}-{}", y, m, d);
        }
    }
}