// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


// 时间模块入口，对外暴露唯一 C 接口

#![warn(deprecated)]

pub mod error;
pub mod error_string;
pub mod resources;
pub mod time;
pub mod time_api;

pub use time_api::*;

// Windows DLL 入口点
#[cfg(windows)]
#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(_hinstDLL: *mut std::ffi::c_void, fdwReason: u32, _lpvReserved: *mut std::ffi::c_void) -> u32 {
    match fdwReason {
        0 => {  // DLL_PROCESS_DETACH
            // DLL 卸载时，通知后台线程退出
            crate::time::core::ntp::shutdown();
        }
        1 => {  // DLL_PROCESS_ATTACH
            // DLL 加载时，可以做一些初始化（可选）
        }
        _ => {}
    }
    1  // 返回 TRUE 表示成功
}

// 确保时间类型大小正确
const _ASSERT_SIZE: () = {
    assert!(std::mem::size_of::<u64>() == 8);
    assert!(std::mem::size_of::<i64>() == 8);
};
