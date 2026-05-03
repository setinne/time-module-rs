// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html

// 时间模块入口，对外暴露唯一 C 接口
#[cfg(target_env = "gnu")]

pub mod error;
pub mod resources;
pub mod time;
pub mod time_api;

pub use time_api::*;