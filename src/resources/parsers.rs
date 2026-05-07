// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html



//! 资源文件解析

use std::collections::HashMap;
use super::embedded::bytes_to_lines;

const COUNTRIES_TZ_DATA: &[u8] = include_bytes!("../../resources/countries_tz.txt");
const TZ_OFFSETS_DATA: &[u8] = include_bytes!("../../resources/tz_offsets.txt");

pub fn parse_countries_tz() -> HashMap<String, Vec<i32>> {
    let lines = bytes_to_lines(COUNTRIES_TZ_DATA);
    let mut map = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 2 {
            continue;
        }
        let code = parts[0].to_uppercase();
        let offsets: Vec<i32> = parts[1..]
            .iter()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        if !offsets.is_empty() {
            map.insert(code, offsets);
        }
    }
    map
}

pub fn parse_tz_offsets() -> HashMap<String, i32> {
    let lines = bytes_to_lines(TZ_OFFSETS_DATA);
    let mut map = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 2 {
            continue;
        }
        let name = parts[0].trim().to_string();
        if let Ok(offset) = parts[1].trim().parse::<i32>() {
            map.insert(name, offset);
        }
    }
    map
}

pub fn load_ntp_servers() -> Vec<String> {
    let servers = crate::resources::embedded::get_ntp_servers_lines();
    if servers.is_empty() {
        vec![
            "114.118.7.163:123".to_string(),
            "203.107.6.88:123".to_string(),
            "216.239.35.0:123".to_string(),
        ]
    } else {
        servers.into_iter().map(|s| {
            if s.contains(':') { s } else { format!("{}:123", s) }
        }).collect()
    }
}