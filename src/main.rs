// hecto 文本编辑器 —— 入口文件
// 启用 clippy 的严格 lint 检查，确保代码质量
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]
mod editor;
use editor::Editor;
// prelude 模块：定义全局通用的类型别名和常量，如 Position、Size、Location 等
mod prelude;

/// 程序入口：创建编辑器实例并启动主事件循环
fn main() {
    Editor::new().unwrap().run();
}
