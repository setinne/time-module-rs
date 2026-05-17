// tests/ntp_integration.rs
// 集成测试：模拟 NTP 服务器，验证同步功能
// 使用内部函数，无需动态加载 DLL

use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// 引入被测试库的内部模块（需要 lib.rs 或 time_api.rs 中导出这些函数）
use time_module::time_api::{
    api_ForceResyncEx, api_GetNTPStatus, api_IsNetworkTimeAvailableEx,
    api_SetAutoSyncEnabled, api_Shutdown,
};

/// 构造一个有效的 NTP 响应包（简化版）
/// 返回一个 48 字节的 NTP 包，模拟服务器时间为当前系统时间 + offset_secs
fn build_ntp_response(offset_secs: i64) -> [u8; 48] {
    let mut packet = [0u8; 48];
    // LI=0, VN=4, Mode=4 (Server)
    packet[0] = 0x1c;
    // Stratum=1 (primary reference)
    packet[1] = 1;
    // Poll=4 (16s), Precision=0xfa (-6)
    packet[2] = 4;
    packet[3] = 0xfa;

    // 获取当前系统时间（UTC 秒）
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let server_time = now + offset_secs;
    // NTP 时间戳从 1900-01-01 开始，需要加上 2208988800
    let ntp_secs = (server_time + 2208988800) as u32;
    // 设置 Transmit Timestamp (偏移 40 字节)
    packet[40..44].copy_from_slice(&ntp_secs.to_be_bytes());
    // 设置 Fraction 部分为 0（简化）
    packet
}

/// 启动一个模拟 NTP 服务器，在指定端口监听，返回线程和停止标志
fn start_mock_ntp_server(port: u16, offset_secs: i64) -> (Arc<AtomicBool>, std::thread::JoinHandle<()>) {
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    let response = build_ntp_response(offset_secs);
    let handle = thread::spawn(move || {
        let socket = UdpSocket::bind(("127.0.0.1", port)).expect("Failed to bind UDP socket");
        let mut buf = [0u8; 1024];
        while running_clone.load(Ordering::Relaxed) {
            match socket.recv_from(&mut buf) {
                Ok((size, src)) => {
                    // 简单回复固定的响应包
                    let _ = socket.send_to(&response, src);
                }
                Err(e) => {
                    eprintln!("Mock NTP server recv error: {}", e);
                    break;
                }
            }
        }
    });
    (running, handle)
}

#[test]
fn test_ntp_sync_with_mock_server() {
    // 禁用自动同步，避免后台干扰
    unsafe { api_SetAutoSyncEnabled(false) };

    // 启动模拟 NTP 服务器，偏移 +3600 秒（1 小时）
    let port = 12345;
    let (_running, _handle) = start_mock_ntp_server(port, 3600);

    // 设置环境变量让 NTP 客户端使用本地服务器（需要代码支持）
    // 注意：原项目可能从配置文件读取 NTP 服务器列表，这里假设有一个 api_SetNtpServers 或环境变量
    // 如果没有，可以临时修改内部全局配置（仅测试用）。这里我们直接修改内部静态变量（通过 unsafe）。
    // 为了测试，我们定义一个内部函数来设置服务器地址（需要在库中添加测试钩子）。
    // 若无法动态修改，可以预先将 ntp_servers.txt 内容改为 "127.0.0.1:12345"。
    // 由于我们无法修改外部文件，这里假设库支持环境变量 "TIME_MODULE_NTP_SERVERS"。
    std::env::set_var("TIME_MODULE_NTP_SERVERS", format!("127.0.0.1:{}", port));

    // 强制同步
    let ret = unsafe { api_ForceResyncEx() };
    assert_eq!(ret, 0, "api_ForceResyncEx should return Success (0)");

    // 等待最多 5 秒，直到 NTP 状态变为 Synced (2)
    let mut synced = false;
    for _ in 0..50 {
        let status = unsafe { api_GetNTPStatus() };
        if status == 2 {
            synced = true;
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
    assert!(synced, "NTP sync did not complete within 5 seconds");

    // 验证网络时间可用
    let available = unsafe { api_IsNetworkTimeAvailableEx() };
    assert_eq!(available, 1, "Network time should be available");

    // 清理：关闭后台线程
    unsafe { api_Shutdown() };
    // 停止模拟服务器
    _running.store(false, Ordering::Relaxed);
    // 等待线程结束
    _handle.join().unwrap();
}

#[test]
fn test_ntp_status_not_started() {
    unsafe { api_SetAutoSyncEnabled(false) };
    // 确保 NTP 未启动（可能需要重置状态）
    let status = unsafe { api_GetNTPStatus() };
    // 根据实现，可能是 NotStarted (0) 或 Syncing (1)，这里断言至少不是 Synced
    assert!(status == 0 || status == 1, "Unexpected status: {}", status);
    unsafe { api_Shutdown() };
}