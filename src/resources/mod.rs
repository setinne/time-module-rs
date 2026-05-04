// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 资源文件管理（内嵌 + 外部覆盖）

mod embedded;
mod external;
mod parsers;

pub use embedded::{get_countries_tz_lines, get_ntp_servers_lines, get_tz_offsets_lines};
pub use parsers::{parse_countries_tz, parse_tz_offsets, load_ntp_servers};