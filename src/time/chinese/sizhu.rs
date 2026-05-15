// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 四柱计算模块
//! 基于公历年月日时计算干支年、月、日、时，支持时辰、初正刻等细节

const GAN: [&str; 10] = ["甲","乙","丙","丁","戊","己","庚","辛","壬","癸"];
const ZHI: [&str; 12] = ["子","丑","寅","卯","辰","巳","午","未","申","酉","戌","亥"];
const ANIMALS: [&str; 12] = ["鼠","牛","虎","兔","龙","蛇","马","羊","猴","鸡","狗","猪"];

/// 计算干支索引（0-59），基于公历年份（立春后才是新干支年）
/// 立春通常为 2 月 4 日左右，这里简化忽略立春（用户可自行判断）
pub fn get_ganzhi_index(year: i32) -> (usize, usize) {
    // 庚子年起点为 1840 年（甲子年 1864 年，但庚子为 40）
    let idx = (year - 4) % 60;
    let gan = idx % 10;
    let zhi = idx % 12;
    (gan as usize, zhi as usize)
}

pub fn get_animal(year: i32) -> usize {
    (year - 4) % 12 as usize
}

/// 时辰：每 2 小时一个，子时 23-1，正刻为 0,15,30,45 分钟，初正刻等
/// 返回 (时辰索引, 初/正, 刻, 分余)
pub fn get_shichen(hour: i32, minute: i32) -> (usize, usize, usize, usize) {
    // 时辰索引：子0 丑1 ... 亥11
    let mut h = hour;
    if h >= 23 { h -= 23; } else if h >= 1 { h = (h + 1) / 2; } else { h = 0; }
    let period = if (hour % 2 == 1 && minute >= 0) { 1 } else { 0 }; // 正
    let ke = minute / 15;
    let fen = minute % 15;
    (h as usize, period, ke as usize, fen as usize)
}