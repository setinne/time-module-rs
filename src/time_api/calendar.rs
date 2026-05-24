//! 历法设置 API

use crate::time::calc::{CalendarType, set_calendar_type, get_calendar_type};

/// 设置历法类型（0=公历，1=儒略历）
#[no_mangle]
pub extern "C" fn api_SetCalendarType(cal_type: i32) {
    match cal_type {
        1 => set_calendar_type(CalendarType::Julian),
        _ => set_calendar_type(CalendarType::Gregorian),
    }
}

/// 获取当前历法类型（0=公历，1=儒略历）
#[no_mangle]
pub extern "C" fn api_GetCalendarType() -> i32 {
    get_calendar_type() as i32
}