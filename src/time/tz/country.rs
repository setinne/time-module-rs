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

use crate::error::TimeErrorCode;

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
/// - `Ok(i32)`: 时区偏移秒数（东正西负）
/// - `Err(TimeErrorCode::CountryNotFound)`: 国家代码不存在
pub fn offset_from_location(
    longitude: f64, 
    latitude: f64, 
    country: Option<&str>
) -> Result<i32, TimeErrorCode> {
    let lon = longitude.clamp(-180.0, 180.0);
    let lat = latitude.clamp(-90.0, 90.0);
    
    if let Some(code) = country {
        let code_upper = code.to_uppercase();
        let map = get_country_map();
        let offsets = map.get(&code_upper)
            .ok_or(TimeErrorCode::CountryNotFound)?;
        
        if offsets.len() == 1 {
            return Ok(offsets[0]);
        }
        
        // 多个偏移时，尝试按经纬度选择
        if offsets.len() > 1 {
            return Ok(select_offset_by_location(&code_upper, offsets, lon, lat));
        }
    }
    
    // 如果没有提供国家或国家只有一个偏移，退回到经纬度计算
    Ok(((lon / 15.0).round() * 3600.0) as i32)
}

/// 根据经纬度和国家代码选择合适的时区偏移
fn select_offset_by_location(country: &str, offsets: &[i32], lon: f64, lat: f64) -> i32 {
    match country {
        "US" | "CA" => {
            if lat > 60.0 {
                // 阿拉斯加和加拿大北部
                if lon < -140.0 { return offsets[0]; }
                else { return offsets[1]; }
            } else if lat < 25.0 {
                // 夏威夷、关岛等
                return offsets[5];
            } else {
                // 美国本土和加拿大南部
                if lon < -120.0 {
                    return offsets[0];  // 太平洋
                } else if lon < -97.0 {
                    return offsets[1];  // 山地
                } else if lon < -82.5 {
                    return offsets[2];  // 中部
                } else if lon < -67.5 {
                    return offsets[3];  // 东部
                } else {
                    return offsets[4];  // 大西洋
                }
            }
        }
        "AU" => {
            if lon < 120.0 {
                offsets[0]  // 西澳
            } else if lon < 142.5 {
                offsets[1]  // 中澳
            } else {
                offsets[2]  // 东澳
            }
        }
        "RU" => {
            // 俄罗斯跨多个时区，按经度细分
            if lon < 30.0 {
                offsets[0]  // 加里宁格勒
            } else if lon < 60.0 {
                offsets[1]  // 莫斯科
            } else if lon < 90.0 {
                if lat > 70.0 {
                    offsets[2]  // 西伯利亚西部
                } else {
                    offsets[3]  // 西伯利亚中部
                }
            } else if lon < 120.0 {
                offsets[4]  // 西伯利亚东部
            } else if lon < 150.0 {
                offsets[5]  // 远东
            } else {
                offsets[6]  // 勘察加
            }
        }
        "BR" => {
            if lat < -20.0 && lon < -60.0 {
                offsets[2]  // 亚马逊
            } else if lon < -45.0 {
                offsets[0]  // 巴西利亚时间
            } else {
                offsets[1]  // 东部
            }
        }
        "ID" => {
            if lon < 120.0 {
                offsets[0]  // 西印尼
            } else if lon < 135.0 {
                offsets[1]  // 中印尼
            } else {
                offsets[2]  // 东印尼
            }
        }
        "KZ" => {
            if lon < 60.0 {
                offsets[0]  // 西哈萨克
            } else if lon < 75.0 {
                offsets[1]  // 中哈萨克
            } else {
                offsets[2]  // 东哈萨克
            }
        }
        _ => {
            // 对于未明确处理的多时区国家，退回到经纬度计算
            ((lon / 15.0).round() * 3600.0) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_us_timezone_by_location() {
        let offset = offset_from_location(-74.0, 40.7, Some("US")).unwrap();
        assert_eq!(offset, -18000);
    }
    
    #[test]
    fn test_china_timezone() {
        let offset = offset_from_location(116.4, 39.9, Some("CN")).unwrap();
        assert_eq!(offset, 28800);
        
        let offset = offset_from_location(87.6, 43.8, Some("CN")).unwrap();
        assert_eq!(offset, 28800);
    }
    
    #[test]
    fn test_fallback_by_longitude() {
        let offset = offset_from_location(120.0, 30.0, None).unwrap();
        assert_eq!(offset, 28800);
    }
    
    #[test]
    fn test_country_not_found() {
        let result = offset_from_location(0.0, 0.0, Some("XYZ"));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err() as i32, 8); // CountryNotFound
    }
}