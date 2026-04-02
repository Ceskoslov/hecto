//! prelude 模块：定义项目中广泛使用的类型别名和基础数据结构
//! 为不同的索引类型赋予语义化名称，提高代码可读性

/// 字素簇索引（以 Unicode 字素簇为单位的偏移量）
pub type GraphemeIdx = usize;
/// 行索引
pub type LineIdx = usize;
/// 字节索引（原始字节偏移量）
pub type ByteIdx = usize;
/// 列索引（屏幕列位置）
pub type ColIdx = usize;
/// 行索引（屏幕行位置）
pub type RowIdx = usize;

mod position;
pub use position::Position;
mod size;
pub use size::Size;
mod location;
pub use location::Location;
/// 程序名称，从 Cargo.toml 中自动获取
pub const NAME: &str = env!("CARGO_PKG_NAME");
/// 程序版本号，从 Cargo.toml 中自动获取
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
