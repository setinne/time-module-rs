// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


// 使用 include_bytes! 宏嵌入资源文件
use std::collections::HashMap;
use std::path::PathBuf;

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

/// 获取 DLL 所在目录（更可靠的方法）
fn get_dll_directory() -> PathBuf {
    // 方法1：从当前可执行文件路径获取
    let exe_path = std::env::current_exe().unwrap_or_default();
    let mut dll_dir = exe_path;
    dll_dir.pop();
    
    // 验证目录是否存在
    if dll_dir.exists() {
        return dll_dir;
    }
    
    // 方法2：使用当前工作目录作为 fallback
    std::env::current_dir().unwrap_or_default()
}

/// 尝试从外部文件读取（优先级更高），如果失败则返回 None
fn read_external_file_if_exists(filename: &str) -> Option<Vec<String>> {
    // 优先尝试 DLL 所在目录下的 resources 文件夹
    let mut path = get_dll_directory();
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
    
    // 降级：尝试当前工作目录下的 resources 文件夹
    let mut path = std::env::current_dir().unwrap_or_default();
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

// 其余函数保持不变...
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

pub fn get_countries_tz_lines() -> Vec<String> {
    read_external_file_if_exists("countries_tz.txt")
        .unwrap_or_else(|| bytes_to_lines(COUNTRIES_TZ_DATA))
}

pub fn get_ntp_servers_lines() -> Vec<String> {
    read_external_file_if_exists("ntp_servers.txt")
        .unwrap_or_else(|| bytes_to_lines(NTP_SERVERS_DATA))
}

pub fn get_tz_offsets_lines() -> Vec<String> {
    read_external_file_if_exists("tz_offsets.txt")
        .unwrap_or_else(|| bytes_to_lines(TZ_OFFSETS_DATA))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_countries_tz() {
        let map = parse_countries_tz();
        assert_eq!(map.get("CN"), Some(&vec![28800]));
        assert!(map.get("US").unwrap().len() >= 5);
        assert!(map.get("RU").unwrap().len() >= 4);
    }
    
    #[test]
    fn test_load_ntp_servers() {
        let servers = load_ntp_servers();
        assert!(!servers.is_empty());
        // 检查是否都有端口
        for server in servers {
            assert!(server.contains(':'));
        }
    }
}