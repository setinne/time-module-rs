// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html

// 加载 countries_tz.txt，提供国家代码到时区偏移列表的映射
use std::collections::HashMap;
use std::sync::Once;

static mut COUNTRY_MAP: *mut HashMap<String, Vec<i32>> = std::ptr::null_mut();
static INIT_ONCE: Once = Once::new();

fn get_country_map() -> &'static mut HashMap<String, Vec<i32>> {
    unsafe {
        INIT_ONCE.call_once(|| {
            let map = crate::resources::parse_countries_tz();
            COUNTRY_MAP = Box::into_raw(Box::new(map));
        });
        &mut *COUNTRY_MAP
    }
}

pub fn offset_from_location(longitude: f64, latitude: f64, country: Option<&str>) -> i32 {
    let lon = longitude.clamp(-180.0, 180.0);
    let _ = latitude.clamp(-90.0, 90.0);

    if let Some(code) = country {
        let code_upper = code.to_uppercase();
        let map = get_country_map();
        if let Some(offsets) = map.get(&code_upper) {
            if offsets.len() == 1 {
                return offsets[0];
            }
            if offsets.len() > 1 {
                match code_upper.as_str() {
                    "US" | "CA" => {
                        if lon < -120.0 {
                            return offsets[0];
                        } else if lon < -97.0 {
                            return offsets[1];
                        } else if lon < -82.5 {
                            return offsets[2];
                        } else if lon < -67.5 {
                            return offsets[3];
                        } else {
                            return offsets[4];
                        }
                    }
                    "AU" => {
                        if lon < 120.0 {
                            return offsets[0];
                        } else if lon < 142.5 {
                            return offsets[1];
                        } else {
                            return offsets[2];
                        }
                    }
                    "RU" => {
                        if lon < 30.0 {
                            return offsets[0];
                        } else if lon < 60.0 {
                            return offsets[1];
                        } else if lon < 90.0 {
                            return offsets[2];
                        } else {
                            return offsets[3];
                        }
                    }
                    _ => return offsets[0],
                }
            }
        }
    }
    // 回退：经度每 15° 对应 1 小时
    ((lon / 15.0).round() * 3600.0) as i32
}