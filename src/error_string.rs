// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 错误码转字符串

use crate::error::TimeErrorCode;
use std::ffi::CString;
use std::os::raw::c_char;

pub fn get_error_string(code: i32) -> *const c_char {
    let err = match code {
        0 => TimeErrorCode::Success,
        1 => TimeErrorCode::InvalidParam,
        2 => TimeErrorCode::NtpTimeout,
        3 => TimeErrorCode::NoNtpServer,
        4 => TimeErrorCode::Timeout,
        5 => TimeErrorCode::NotSynced,
        6 => TimeErrorCode::FileNotFound,
        7 => TimeErrorCode::ParseError,
        8 => TimeErrorCode::CountryNotFound,
        9 => TimeErrorCode::DstNotAvailable,
        _ => TimeErrorCode::InvalidParam,
    };
    CString::new(err.as_str()).unwrap_or_default().into_raw()
}