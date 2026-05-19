/**
 * time_module.dll Rust 调用示例 (FFI)
 * 最低支持: Rust 1.64
 * 编译: rustc test_time.rs
 * 
 * v0.2.18 更新: 添加错误码处理和错误字符串演示
 * 
 * 需要在 Cargo.toml 中添加:
 * [dependencies]
 * libloading = "0.8"
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
type GetFormattedTimeBufFn = extern "C" fn(*mut u8, i32) -> i32;
type GetWeekdayFn = extern "C" fn(i32, i32, i32) -> i32;
type GetUnixTimestampFn = extern "C" fn() -> i64;
type IsLeapYearExFn = extern "C" fn(i32) -> i32;
type GetErrorStringFn = extern "C" fn(i32) -> *const c_char;
type ShutdownFn = extern "C" fn();

fn print_error(func_name: &str, error_code: i32, get_error_string: GetErrorStringFn, free_string: FreeStringFn) {
    if error_code != 0 {
        let err_ptr = get_error_string(error_code);
        let err_str = unsafe { CStr::from_ptr(err_ptr).to_string_lossy() };
        eprintln!("  [错误] {} 失败: 错误码 {} - {}", func_name, error_code, err_str);
        free_string(err_ptr as *mut c_char);
    }
}

fn main() {
    unsafe {
        let dll = libloading::Library::new("time_module.dll")
            .expect("无法加载 time_module.dll");

        let get_version: GetVersionFn = *dll.get(b"api_GetVersion").unwrap();
        let get_version_string: GetVersionStringFn = *dll.get(b"api_GetVersionString").unwrap();
        let free_string: FreeStringFn = *dll.get(b"api_FreeString").unwrap();
        let set_offset: SetTimezoneOffsetFn = *dll.get(b"api_SetTimezoneOffset").unwrap();
        let get_local_time: GetLocalTimeFn = *dll.get(b"api_GetLocalTime").unwrap();
        let get_formatted_time_buf: GetFormattedTimeBufFn = *dll.get(b"api_GetFormattedTimeBuf").unwrap();
        let get_weekday: GetWeekdayFn = *dll.get(b"api_GetWeekday").unwrap();
        let get_unix_ts: GetUnixTimestampFn = *dll.get(b"api_GetUnixTimestamp").unwrap();
        let is_leap_ex: IsLeapYearExFn = *dll.get(b"api_IsLeapYearEx").unwrap();
        let get_error_string: GetErrorStringFn = *dll.get(b"api_GetErrorString").unwrap();
        let shutdown: ShutdownFn = *dll.get(b"api_Shutdown").unwrap();

        println!("========== time_module.dll 示例 (v0.2.18) ==========\n");

        // 1. 版本
        let ver = get_version();
        println!("[1] 版本信息");
        println!("    DLL 版本: {}.{}.{}", ver >> 16, (ver >> 8) & 0xFF, ver & 0xFF);
        let vs_ptr = get_version_string();
        let vs = CStr::from_ptr(vs_ptr).to_string_lossy();
        println!("    版本字符串: {}", vs);
        free_string(vs_ptr as *mut c_char);
        println!();

        // 2. 设置时区
        println!("[2] 时区设置");
        let ret = set_offset(28800);
        if ret != 0 {
            print_error("SetTimezoneOffset", ret, get_error_string, free_string);
        } else {
            println!("    设置时区 UTC+8 成功");
        }

        // 3. 演示无效时区偏移
        println!("\n[3] 无效时区偏移测试");
        let ret = set_offset(50400);
        if ret == 18 {
            println!("    超出范围值 50400 正确返回错误码 18 (TimezoneOffsetOutOfRange)");
        }
        set_offset(28800);
        println!();

        // 4. 本地时间
        println!("[4] 本地时间");
        let ft = get_local_time();
        println!("    本地时间: {:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
                 ft.year, ft.month, ft.day, ft.hour, ft.minute, ft.second, ft.ms);
        println!();

        // 5. 格式化时间
        println!("[5] 格式化时间");
        let mut buf = [0u8; 64];
        let len = get_formatted_time_buf(buf.as_mut_ptr(), buf.len() as i32);
        if len > 0 {
            let s = String::from_utf8_lossy(&buf[..len as usize]);
            println!("    格式化时间: {}", s);
        }
        println!();

        // 6. 星期
        println!("[6] 星期信息");
        let wd = get_weekday(ft.year, ft.month, ft.day);
        let weekdays = ["星期日", "星期一", "星期二", "星期三", "星期四", "星期五", "星期六"];
        println!("    星期: {} ({})", weekdays[wd as usize], wd);
        println!();

        // 7. Unix 时间戳
        println!("[7] Unix 时间戳");
        let ts = get_unix_ts();
        println!("    Unix时间戳: {} 秒", ts);
        println!();

        // 8. 闰年
        println!("[8] 闰年判断");
        let leap = is_leap_ex(2000);
        println!("    2000年是闰年: {}", if leap != 0 { "是" } else { "否" });
        println!();

        // 9. 错误字符串演示
        println!("[9] 错误字符串演示");
        let err_ptr = get_error_string(18);
        let err_str = CStr::from_ptr(err_ptr).to_string_lossy();
        println!("    错误码 18: {}", err_str);
        free_string(err_ptr as *mut c_char);
        let err_ptr = get_error_string(19);
        let err_str = CStr::from_ptr(err_ptr).to_string_lossy();
        println!("    错误码 19: {}", err_str);
        free_string(err_ptr as *mut c_char);
        println!();

        // 10. 关闭
        println!("[10] 关闭 DLL");
        shutdown();
        println!("    完成");
    }
}