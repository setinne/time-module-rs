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

use crate::time::defines::{
    NTP_INITIAL_RETRY_INTERVAL_SEC,
    NTP_MAX_RETRY_INTERVAL_SEC,
    NTP_RETRY_MULTIPLIER,
    get_ntp_update_interval,
};

static RETRY_COUNT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
static NEXT_INTERVAL_SEC: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(NTP_INITIAL_RETRY_INTERVAL_SEC);

pub fn ntp_updater_with_shutdown() {
    loop {
        if super::SHUTDOWN.load(Ordering::Acquire) {
            break;
        }

        let success = if let Some((sec, us, _)) = super::fetcher::fetch_best_ntp() {
            super::update_cache(sec, us);
            // 成功后重置退避
            RETRY_COUNT.store(0, Ordering::Release);
            NEXT_INTERVAL_SEC.store(get_ntp_update_interval(), Ordering::Release);
            true
        } else {
            false
        };

        if !success {
            let count = RETRY_COUNT.fetch_add(1, Ordering::Relaxed);
            let new_interval = NTP_INITIAL_RETRY_INTERVAL_SEC * NTP_RETRY_MULTIPLIER.pow(count as u32);
            let new_interval = new_interval.min(NTP_MAX_RETRY_INTERVAL_SEC);
            NEXT_INTERVAL_SEC.store(new_interval, Ordering::Release);
        }

        let interval = Duration::from_secs(NEXT_INTERVAL_SEC.load(Ordering::Acquire));
        let step = Duration::from_millis(100);
        let mut slept = Duration::from_secs(0);
        while slept < interval {
            if super::SHUTDOWN.load(Ordering::Acquire) {
                break;
            }
            thread::sleep(step);
            slept += step;
        }
    }
}
