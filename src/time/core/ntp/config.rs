// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html

// 加载 NTP 服务器 IP 列表
pub fn load_ntp_servers() -> Vec<String> {
    crate::resources::load_ntp_servers()
}
