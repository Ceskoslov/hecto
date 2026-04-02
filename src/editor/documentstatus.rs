use crate::prelude::*;
use super::FileType;

/// 文档状态：保存当前文件的元信息，供状态栏显示使用
#[derive(Default, Eq, PartialEq, Debug)]
pub struct DocumentStatus {
    pub total_lines: usize,          // 文档总行数
    pub current_line_idx: LineIdx,   // 当前光标所在行
    pub is_modified: bool,           // 文档是否已修改
    pub file_name: String,           // 文件名
    pub file_type: FileType,         // 文件类型
}

impl DocumentStatus {
    pub fn modified_indicator_to_string(&self) -> String {
        if self.is_modified {
            "(modified)".to_string()
        } else {
            String::new()
        }
    }

    pub fn line_count_to_string(&self) -> String {
        format!("{} lines", self.total_lines)
    }

    pub fn position_indicator_to_string(&self) -> String {
        format!(
            "{}/{}",
            self.current_line_idx.saturating_add(1),
            self.total_lines
        )
    }
    pub fn file_type_to_string(&self) -> String {
        self.file_type.to_string()
    }
}
