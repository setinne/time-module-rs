// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 模块入口，对外导出中国传统时间相关的 API
//! 包括农历、干支、时辰等计算，基于公历年月日时输入，输出对应的中国传统时间信息

pub mod astronomy;
pub mod lunar;
pub mod sizhu;
pub mod era;

use crate::time::calc::FullTime;
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn api_GetChineseDateTime(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32) -> *const c_char {
    let jd = astronomy::gregorian_to_jd(year, month, day, hour as f64 + minute as f64 / 60.0 + second as f64 / 3600.0);
    // 调用 lunar::jd_to_lunar 等
    let (lunar_year, lunar_month, lunar_day, is_leap) = lunar::jd_to_lunar(jd);
    let (gan_idx, zhi_idx) = sizhu::get_ganzhi_index(lunar_year);
    let gan = sizhu::GAN[gan_idx];
    let zhi = sizhu::ZHI[zhi_idx];
    let animal = sizhu::ANIMALS[sizhu::get_animal(lunar_year)];
    let era_name = era::get_era_name(year).unwrap_or("");
    let (shichen_idx, period, ke, fen) = sizhu::get_shichen(hour, minute);
    let shichen = sizhu::ZHI[shichen_idx];
    let period_str = if period == 0 { "初" } else { "正" };
    let output = format!("{}{}年（{}{}{}年）{}{}月{}日{}时{}刻{}分",
        era_name, year, gan, zhi, animal,
        if is_leap { "闰" } else { "" }, lunar_month, lunar_day,
        shichen, period_str, ke, fen);
    CString::new(output).unwrap().into_raw()
}