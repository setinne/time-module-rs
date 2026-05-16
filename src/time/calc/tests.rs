// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html

// 测试模块内部导入，不在外部重复导入
#[cfg(test)]
mod tests {
    use crate::time::calc::convert::utc_to_fulltime;
    use crate::time::calc::jd::{gregorian_to_jd, jd_to_gregorian};
    use crate::time::calc::{set_calendar_type, CalendarType, weekday};

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
        let jd = gregorian_to_jd(2000, 2, 29);
        let (y, m, d) = jd_to_gregorian(jd);
        assert_eq!((y, m, d), (2000, 2, 29));
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

    #[test]
    fn test_julian_day_roundtrip() {
        let test_dates = [
            (-4713, 1, 1),
            (-1, 12, 31),
            (0, 1, 1),
            (1, 1, 1),
            (1582, 10, 15),
            (1970, 1, 1),
            (2038, 1, 19),
            (9999, 12, 31),
        ];
        for (y, m, d) in test_dates {
            let jd = gregorian_to_jd(y, m, d);
            let (y2, m2, d2) = jd_to_gregorian(jd);
            assert_eq!((y, m, d), (y2, m2, d2), "Failed for {}-{}-{}", y, m, d);
        }
    }

    #[test]
    fn test_weekday_consistency() {
        // 先设置为儒略历，计算儒略历日期 1582-10-05 的星期
        set_calendar_type(CalendarType::Julian);
        let wd_jul = weekday(1582, 10, 5);

        // 再设置为公历，计算公历日期 1582-10-15 的星期
        set_calendar_type(CalendarType::Gregorian);
        let wd_greg = weekday(1582, 10, 15);

        // 两者应该相同
        assert_eq!(wd_greg, wd_jul);
    }
}