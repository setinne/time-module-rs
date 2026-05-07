// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html

// 加载 tz_offsets.txt，提供时区名称到偏移秒的映射
use std::collections::HashMap;
use std::sync::Once;

static mut TZNAME_MAP: *mut HashMap<String, i32> = std::ptr::null_mut();
static INIT_ONCE: Once = Once::new();

fn get_tzname_map() -> &'static mut HashMap<String, i32> {
    unsafe {
        INIT_ONCE.call_once(|| {
            let map = crate::resources::parse_tz_offsets();
            TZNAME_MAP = Box::into_raw(Box::new(map));
        });
        &mut *TZNAME_MAP
    }
}

pub fn get_offset_by_name(name: &str) -> Option<i32> {
    let map = get_tzname_map();
    map.get(name).copied()
}