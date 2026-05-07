// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! Windows 系统 API 获取 DST 信息

#![allow(non_snake_case)]

use std::mem;

type DWORD = u32;
type LONG = i32;

const TIME_ZONE_ID_DAYLIGHT: DWORD = 2;

#[repr(C)]
#[derive(Default)]
struct SYSTEMTIME {
    wYear: u16,
    wMonth: u16,
    wDayOfWeek: u16,
    wDay: u16,
    wHour: u16,
    wMinute: u16,
    wSecond: u16,
    wMilliseconds: u16,
}

#[repr(C)]
struct TIME_ZONE_INFORMATION {
    Bias: LONG,
    StandardName: [u16; 32],
    StandardDate: SYSTEMTIME,
    StandardBias: LONG,
    DaylightName: [u16; 32],
    DaylightDate: SYSTEMTIME,
    DaylightBias: LONG,
}

extern "system" {
    fn GetTimeZoneInformation(lpTimeZoneInformation: *mut TIME_ZONE_INFORMATION) -> DWORD;
}

/// 获取系统 DST 偏移（秒）
pub fn get_system_dst_offset() -> i32 {
    unsafe {
        let mut tz: TIME_ZONE_INFORMATION = mem::zeroed();
        let result = GetTimeZoneInformation(&mut tz);
        
        if result == TIME_ZONE_ID_DAYLIGHT {
            (tz.DaylightBias as i32).abs() * 60
        } else {
            0
        }
    }
}

/// 判断系统当前是否处于 DST
pub fn is_system_dst() -> bool {
    unsafe {
        let mut tz: TIME_ZONE_INFORMATION = mem::zeroed();
        let result = GetTimeZoneInformation(&mut tz);
        result == TIME_ZONE_ID_DAYLIGHT
    }
}

/// 获取系统时区基础偏移（秒）
pub fn get_system_base_bias() -> i32 {
    unsafe {
        let mut tz: TIME_ZONE_INFORMATION = mem::zeroed();
        GetTimeZoneInformation(&mut tz);
        -tz.Bias as i32 * 60
    }
}

/// 获取系统当前完整时区偏移（含 DST）
pub fn get_system_current_bias() -> i32 {
    unsafe {
        let mut tz: TIME_ZONE_INFORMATION = mem::zeroed();
        let result = GetTimeZoneInformation(&mut tz);
        let base = -tz.Bias as i32 * 60;
        if result == TIME_ZONE_ID_DAYLIGHT {
            base + (tz.DaylightBias as i32) * 60
        } else {
            base + (tz.StandardBias as i32) * 60
        }
    }
}