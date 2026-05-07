// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! DST 后端抽象

use std::sync::atomic::{AtomicU8, Ordering};
use crate::time::handle::formatting::FullTime;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DSTBackend {
    RuleTable = 0,
    SystemAPI = 1,
}

static BACKEND: AtomicU8 = AtomicU8::new(DSTBackend::RuleTable as u8);

pub fn set_backend(backend: DSTBackend) {
    BACKEND.store(backend as u8, Ordering::Release);
}

pub fn get_backend() -> DSTBackend {
    match BACKEND.load(Ordering::Acquire) {
        1 => DSTBackend::SystemAPI,
        _ => DSTBackend::RuleTable,
    }
}

/// 判断是否处于 DST（自动选择后端）
pub fn is_dst(time: &FullTime, country: &str) -> bool {
    match get_backend() {
        DSTBackend::SystemAPI => {
            super::system::is_system_dst()
        }
        DSTBackend::RuleTable => {
            super::calculator::is_dst_by_rules(time, country)
        }
    }
}

/// 获取 DST 偏移（自动选择后端）
pub fn get_dst_offset(country: &str) -> i32 {
    match get_backend() {
        DSTBackend::SystemAPI => {
            super::system::get_system_dst_offset()
        }
        DSTBackend::RuleTable => {
            super::calculator::get_dst_offset_by_rules(country)
        }
    }
}