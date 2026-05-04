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

/// 根据经纬度和国家代码获取时区偏移（秒）
/// 
/// # 参数
/// - `longitude`: 经度，-180 到 180
/// - `latitude`: 纬度，-90 到 90
/// - `country`: 可选的国家代码（ISO 3166-1 alpha-2）
/// 
/// # 返回
/// 时区偏移秒数（东正西负）
pub fn offset_from_location(longitude: f64, latitude: f64, country: Option<&str>) -> i32 {
    // 限制经纬度范围
    let lon = longitude.clamp(-180.0, 180.0);
    let lat = latitude.clamp(-90.0, 90.0);
    
    if let Some(code) = country {
        let code_upper = code.to_uppercase();
        let map = get_country_map();
        if let Some(offsets) = map.get(&code_upper) {
            if offsets.len() == 1 {
                return offsets[0];
            }
            if offsets.len() > 1 {
                // 根据经纬度选择合适的时区
                return select_offset_by_location(&code_upper, offsets, lon, lat);
            }
        }
    }
    
    // 回退：经度每 15° 对应 1 小时（粗略估计）
    // 经度 0 对应 UTC+0，东经为正
    ((lon / 15.0).round() * 3600.0) as i32
}

/// 根据经纬度和国家代码选择合适的时区偏移
fn select_offset_by_location(country: &str, offsets: &[i32], lon: f64, lat: f64) -> i32 {
    match country {
        "US" | "CA" => {
            // 北美时区（主要基于经度，忽略阿拉斯加和夏威夷的特殊性）
            if lat > 60.0 {
                // 阿拉斯加地区
                if lon < -140.0 { return offsets[0]; } // Alaska (UTC-9)
                else { return offsets[1]; } // UTC-8
            } else if lat < 25.0 {
                // 夏威夷附近
                return offsets[5]; // Hawaii (UTC-10)
            } else {
                // 大陆时区
                if lon < -120.0 {
                    return offsets[0]; // Pacific (UTC-8)
                } else if lon < -97.0 {
                    return offsets[1]; // Mountain (UTC-7)
                } else if lon < -82.5 {
                    return offsets[2]; // Central (UTC-6)
                } else if lon < -67.5 {
                    return offsets[3]; // Eastern (UTC-5)
                } else {
                    return offsets[4]; // Atlantic (UTC-4)
                }
            }
        }
        "AU" => {
            // 澳大利亚时区
            if lon < 120.0 {
                offsets[0] // UTC+8 (Western)
            } else if lon < 142.5 {
                offsets[1] // UTC+9:30 (Central)
            } else {
                offsets[2] // UTC+10 (Eastern)
            }
        }
        "RU" => {
            // 俄罗斯时区（综合经度和纬度）
            if lon < 30.0 {
                offsets[0] // UTC+1 (Kaliningrad)
            } else if lon < 60.0 {
                offsets[1] // UTC+2 (Moscow)
            } else if lon < 90.0 {
                if lat > 70.0 {
                    offsets[2] // UTC+3 (Norilsk)
                } else {
                    offsets[3] // UTC+4 (Samara)
                }
            } else if lon < 120.0 {
                offsets[4] // UTC+5 (Yekaterinburg)
            } else if lon < 150.0 {
                offsets[5] // UTC+6 (Omsk)
            } else {
                offsets[6] // UTC+7 (Vladivostok)
            }
        }
        "BR" => {
            // 巴西时区（基于经度和纬度）
            if lat < -20.0 && lon < -60.0 {
                offsets[2] // UTC-4 (Western Amazon)
            } else if lon < -45.0 {
                offsets[0] // UTC-3 (Brasilia)
            } else {
                offsets[1] // UTC-2 (Fernando de Noronha)
            }
        }
        "ID" => {
            // 印度尼西亚时区
            if lon < 120.0 {
                offsets[0] // UTC+7 (Western)
            } else if lon < 135.0 {
                offsets[1] // UTC+8 (Central)
            } else {
                offsets[2] // UTC+9 (Eastern)
            }
        }
        "KZ" => {
            // 哈萨克斯坦时区
            if lon < 60.0 {
                offsets[0] // UTC+5
            } else if lon < 75.0 {
                offsets[1] // UTC+6
            } else {
                offsets[2] // UTC+6
            }
        }
        _ => {
            // 默认返回第一个偏移
            offsets[0]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_us_timezone_by_location() {
        // 纽约（经度 -74.0，纬度 40.7）应该是 UTC-5 (Eastern)
        let offset = offset_from_location(-74.0, 40.7, Some("US"));
        assert_eq!(offset, -18000); // UTC-5
    }
    
    #[test]
    fn test_china_timezone() {
        // 中国统一使用 UTC+8
        let offset = offset_from_location(116.4, 39.9, Some("CN"));
        assert_eq!(offset, 28800);
        
        let offset = offset_from_location(87.6, 43.8, Some("CN"));
        assert_eq!(offset, 28800);
    }
    
    #[test]
    fn test_fallback_by_longitude() {
        // 没有国家代码时，根据经度推算
        let offset = offset_from_location(120.0, 30.0, None);
        assert_eq!(offset, 28800); // 120/15=8, 8*3600=28800
    }
}