// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! DST 夏令时支持

mod rule;
mod calculator;
mod system;
mod provider;

pub use rule::*;
pub use calculator::{is_dst_by_rules, get_dst_offset_by_rules};
pub use provider::*;
pub use system::{is_system_dst, get_system_dst_offset, get_system_base_bias, get_system_current_bias};

// 默认导出 provider 模块的接口，方便外部调用
pub use provider::is_dst;
pub use provider::get_dst_offset;
// 确保 rule 模块中的 get_rule 被导出
pub use rule::get_rule;
