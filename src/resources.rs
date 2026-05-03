// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html

// 使用 include_bytes! 宏嵌入资源文件
use std::collections::HashMap;

// 嵌入文件为字节数组
const COUNTRIES_TZ_DATA: &[u8] = include_bytes!("../resources/countries_tz.txt");
const NTP_SERVERS_DATA: &[u8] = include_bytes!("../resources/ntp_servers.txt");
const TZ_OFFSETS_DATA: &[u8] = include_bytes!("../resources/tz_offsets.txt");

/// 将字节数据转换为字符串行数组
fn bytes_to_lines(data: &[u8]) -> Vec<String> {
    std::str::from_utf8(data)
        .unwrap_or("")
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect()
}

/// 解析 countries_tz.txt（格式: 国家代码,偏移1,偏移2,...）
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

/// 解析 tz_offsets.txt（格式: 名称,偏移秒）
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

/// 加载 NTP 服务器列表
pub fn load_ntp_servers() -> Vec<String> {
    let lines = bytes_to_lines(NTP_SERVERS_DATA);
    if lines.is_empty() {
        // 默认 fallback
        vec![
            "114.118.7.163:123".to_string(),
            "203.107.6.88:123".to_string(),
            "216.239.35.0:123".to_string(),
        ]
    } else {
        // 确保每行都有端口
        lines
            .into_iter()
            .map(|s| {
                if s.contains(':') {
                    s
                } else {
                    format!("{}:123", s)
                }
            })
            .collect()
    }
}

/// 尝试从外部文件读取（优先级更高），如果失败则返回 None
fn read_external_file_if_exists(filename: &str) -> Option<Vec<String>> {
    let mut path = std::env::current_exe().unwrap_or_default();
    path.pop();
    path.push("resources");
    path.push(filename);
    
    if let Ok(file) = std::fs::File::open(&path) {
        use std::io::{BufRead, BufReader};
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader
            .lines()
            .filter_map(|l| l.ok())
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .collect();
        if !lines.is_empty() {
            return Some(lines);
        }
    }
    None
}

/// 获取 countries_tz 数据（优先外部文件）
pub fn get_countries_tz_lines() -> Vec<String> {
    read_external_file_if_exists("countries_tz.txt")
        .unwrap_or_else(|| bytes_to_lines(COUNTRIES_TZ_DATA))
}

/// 获取 ntp_servers 数据（优先外部文件）
pub fn get_ntp_servers_lines() -> Vec<String> {
    read_external_file_if_exists("ntp_servers.txt")
        .unwrap_or_else(|| bytes_to_lines(NTP_SERVERS_DATA))
}

/// 获取 tz_offsets 数据（优先外部文件）
pub fn get_tz_offsets_lines() -> Vec<String> {
    read_external_file_if_exists("tz_offsets.txt")
        .unwrap_or_else(|| bytes_to_lines(TZ_OFFSETS_DATA))
}