// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html

// 底层时间供应层
pub mod local;
pub mod provider;
pub mod ntp;

pub use local::{get_system_time_us, monotonic_secs};
// 兼容旧名称
pub use local::get_system_time_us as get_system_time_utc;
pub use ntp::get_cached_utc_time;
pub use provider::get_full_time_data;