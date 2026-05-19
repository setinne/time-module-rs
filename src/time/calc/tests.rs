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

/// 验证不同时区偏移得到的本地时间互不影响，
/// 象征多实例场景下的状态隔离（底层计算函数应无副作用）
#[test]
fn test_timezone_offset_independence() {
    use crate::time::calc::convert::utc_to_fulltime;

    let utc_sec: u64 = 1700000000;
    let utc_us: i32 = 0;

    // 偏移 +8 小时（UTC+8）
    let ft_plus8 = utc_to_fulltime(utc_sec, utc_us, 28800);
    // 偏移 -5 小时（UTC-5）
    let ft_minus5 = utc_to_fulltime(utc_sec, utc_us, -18000);

    // 两者的小时、日期应当不同
    assert_ne!(ft_plus8.hour, ft_minus5.hour);

    // 检查 UTC+8 的时间是否比 UTC-5 的时间超前 13 小时（不考虑日期边界）
    let plus8_total_seconds = ft_plus8.hour * 3600 + ft_plus8.minute * 60 + ft_plus8.second;
    let minus5_total_seconds = ft_minus5.hour * 3600 + ft_minus5.minute * 60 + ft_minus5.second;
    let diff = (plus8_total_seconds as i32 - minus5_total_seconds as i32).rem_euclid(86400);
    // 差应为 13 小时（46800 秒）
    assert_eq!(diff, 46800);

    // 验证没有全局状态污染：连续调用同一偏移应得到相同结果
    let ft_plus8_again = utc_to_fulltime(utc_sec, utc_us, 28800);
    assert_eq!(ft_plus8.year, ft_plus8_again.year);
    assert_eq!(ft_plus8.month, ft_plus8_again.month);
    assert_eq!(ft_plus8.day, ft_plus8_again.day);
    assert_eq!(ft_plus8.hour, ft_plus8_again.hour);
}
// ============================================================================
// v0.2.18 新增测试：错误码和时区偏移边界
// ============================================================================

#[cfg(test)]
mod v0_2_18_tests {
    use crate::MIN_VALID_OFFSET;
    use crate::MAX_VALID_OFFSET;
    use crate::error::TimeErrorCode;

    #[test]
    fn test_timezone_offset_range() {
        // 测试有效范围内的偏移
        let valid_offsets = vec![-43200, -28800, 0, 28800, 43200];
        for &offset in &valid_offsets {
            // 验证偏移在范围内
            assert!(offset >= MIN_VALID_OFFSET && offset <= MAX_VALID_OFFSET);
        }

        // 测试超出范围的偏移
        let invalid_offsets = vec![-50400, -50000, 50000, 50400];
        for &offset in &invalid_offsets {
            assert!(offset < MIN_VALID_OFFSET || offset > MAX_VALID_OFFSET);
        }
    }

    #[test]
    fn test_error_code_values() {
        // 验证新增错误码的值
        assert_eq!(TimeErrorCode::TimezoneOffsetOutOfRange as i32, 18);
        assert_eq!(TimeErrorCode::TimezoneNameNotFound as i32, 19);
        assert_eq!(TimeErrorCode::DstRuleNotFound as i32, 20);
        assert_eq!(TimeErrorCode::AsyncTaskFailed as i32, 21);
    }
}