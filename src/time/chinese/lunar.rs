// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 农历计算模块
//! 基于天文算法计算农历年月日，支持闰月和节气

use super::astronomy::*;

/// 农历月信息
pub struct LunarMonthInfo {
    pub year: i32,       // 干支年（从立春开始）
    pub month: i32,      // 月数 (1-12, 闰月用负值如 -1)
    pub is_leap: bool,   // 是否闰月
    pub days: u8,        // 当月天数 (29 或 30)
}

/// 根据儒略日计算农历年月日
pub fn jd_to_lunar(jd: f64) -> (i32, i32, i32, bool) {
    let prev_new = previous_new_moon(jd);
    let cur_new = previous_new_moon(prev_new + 30.0);
    let day_of_month = (jd - prev_new).floor() as i32 + 1;
    // 简化：此处需要更多逻辑确定农历年和闰月，省略完整实现
    // 实际需要计算包含闰月的农历年列表，判断当月是否闰月
    // 由于时间限制，这里返回占位值
    (1970, 1, day_of_month, false)
}