//! 常量定义

/// 有效时区偏移范围：UTC-12 到 UTC+14（-43200 秒 到 50400 秒）
/// 注：UTC+13 和 UTC+14 为政治选择时区（如基里巴斯、托克劳）
pub const MIN_VALID_OFFSET: i32 = -43200;   // UTC-12
pub const MAX_VALID_OFFSET: i32 = 50400;    // UTC+14

/// 常用时区偏移常量（秒）
pub const OFFSET_UTC_MINUS_12: i32 = -43200;
pub const OFFSET_UTC_MINUS_11: i32 = -39600;
pub const OFFSET_UTC_MINUS_10: i32 = -36000;
pub const OFFSET_UTC_MINUS_9:  i32 = -32400;
pub const OFFSET_UTC_MINUS_8:  i32 = -28800;
pub const OFFSET_UTC_MINUS_7:  i32 = -25200;
pub const OFFSET_UTC_MINUS_6:  i32 = -21600;
pub const OFFSET_UTC_MINUS_5:  i32 = -18000;
pub const OFFSET_UTC_MINUS_4:  i32 = -14400;
pub const OFFSET_UTC_MINUS_3:  i32 = -10800;
pub const OFFSET_UTC_MINUS_2:  i32 = -7200;
pub const OFFSET_UTC_MINUS_1:  i32 = -3600;
pub const OFFSET_UTC_0:        i32 = 0;
pub const OFFSET_UTC_PLUS_1:   i32 = 3600;
pub const OFFSET_UTC_PLUS_2:   i32 = 7200;
pub const OFFSET_UTC_PLUS_3:   i32 = 10800;
pub const OFFSET_UTC_PLUS_4:   i32 = 14400;
pub const OFFSET_UTC_PLUS_5:   i32 = 18000;
pub const OFFSET_UTC_PLUS_6:   i32 = 21600;
pub const OFFSET_UTC_PLUS_7:   i32 = 25200;
pub const OFFSET_UTC_PLUS_8:   i32 = 28800;
pub const OFFSET_UTC_PLUS_9:   i32 = 32400;
pub const OFFSET_UTC_PLUS_10:  i32 = 36000;
pub const OFFSET_UTC_PLUS_11:  i32 = 39600;
pub const OFFSET_UTC_PLUS_12:  i32 = 43200;
pub const OFFSET_UTC_PLUS_13:  i32 = 46800;  // 托克劳、萨摩亚（DST）
pub const OFFSET_UTC_PLUS_14:  i32 = 50400;  // 基里巴斯莱恩群岛

/// 版本号常量
pub const VERSION_MAJOR: i32 = 0;
pub const VERSION_MINOR: i32 = 2;
pub const VERSION_PATCH: i32 = 20;