//! 时区处理：国家代码 ↔ 偏移，时区名称 ↔ 偏移

mod country;
mod name;

pub use country::offset_from_location;
pub use name::get_offset_by_name;
