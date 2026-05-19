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
    DstNotAvailable = 9,
    InternalPanic = 10,
    UnknownError = 11,
    NotInitialized = 12,
    InvalidDate = 13,
    BufferTooSmall = 14,
    NtpServerUnreachable = 15,
    NtpResponseInvalid = 16,
    LogCallbackNotSet = 17, 
    // v0.2.18 新增
    TimezoneOffsetOutOfRange = 18,   // 时区偏移超出有效范围
    TimezoneNameNotFound = 19,       // 时区名称不存在
    DstRuleNotFound = 20,            // DST规则未找到
    AsyncTaskFailed = 21,            // 异步任务启动失败
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
            TimeErrorCode::InternalPanic => "Internal panic occurred",
            TimeErrorCode::UnknownError => "Unknown error",
            TimeErrorCode::NotInitialized => "Component not initialized",
            TimeErrorCode::InvalidDate => "Invalid date",
            TimeErrorCode::BufferTooSmall => "Buffer too small",
            TimeErrorCode::NtpServerUnreachable => "NTP server unreachable",
            TimeErrorCode::NtpResponseInvalid => "Invalid NTP response",
            TimeErrorCode::LogCallbackNotSet => "Log callback not set",
            // v0.2.18 新增
            TimeErrorCode::TimezoneOffsetOutOfRange => "Timezone offset out of range (-43200..43200)",
            TimeErrorCode::TimezoneNameNotFound => "Timezone name not found",
            TimeErrorCode::DstRuleNotFound => "DST rule not found for country",
            TimeErrorCode::AsyncTaskFailed => "Failed to start async task",
        }
    }
}