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

use crate::time::git::get_system_time_utc;
use crate::time::git::ntp::get_cached_utc_time;
use crate::time::config::is_auto_sync_enabled;

/// 获取经校准的本地时间
/// 
/// 如果自动同步启用且 NTP 可用，返回 NTP 时间（更精确）
/// 否则返回系统本地时间
pub fn get_calibrated_local_time() -> (u64, i32) {
    let (sys_sec, sys_us) = get_system_time_utc();
    
    // 如果自动同步启用且 NTP 已有缓存，直接使用 NTP 时间
    if is_auto_sync_enabled() {
        if let Some((ntp_sec, ntp_us)) = get_cached_utc_time() {
            // 优先使用 NTP 时间（更精确）
            return (ntp_sec, ntp_us);
        }
    }
    
    // 降级：使用系统时间
    (sys_sec, sys_us)
}

/// 获取本地时间（不经过 NTP 校准）
pub fn get_raw_local_time() -> (u64, i32) {
    get_system_time_utc()
}

/// 检查系统时间是否与 NTP 同步（误差在阈值内）
pub fn is_time_synced() -> bool {
    if !is_auto_sync_enabled() {
        return false;
    }
    
    if let Some((ntp_sec, _)) = get_cached_utc_time() {
        let (sys_sec, _) = get_system_time_utc();
        let diff = if ntp_sec > sys_sec {
            ntp_sec - sys_sec
        } else {
            sys_sec - ntp_sec
        };
        diff <= crate::time::defines::SYNC_THRESHOLD_SECONDS
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_calibrated_local_time_returns_tuple() {
        let (sec, us) = get_calibrated_local_time();
        // 时间戳应该在 1600000000 到 2000000000 之间（大约 2020-2033 年）
        assert!(sec > 1600000000 && sec < 2000000000);
        assert!(us < 1_000_000);
    }
    
    #[test]
    fn test_get_raw_local_time() {
        let (sec, us) = get_raw_local_time();
        assert!(us < 1_000_000);
        assert!(sec > 1600000000);
    }
}