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

pub fn ntp_updater() {
    loop {
        if let Some((sec, us, _)) = super::fetcher::fetch_best_ntp() {
            super::update_cache(sec, us);
        }
        thread::sleep(Duration::from_secs(crate::time::defines::NTP_UPDATE_INTERVAL_SEC));
    }
}