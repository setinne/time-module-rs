// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


// 后台线程定期更新 NTP 缓存
use std::time::Duration;
use std::thread;
use std::sync::atomic::Ordering;

use super::SHUTDOWN;

pub fn ntp_updater_with_shutdown() {
    let interval = Duration::from_secs(crate::time::defines::NTP_UPDATE_INTERVAL_SEC);
    
    loop {
        // 检查退出标志
        if SHUTDOWN.load(Ordering::Acquire) {
            break;
        }
        
        if let Some((sec, us, _)) = super::fetcher::fetch_best_ntp() {
            super::update_cache(sec, us);
        }
        
        // 分段睡眠，以便快速响应退出信号
        let mut slept = Duration::from_secs(0);
        let step = Duration::from_millis(100);  // 100ms 检查一次
        
        while slept < interval {
            if SHUTDOWN.load(Ordering::Acquire) {
                break;
            }
            thread::sleep(step);
            slept += step;
        }
    }
}