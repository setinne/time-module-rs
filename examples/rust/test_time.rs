/**
 * time_module.dll Rust 调用示例 (FFI)
 * 最低支持: Rust 1.64
 * 编译: rustc test_time.rs
 */

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct FullTime {
    year: i32, month: i32, day: i32,
    hour: i32, minute: i32, second: i32,
    ms: i32, us: i32,
}

type GetVersionFn = extern "C" fn() -> i32;
type GetVersionStringFn = extern "C" fn() -> *const c_char;
type FreeStringFn = extern "C" fn(*mut c_char);
type SetTimezoneOffsetFn = extern "C" fn(i32) -> i32;
type GetLocalTimeFn = extern "C" fn() -> FullTime;
type GetFormattedTimeFn = extern "C" fn() -> *const c_char;
type GetFormattedTimeBufFn = extern "C" fn(*mut u8, i32) -> i32;
type GetWeekdayFn = extern "C" fn(i32, i32, i32) -> i32;
type GetUnixTimestampFn = extern "C" fn() -> i64;
type IsLeapYearExFn = extern "C" fn(i32) -> i32;
type ShutdownFn = extern "C" fn();

fn main() {
    unsafe {
        let dll = libloading::Library::new("time_module.dll")
            .expect("无法加载 time_module.dll");

        let get_version: GetVersionFn = *dll.get(b"api_GetVersion").unwrap();
        let get_version_string: GetVersionStringFn = *dll.get(b"api_GetVersionString").unwrap();
        let free_string: FreeStringFn = *dll.get(b"api_FreeString").unwrap();
        let set_offset: SetTimezoneOffsetFn = *dll.get(b"api_SetTimezoneOffset").unwrap();
        let get_local_time: GetLocalTimeFn = *dll.get(b"api_GetLocalTime").unwrap();
        let get_formatted_time: GetFormattedTimeFn = *dll.get(b"api_GetFormattedTime").unwrap();
        let get_formatted_time_buf: GetFormattedTimeBufFn = *dll.get(b"api_GetFormattedTimeBuf").unwrap();
        let get_weekday: GetWeekdayFn = *dll.get(b"api_GetWeekday").unwrap();
        let get_unix_ts: GetUnixTimestampFn = *dll.get(b"api_GetUnixTimestamp").unwrap();
        let is_leap_ex: IsLeapYearExFn = *dll.get(b"api_IsLeapYearEx").unwrap();
        let shutdown: ShutdownFn = *dll.get(b"api_Shutdown").unwrap();

        // 版本
        let ver = get_version();
        println!("DLL 版本: {}.{}.{}", ver >> 16, (ver >> 8) & 0xFF, ver & 0xFF);
        let vs_ptr = get_version_string();
        let vs = CStr::from_ptr(vs_ptr).to_string_lossy();
        println!("版本字符串: {}", vs);
        free_string(vs_ptr as *mut c_char);

        // 设置时区
        set_offset(28800);

        // 本地时间
        let ft = get_local_time();
        println!("本地时间: {:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
                ft.year, ft.month, ft.day, ft.hour, ft.minute, ft.second, ft.ms);

        // 格式化时间（缓冲区）
        let mut buf = [0u8; 64];
        let len = get_formatted_time_buf(buf.as_mut_ptr(), buf.len() as i32);
        if len > 0 {
            let s = String::from_utf8_lossy(&buf[..len as usize]);
            println!("格式化时间: {}", s);
        }

        // 星期
        let wd = get_weekday(ft.year, ft.month, ft.day);
        println!("星期: {} (0=星期日)", wd);

        // Unix 时间戳
        let ts = get_unix_ts();
        println!("Unix时间戳: {} 秒", ts);

        // 闰年
        let leap = is_leap_ex(2000);
        println!("2000年是闰年: {}", if leap != 0 { "是" } else { "否" });

        // 关闭
        shutdown();
    }
}