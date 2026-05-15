// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 中国农历相关天文计算
//! 包括太阳和月亮的黄经计算、朔望月计算等核心

use std::f64::consts::PI;

/// 转化为弧度
pub fn to_radians(deg: f64) -> f64 {
    deg * PI / 180.0
}

/// 角度标准化到 [0, 360)
pub fn norm_angle(deg: f64) -> f64 {
    let mut d = deg % 360.0;
    if d < 0.0 { d += 360.0; }
    d
}

/// 计算儒略日（JD）
/// JD = 367*y - floor(7*(y+floor((m+9)/12))/4) + floor(275*m/9) + d + 1721013.5
pub fn gregorian_to_jd(year: i32, month: i32, day: i32, hour: f64) -> f64 {
    let a = (14 - month) / 12;
    let y = year + 4800 - a;
    let m = month + 12 * a - 3;
    let jd = day as f64 + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045;
    jd + hour / 24.0
}

/// 儒略日转公历（仅用于验证）
pub fn jd_to_gregorian(jd: f64) -> (i32, i32, i32, f64) {
    let jd = jd + 0.5;
    let z = jd.floor() as i64;
    let f = jd - z as f64;
    let a = if z < 2299161 { z } else {
        let alpha = ((z as f64 - 1867216.25) / 36524.25).floor() as i64;
        z + 1 + alpha - alpha / 4
    };
    let b = a + 1524;
    let c = ((b as f64 - 122.1) / 365.25).floor() as i64;
    let d = (365.25 * c as f64).floor() as i64;
    let e = ((b - d) as f64 / 30.6001).floor() as i64;
    let day = (b - d - (30.6001 * e as f64).floor() as i64) as i32;
    let month = if e < 14 { e - 1 } else { e - 13 };
    let year = c - 4715;
    (year as i32, month as i32, day as i32, f * 24.0)
}

/// 地球轨道离心率
fn earth_eccentricity(jd: f64) -> f64 {
    let t = (jd - 2451545.0) / 36525.0; // 儒略世纪
    0.016708634 - 0.000042037 * t - 0.0000001267 * t * t
}

/// 太阳平黄经（几何平黄经）
fn sun_mean_longitude(jd: f64) -> f64 {
    let t = (jd - 2451545.0) / 36525.0;
    let l0 = 280.46646 + 36000.76983 * t + 0.0003032 * t * t;
    norm_angle(l0)
}

/// 太阳平近点角
fn sun_mean_anomaly(jd: f64) -> f64 {
    let t = (jd - 2451545.0) / 36525.0;
    let m = 357.52911 + 35999.05029 * t - 0.0001537 * t * t;
    norm_angle(m)
}

/// 太阳真黄经（中心差修正）
pub fn sun_ecliptic_longitude(jd: f64) -> f64 {
    let m = sun_mean_anomaly(jd).to_radians();
    let c = (1.914602 - 0.004817 * (jd - 2451545.0) / 36525.0) * m.sin()
        + 0.019993 * (2.0 * m).sin()
        + 0.000289 * (3.0 * m).sin();
    let l = sun_mean_longitude(jd) + c;
    norm_angle(l)
}

/// 月亮平黄经
fn moon_mean_longitude(jd: f64) -> f64 {
    let t = (jd - 2451545.0) / 36525.0;
    let l = 218.3164477 + 481267.88123421 * t - 0.0015786 * t * t + t * t * t / 538841.0 - t * t * t * t / 65194000.0;
    norm_angle(l)
}

/// 月亮平近地点角
fn moon_mean_perigee(jd: f64) -> f64 {
    let t = (jd - 2451545.0) / 36525.0;
    let p = 83.3532465 + 4069.0137287 * t - 0.0103200 * t * t - 0.0000124 * t * t * t;
    norm_angle(p)
}

/// 月亮平升交点
fn moon_mean_node(jd: f64) -> f64 {
    let t = (jd - 2451545.0) / 36525.0;
    let n = 125.0445479 - 1934.1362891 * t + 0.0020754 * t * t + t * t * t / 467441.0;
    norm_angle(n)
}

/// 月亮真黄经（加上若干周期项）
pub fn moon_ecliptic_longitude(jd: f64) -> f64 {
    let t = (jd - 2451545.0) / 36525.0;
    let l = moon_mean_longitude(jd);
    let m = sun_mean_anomaly(jd);
    let mp = moon_mean_perigee(jd);
    let f = moon_mean_node(jd);
    let d = l - sun_mean_longitude(jd); // 日月距角
    
    let d_rad = d.to_radians();
    let m_rad = m.to_radians();
    let mp_rad = mp.to_radians();
    let f_rad = f.to_radians();
    
    // 主要周期项（仅取最大几项）
    let mut dl = 0.0;
    dl += -1.274 * (mp_rad - 2.0 * d_rad).sin();
    dl += +0.658 * (2.0 * d_rad).sin();
    dl += -0.186 * (m_rad).sin();
    dl += -0.059 * (2.0 * mp_rad - 2.0 * d_rad).sin();
    dl += -0.057 * (2.0 * d_rad - m_rad).sin();
    dl += +0.053 * (2.0 * d_rad + m_rad).sin();
    dl += +0.046 * (2.0 * mp_rad).sin();
    dl += +0.041 * (mp_rad).sin();
    dl += -0.035 * (2.0 * d_rad - mp_rad).sin();
    dl += -0.031 * (m_rad - 2.0 * d_rad).sin();
    dl += -0.015 * (2.0 * d_rad + mp_rad - m_rad).sin();
    let lambda = l + dl;
    norm_angle(lambda)
}

/// 日月黄经差（月亮减太阳）
pub fn moon_sun_ecliptic_diff(jd: f64) -> f64 {
    let diff = moon_ecliptic_longitude(jd) - sun_ecliptic_longitude(jd);
    norm_angle(diff)
}

/// 给定儒略日，求最近的朔日（农历初一）时刻，返回儒略日
pub fn previous_new_moon(jd: f64) -> f64 {
    let k = ((jd - 2451550.09765) / 29.530588853).floor();
    let mut jd0 = 2451550.09765 + k * 29.530588853;
    let mut diff = moon_sun_ecliptic_diff(jd0);
    // 修正使 diff 接近 0
    while diff > 0.5 { jd0 -= 1.0; diff = moon_sun_ecliptic_diff(jd0); }
    while diff < -0.5 { jd0 += 1.0; diff = moon_sun_ecliptic_diff(jd0); }
    // 迭代逼近
    for _ in 0..10 {
        let f = moon_sun_ecliptic_diff(jd0);
        let df = (moon_sun_ecliptic_diff(jd0 + 0.001) - f) / 0.001;
        let delta = -f / df;
        if delta.abs() < 0.0001 { break; }
        jd0 += delta;
    }
    jd0
}