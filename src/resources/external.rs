// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 外部文件读取

use std::path::PathBuf;

fn get_dll_directory() -> PathBuf {
    let exe_path = std::env::current_exe().unwrap_or_default();
    let mut dll_dir = exe_path;
    dll_dir.pop();
    if dll_dir.exists() {
        return dll_dir;
    }
    std::env::current_dir().unwrap_or_default()
}

pub fn read_external_file(filename: &str) -> Option<Vec<String>> {
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