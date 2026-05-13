// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


// NTP 模块对外接口
mod config;
mod fetcher;
mod updater;

use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::sync::Once;

static CACHED_SEC: AtomicU64 = AtomicU64::new(0);
static CACHED_US: AtomicU64 = AtomicU64::new(0);
static AVAILABLE: AtomicBool = AtomicBool::new(false);
static START_ONCE: Once = Once::new();
static SHUTDOWN: AtomicBool = AtomicBool::new(false);  //退出标志

pub fn is_started() -> bool {
    START_ONCE.is_completed()
}

pub fn get_cached_utc_time() -> Option<(u64, i32)> {
    START_ONCE.call_once(|| { 
        std::thread::spawn(|| {
            updater::ntp_updater_with_shutdown();
        });
    });
    
    if AVAILABLE.load(Ordering::Acquire) {
        let sec = CACHED_SEC.load(Ordering::Acquire);
        let us = CACHED_US.load(Ordering::Acquire) as i32;
        if sec > 0 { Some((sec, us)) } else { None }
    } else { None }
}

pub fn is_ntp_available() -> bool { 
    AVAILABLE.load(Ordering::Acquire) 
}

pub fn force_resync() -> bool {
    if let Some((sec, us, _)) = fetcher::fetch_best_ntp() {
        CACHED_SEC.store(sec, Ordering::Release);
        CACHED_US.store(us.max(0) as u64, Ordering::Release);
        AVAILABLE.store(true, Ordering::Release);
        true
    } else { false }
}

pub(crate) fn update_cache(sec: u64, us: i32) {
    CACHED_SEC.store(sec, Ordering::Release);
    CACHED_US.store(us as u64, Ordering::Release);
    AVAILABLE.store(true, Ordering::Release);
}

/// 关闭 NTP 后台线程（DLL 卸载时调用）
pub fn shutdown() {
    SHUTDOWN.store(true, Ordering::Release);
}