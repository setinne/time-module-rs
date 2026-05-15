// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


#[cfg(test)]
mod tests {
    use super::super::convert::utc_to_fulltime;
    use super::super::jd::{gregorian_to_jd, jd_to_gregorian};

    #[test]
    fn test_utc_to_fulltime_positive() {
        let ft = utc_to_fulltime(0, 0, 28800);
        assert_eq!(ft.year, 1970);
        assert_eq!(ft.month, 1);
        assert_eq!(ft.day, 1);
        assert_eq!(ft.hour, 8);
    }

    #[test]
    fn test_utc_to_fulltime_negative() {
        let ft = utc_to_fulltime(0, 0, -28800);
        assert_eq!(ft.year, 1969);
        assert_eq!(ft.month, 12);
        assert_eq!(ft.day, 31);
        assert_eq!(ft.hour, 16);
    }

    #[test]
    fn test_2038_timestamp() {
        let ft = utc_to_fulltime(2147483647, 0, 0);
        assert_eq!(ft.year, 2038);
        assert_eq!(ft.month, 1);
        assert_eq!(ft.day, 19);
        assert_eq!(ft.hour, 3);
        assert_eq!(ft.minute, 14);
        assert_eq!(ft.second, 7);
    }

    #[test]
    fn test_leap_year_via_jd() {
        // 闰年：2000-02-29
        let jd = gregorian_to_jd(2000, 2, 29);
        let (y, m, d) = jd_to_gregorian(jd);
        assert_eq!((y, m, d), (2000, 2, 29));

        // 非闰年：1900-03-01（1900年2月29日不存在）
        let jd = gregorian_to_jd(1900, 3, 1);
        let (y, m, d) = jd_to_gregorian(jd);
        assert_eq!((y, m, d), (1900, 3, 1));
    }

    #[test]
    fn test_day_of_year_via_jd() {
        let jd = gregorian_to_jd(2020, 2, 29);
        let (y, m, d) = jd_to_gregorian(jd);
        assert_eq!((y, m, d), (2020, 2, 29));
    }
}

#[test]
fn test_is_leap_year() {
    use crate::time_api::api_IsLeapYear;
    use crate::time_api::api_IsLeapYearEx;
    assert!(api_IsLeapYear(2000));
    assert!(!api_IsLeapYear(1900));
    assert!(!api_IsLeapYear(2021));
    assert_eq!(api_IsLeapYearEx(2000), 1);
    assert_eq!(api_IsLeapYearEx(1900), 0);
}

#[test]
fn test_day_of_year() {
    use crate::time_api::api_DayOfYear;
    assert_eq!(api_DayOfYear(2020, 1, 1), 1);
    assert_eq!(api_DayOfYear(2020, 12, 31), 366); // 闰年
    assert_eq!(api_DayOfYear(2021, 12, 31), 365);
    assert_eq!(api_DayOfYear(2021, 3, 1), 60); // 非闰年3月1日为第60天
}

#[test]
fn test_unix_timestamp() {
    use crate::time_api::{
        api_GetUnixTimestamp,
        api_GetUnixTimestampMs,
        api_GetUnixTimestampUs,
        api_GetUnixTimestampNs,
    };
    let ts = api_GetUnixTimestamp();
    assert!(ts > 0);
    let ts_ms = api_GetUnixTimestampMs();
    assert!(ts_ms > ts * 1000);
    let ts_us = api_GetUnixTimestampUs();
    assert!(ts_us > ts_ms * 1000);
    let ts_ns = api_GetUnixTimestampNs();
    assert!(ts_ns > ts_us * 1000);
}

#[test]
fn test_invalid_date() {
    use crate::time_api::api_DayOfYear;
    use crate::time_api::api_GetWeekday;
    use crate::time_api::api_GetLastError;
    use crate::error::TimeErrorCode;

    assert_eq!(api_DayOfYear(2021, 2, 29), -1);
    assert_eq!(api_GetLastError(), TimeErrorCode::InvalidDate as i32);
    assert_eq!(api_GetWeekday(2021, 2, 29), -1);
    assert_eq!(api_GetLastError(), TimeErrorCode::InvalidDate as i32);
}