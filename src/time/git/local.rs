// 获取系统 UTC 时间及单调时钟（兼容 XP，使用 Once + static mut）
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use std::sync::Once;

pub fn get_system_time_utc() -> (u64, i32) {
    let d = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    (d.as_secs(), d.subsec_micros() as i32)
}

static MONO_ONCE: Once = Once::new();
static mut MONO_START: Option<Instant> = None;

pub fn monotonic_secs() -> f64 {
    MONO_ONCE.call_once(|| {
        unsafe { MONO_START = Some(Instant::now()); }
    });
    let start = unsafe { MONO_START.as_ref().unwrap() };
    start.elapsed().as_secs_f64()
}