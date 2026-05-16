// Copyright (c) 2026 Setinne
// SPDX-License-Identifier: LGPL-2.1-only
//
// This file is part of the TIME_MODULE project.
// Licensed under the GNU Lesser General Public License v2.1.
// You may obtain a copy of the License at:
//     https://www.gnu.org/licenses/lgpl-2.1.html


//! 标准 NTP 协议实现（RFC 5905），增强验证

use std::net::{UdpSocket, ToSocketAddrs};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const NTP_UNIX_OFFSET: u64 = 2208988800;
const NTP_FRACTION_MAX: f64 = 4294967296.0;

/// NTP 包结构（48 字节）
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct NtpPacket {
    data: [u8; 48],
}

impl NtpPacket {
    fn new_client() -> Self {
        let mut packet = NtpPacket { data: [0u8; 48] };
        // LI=0, VN=4, Mode=3 (client)
        packet.data[0] = 0x23;  // 修复：正确的 NTP 版本和模式
        packet
    }
    
    fn li(&self) -> u8 {
        (self.data[0] >> 6) & 0x03
    }
    
    fn vn(&self) -> u8 {
        (self.data[0] >> 3) & 0x07
    }
    
    fn mode(&self) -> u8 {
        self.data[0] & 0x07
    }
    
    fn stratum(&self) -> u8 {
        self.data[1]
    }
    
    fn is_valid_response(&self) -> bool {
        // 检查版本号和模式
        self.vn() == 4 && self.mode() == 4 && 
        // 检查闰秒标志（3 表示时钟未同步）
        self.li() != 3 &&
        // 检查层数（0 表示未同步）
        self.stratum() != 0 &&
        // 检查传输时间戳不为 0
        self.transmit_timestamp_sec() > 0
        
    }
    
    fn receive_timestamp_sec(&self) -> u64 {
        u32::from_be_bytes([
            self.data[32], self.data[33], self.data[34], self.data[35]
        ]) as u64
    }
    
    fn receive_timestamp_frac(&self) -> f64 {
        let frac = u32::from_be_bytes([
            self.data[36], self.data[37], self.data[38], self.data[39]
        ]);
        frac as f64 / NTP_FRACTION_MAX
    }
    
    fn transmit_timestamp_sec(&self) -> u64 {
        u32::from_be_bytes([
            self.data[40], self.data[41], self.data[42], self.data[43]
        ]) as u64
    }
    
    fn transmit_timestamp_frac(&self) -> f64 {
        let frac = u32::from_be_bytes([
            self.data[44], self.data[45], self.data[46], self.data[47]
        ]);
        frac as f64 / NTP_FRACTION_MAX
    }
}

struct NtpResponse {
    offset_sec: f64,
    delay_sec: f64,
}

fn query_single(server: &str) -> Option<NtpResponse> {
    let sock = UdpSocket::bind("0.0.0.0:0").ok()?;
    sock.set_read_timeout(Some(Duration::from_millis(3000))).ok()?;
    sock.set_write_timeout(Some(Duration::from_millis(3000))).ok()?;
    
    let addrs = server.to_socket_addrs().ok()?;
    
    for addr in addrs {
        let packet = NtpPacket::new_client();
        
        let t1 = SystemTime::now().duration_since(UNIX_EPOCH).ok()?;
        let t1_total = t1.as_secs() as f64 + t1.subsec_nanos() as f64 / 1_000_000_000.0;
        
        if sock.send_to(&packet.data, addr).is_err() {
            continue;
        }
        
        let mut buf = [0u8; 48];
        match sock.recv_from(&mut buf) {
            Ok((size, _)) if size >= 48 => {
                let t4 = SystemTime::now().duration_since(UNIX_EPOCH).ok()?;
                let t4_total = t4.as_secs() as f64 + t4.subsec_nanos() as f64 / 1_000_000_000.0;
                
                let response = NtpPacket { data: buf };
                
                // 增强验证
                if !response.is_valid_response() {
                    continue;
                }
                
                let t2_total = (response.receive_timestamp_sec() - NTP_UNIX_OFFSET) as f64 + response.receive_timestamp_frac();
                let t3_total = (response.transmit_timestamp_sec() - NTP_UNIX_OFFSET) as f64 + response.transmit_timestamp_frac();
                
                let delay = (t4_total - t1_total) - (t3_total - t2_total);
                let offset = ((t2_total - t1_total) + (t3_total - t4_total)) / 2.0;
                
                // 增强延迟检查
                if delay >= 0.0 && delay < 1.0 && delay.abs() < 0.5 {
                    return Some(NtpResponse { offset_sec: offset, delay_sec: delay });
                }
            }
            _ => continue,
        }
    }
    None
}

pub fn fetch_best_ntp() -> Option<(u64, i32, f64)> {
    let servers = super::config::load_ntp_servers();
    
    let mut best_result: Option<(u64, i32, f64)> = None;
    let mut best_delay = f64::MAX;
    
    for server in servers {
        if let Some(response) = query_single(&server) {
            if response.delay_sec < best_delay {
                best_delay = response.delay_sec;
                
                let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?;
                let now_total = now.as_secs() as f64 + now.subsec_nanos() as f64 / 1_000_000_000.0;
                let corrected = now_total + response.offset_sec;
                
                let sec = corrected as u64;
                let us = ((corrected.fract() * 1_000_000.0) as i32).clamp(0, 999_999);
                
                best_result = Some((sec, us, response.delay_sec));
            }
        }
    }
    best_result
}