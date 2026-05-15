// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 外部文件读取

use std::path::PathBuf;

#[cfg(windows)]
fn get_dll_directory() -> PathBuf {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    
    // 使用在 DllMain 中保存的 DLL 句柄
    let h_module = crate::get_dll_hinstance();
    
    if !h_module.is_null() {
        unsafe {
            let mut path_buf: Vec<u16> = vec![0u16; 32768];
            let len = GetModuleFileNameW(
                h_module,
                path_buf.as_mut_ptr(),
                path_buf.len() as u32
            );
            
            if len > 0 {
                path_buf.truncate(len as usize);
                let path = OsString::from_wide(&path_buf);
                let mut path = PathBuf::from(path);
                path.pop(); // 移除 DLL 文件名，保留目录
                if path.exists() {
                    return path;
                }
            }
        }
    }
    
    // 回退方案
    let exe_path = std::env::current_exe().unwrap_or_default();
    let mut dll_dir = exe_path;
    dll_dir.pop();
    if dll_dir.exists() {
        return dll_dir;
    }
    std::env::current_dir().unwrap_or_default()
}

#[cfg(not(windows))]
fn get_dll_directory() -> PathBuf {
    let exe_path = std::env::current_exe().unwrap_or_default();
    let mut dll_dir = exe_path;
    dll_dir.pop();
    if dll_dir.exists() {
        return dll_dir;
    }
    std::env::current_dir().unwrap_or_default()
}

// Windows API 声明
#[cfg(windows)]
extern "system" {
    fn GetModuleFileNameW(
        hModule: *const std::ffi::c_void,
        lpFilename: *mut u16,
        nSize: u32,
    ) -> u32;
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

    // 回退到当前目录
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