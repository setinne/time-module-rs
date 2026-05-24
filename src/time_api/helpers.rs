//! 辅助函数

use std::panic::UnwindSafe;
use crate::error::TimeErrorCode;

pub fn result_to_i32(result: Result<(), TimeErrorCode>) -> i32 {
    match result {
        Ok(()) => TimeErrorCode::Success as i32,
        Err(e) => e as i32,
    }
}

pub fn safe_catch<T, F>(f: F, default: T) -> T
where
    F: FnOnce() -> T + UnwindSafe,
{
    std::panic::catch_unwind(f).unwrap_or(default)
}

/// 判断是否为有效日期
pub fn is_valid_date(year: i32, month: i32, day: i32) -> bool {
    if month < 1 || month > 12 {
        return false;
    }
    let days_in_month = match month {
        2 => {
            let leap = crate::time_api::date_utils::api_IsLeapYear(year);
            if leap { 29 } else { 28 }
        }
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    day >= 1 && day <= days_in_month
}