// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 内嵌资源数据
pub const DST_RULES_DATA: &[u8] = include_bytes!("../../resources/dst_rules.txt");
const COUNTRIES_TZ_DATA: &[u8] = include_bytes!("../../resources/countries_tz.txt");
const NTP_SERVERS_DATA: &[u8] = include_bytes!("../../resources/ntp_servers.txt");
const TZ_OFFSETS_DATA: &[u8] = include_bytes!("../../resources/tz_offsets.txt");

use super::external::read_external_file;

pub fn bytes_to_lines(data: &[u8]) -> Vec<String> {
    std::str::from_utf8(data)
        .unwrap_or("")
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect()
}

pub fn get_countries_tz_lines() -> Vec<String> {
    read_external_file("countries_tz.txt")
        .unwrap_or_else(|| bytes_to_lines(COUNTRIES_TZ_DATA))
}

pub fn get_ntp_servers_lines() -> Vec<String> {
    read_external_file("ntp_servers.txt")
        .unwrap_or_else(|| bytes_to_lines(NTP_SERVERS_DATA))
}

pub fn get_tz_offsets_lines() -> Vec<String> {
    read_external_file("tz_offsets.txt")
        .unwrap_or_else(|| bytes_to_lines(TZ_OFFSETS_DATA))
}

// 添加获取 DST 规则配置的函数

pub fn get_dst_rules_lines() -> Vec<String> {
    read_external_file("dst_rules.txt")
        .unwrap_or_else(|| bytes_to_lines(DST_RULES_DATA))
}
