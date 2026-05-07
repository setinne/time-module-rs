// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! UTC 时间戳转本地时间结构体
mod converter;
mod days;
mod tests;

pub use converter::utc_to_fulltime;

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