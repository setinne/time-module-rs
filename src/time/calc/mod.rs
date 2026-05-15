// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! UTC 时间戳转本地时间结构体

mod convert;
mod jd;
mod calendar;
mod sync;
mod tests;
pub mod weekday;

pub use convert::{utc_to_fulltime, utc_to_fulltime_ns, set_calendar_type, get_calendar_type};
pub use sync::{check_time_accuracy, get_calibrated_local_time, is_time_synced};
pub use calendar::CalendarType;
pub use weekday::{weekday, weekday_iso, weekday_name, weekday_name_zh};


#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FullTime {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
    pub ms: i32,
    pub us: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FullTimeNs {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
    pub ns: i32,
}