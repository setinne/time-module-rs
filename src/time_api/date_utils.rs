//! 日期工具 API

use crate::error::TimeErrorCode;
use crate::time_api::globals::LAST_ERROR;
use crate::time_api::helpers::is_valid_date;

/// 判断是否为闰年
#[no_mangle]
pub extern "C" fn api_IsLeapYear(year: i32) -> bool {
    (year.rem_euclid(4) == 0 && year.rem_euclid(100) != 0) || year.rem_euclid(400) == 0
}

/// 判断是否为闰年（Ex 版本，返回 1/0）
#[no_mangle]
pub extern "C" fn api_IsLeapYearEx(year: i32) -> i32 {
    if api_IsLeapYear(year) { 1 } else { 0 }
}

/// 获取指定日期在一年中的第几天（1-366），失败返回 -1
#[no_mangle]
pub extern "C" fn api_DayOfYear(year: i32, month: i32, day: i32) -> i32 {
    if !is_valid_date(year, month, day) {
        LAST_ERROR.store(TimeErrorCode::InvalidDate as i32, std::sync::atomic::Ordering::Release);
        return -1;
    }
    let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let is_leap = api_IsLeapYear(year);
    let mut total = 0;
    for m in 1..month {
        total += if m == 2 && is_leap {
            29
        } else {
            days_in_month[(m - 1) as usize]
        };
    }
    total + day
}