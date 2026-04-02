use std::fmt::{Display, Formatter, Result};

/// 文件类型：用于决定使用哪种语法高亮器
#[derive(Default, Eq, PartialEq, Debug, Clone, Copy)]
pub enum FileType {
    Rust,     // Rust 源文件，启用 Rust 语法高亮
    #[default]
    Text,     // 纯文本文件，无语法高亮
}

impl Display for FileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            FileType::Rust => write!(f, "Rust"),
            FileType::Text => write!(f, "Text"),
        }
    }
}