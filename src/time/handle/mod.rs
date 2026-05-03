// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


// 时间处理层：管理本地时间与 NTP 的偏移
pub mod compare;
pub mod formatting;

use crate::time::git::{get_system_time_utc, monotonic_secs};
use crate::time::git::ntp::get_cached_utc_time;
use crate::time::config::is_auto_sync_enabled;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};

static OFFSET: AtomicI64 = AtomicI64::new(0);
static LAST_SYNC_MONO_MS: AtomicU64 = AtomicU64::new(0);

fn update_offset(ntp_sec: u64) {
    let (sys_sec, _) = get_system_time_utc();
    let off = ntp_sec as i64 - sys_sec as i64;
    OFFSET.store(off, Ordering::Release);
    LAST_SYNC_MONO_MS.store((monotonic_secs() * 1000.0) as u64, Ordering::Release);
}

fn get_valid_offset() -> i64 {
    if !is_auto_sync_enabled() { return 0; }
    let last = LAST_SYNC_MONO_MS.load(Ordering::Acquire);
    let now = (monotonic_secs() * 1000.0) as u64;
    if last == 0 || now - last >= 3_600_000 { return 0; }
    OFFSET.load(Ordering::Acquire)
}

pub fn get_calibrated_local_time() -> (u64, i32) {
    let (sys_sec, sys_us) = get_system_time_utc();
    let last = LAST_SYNC_MONO_MS.load(Ordering::Acquire);
    let now = (monotonic_secs() * 1000.0) as u64;
    if is_auto_sync_enabled() && (last == 0 || now - last >= 3_600_000) {
        if let Some((ntp_sec, _)) = get_cached_utc_time() {
            update_offset(ntp_sec);
        }
    }
    let off = get_valid_offset();
    let cal_sec = if off >= 0 {
        sys_sec.saturating_add(off as u64)
    } else {
        sys_sec.saturating_sub((-off) as u64)
    };
    (cal_sec, sys_us)
}