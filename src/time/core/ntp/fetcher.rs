// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html

// 精确 NTP 协议实现：往返延迟校正
use std::net::{UdpSocket, ToSocketAddrs};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const NTP_UNIX_OFFSET: u64 = 2208988800;
const NTP_FRACTION_MAX: f64 = 4294967296.0;

fn query_single(server: &str) -> Option<(u64, i32, f64)> {
    let sock = UdpSocket::bind("0.0.0.0:0").ok()?;
    let _ = sock.set_read_timeout(Some(Duration::from_millis(3000)));
    let _ = sock.set_write_timeout(Some(Duration::from_millis(3000)));
    let mut packet = [0u8; 48];
    packet[0] = 0x1B;
    let addrs = server.to_socket_addrs().ok()?;
    for addr in addrs {
        let t1 = SystemTime::now().duration_since(UNIX_EPOCH).ok()?;
        let t1_sec = t1.as_secs();
        let t1_frac = t1.subsec_nanos() as f64 / 1_000_000_000.0;
        if sock.send_to(&packet, addr).is_err() { continue; }
        let mut buf = [0u8; 48];
        match sock.recv_from(&mut buf) {
            Ok((size, _)) if size >= 48 => {
                let t4 = SystemTime::now().duration_since(UNIX_EPOCH).ok()?;
                let t4_sec = t4.as_secs();
                let t4_frac = t4.subsec_nanos() as f64 / 1_000_000_000.0;
                let ntp_sec = u32::from_be_bytes([buf[40], buf[41], buf[42], buf[43]]) as u64;
                let ntp_frac = u32::from_be_bytes([buf[44], buf[45], buf[46], buf[47]]) as f64 / NTP_FRACTION_MAX;
                let t2_sec = ntp_sec - NTP_UNIX_OFFSET;
                let t2_frac = ntp_frac;
                let theta = ((t2_sec as f64 + t2_frac) - (t1_sec as f64 + t1_frac)
                            + (t4_sec as f64 + t4_frac) - (t2_sec as f64 + t2_frac)) / 2.0;
                let offset_sec = theta as u64;
                let offset_nsec = (theta.fract() * 1_000_000_000.0) as u32;
                let corr_sec = t4_sec.saturating_add(offset_sec);
                let corr_nsec = (t4_frac * 1_000_000_000.0) as u32 + offset_nsec;
                let corr_us = (corr_nsec / 1000) as i32;
                return Some((corr_sec, corr_us, theta.abs()));
            }
            _ => continue,
        }
    }
    None
}

pub fn fetch_best_ntp() -> Option<(u64, i32, f64)> {
    let servers = super::config::load_ntp_servers();
    for s in servers {
        if let Some(res) = query_single(&s) {
            return Some(res);
        }
    }
    None
}