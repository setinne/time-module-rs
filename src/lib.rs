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

// 保存 DLL 句柄
#[cfg(windows)]
static mut DLL_HINSTANCE: *mut std::ffi::c_void = std::ptr::null_mut();

#[cfg(windows)]
pub fn get_dll_hinstance() -> *mut std::ffi::c_void {
    unsafe { DLL_HINSTANCE }
}

// Windows DLL 入口点
#[cfg(windows)]
#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(hinstDLL: *mut std::ffi::c_void, fdwReason: u32, _lpvReserved: *mut std::ffi::c_void) -> u32 {
    match fdwReason {
        0 => {  // DLL_PROCESS_DETACH
            // 不执行任何操作，让调用者负责清理
            // 如果调用者未调用 api_Shutdown，线程可能仍在运行，但这是用户的责任
}
        1 => {  // DLL_PROCESS_ATTACH
            // DLL 加载时的初始化
            unsafe { DLL_HINSTANCE = hinstDLL; }
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