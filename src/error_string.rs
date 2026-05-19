// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 错误码转字符串（动态分配，调用者必须用 api_FreeString 释放）

use std::ffi::CString;
use std::os::raw::c_char;

/// 获取错误码描述字符串（动态分配，需调用 api_FreeString 释放）
#[no_mangle]
pub extern "C" fn get_error_string(code: i32) -> *mut c_char {
    let s = match code {
        0 => "Success",
        1 => "Invalid parameter",
        2 => "NTP timeout",
        3 => "No NTP server available",
        4 => "Operation timeout",
        5 => "NTP not synced yet",
        6 => "Resource file not found",
        7 => "Parse error",
        8 => "Country code not found",
        9 => "DST rule not available",
        10 => "Internal panic (recovered)",
        11 => "Unknown error",
        12 => "Component not initialized",
        13 => "Invalid date",
        14 => "Buffer too small",
        15 => "NTP server unreachable",
        16 => "Invalid NTP response",
        17 => "Log callback not set",
        18 => "Timezone offset out of range (-43200..43200)",
        19 => "Timezone name not found",
        20 => "DST rule not found for country",
        21 => "Failed to start async task",
        _ => "Unknown error code",
    };
    CString::new(s).unwrap_or_default().into_raw()
}