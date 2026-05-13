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
    loop {
        let interval = Duration::from_secs(crate::time::defines::get_ntp_update_interval());

        if SHUTDOWN.load(Ordering::Acquire) {
            break;
        }

        if let Some((sec, us, _)) = super::fetcher::fetch_best_ntp() {
            super::update_cache(sec, us);
        }

        let mut slept = Duration::from_secs(0);
        let step = Duration::from_millis(100);

        while slept < interval {
            if SHUTDOWN.load(Ordering::Acquire) {
                break;
            }
            thread::sleep(step);
            slept += step;
        }
    }
}