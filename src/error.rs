// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


// 错误码定义
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeErrorCode {
    Success = 0,
    InvalidParam = 1,
    NtpTimeout = 2,
    NoNtpServer = 3,
    Timeout = 4,
    NotSynced = 5,
    FileNotFound = 6,
    ParseError = 7,
    CountryNotFound = 8,
    DstNotAvailable = 9,  // DST 规则不可用（无规则或已禁用）
}

impl TimeErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            TimeErrorCode::Success => "Success",
            TimeErrorCode::InvalidParam => "Invalid parameter",
            TimeErrorCode::NtpTimeout => "NTP request timeout",
            TimeErrorCode::NoNtpServer => "No NTP server available",
            TimeErrorCode::Timeout => "Operation timeout",
            TimeErrorCode::NotSynced => "NTP not synced yet",
            TimeErrorCode::FileNotFound => "Resource file not found",
            TimeErrorCode::ParseError => "Parse error",
            TimeErrorCode::CountryNotFound => "Country code not found in timezone database",
            TimeErrorCode::DstNotAvailable => "DST not available (no rule or disabled)",
        }
    }
}
pub use TimeErrorCode as TimeError;
