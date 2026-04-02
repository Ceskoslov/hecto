use super::super::super::FileType;
use std::{
    fmt::{self, Display},
    path::{Path, PathBuf},
};

/// 文件信息：记录当前编辑文件的路径和类型
#[derive(Default, Debug)]
pub struct FileInfo {
    path: Option<PathBuf>,   // 文件路径（新文件为 None）
    file_type: FileType,     // 文件类型（茎展名推断）
}

impl FileInfo {
    pub fn from(file_name: &str)-> Self {
        let path = PathBuf::from(file_name);
        let file_type = if path.extension().map_or(false, |ext| ext.eq_ignore_ascii_case("rs"))
        {
            FileType::Rust
        } else {
            FileType::Text
        };
        Self {
            path: Some(path),
            file_type,
        }
    }
    pub fn get_path(&self) -> Option<&Path> {
        self.path.as_deref()
    }
    pub const fn has_path(&self) -> bool {
        self.path.is_some()
    }
    pub const fn get_file_type(&self) -> FileType {
        self.file_type
    }
}

impl Display for FileInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.get_path().and_then(|path| path.file_name()).and_then(|name| name.to_str()).unwrap_or("[No Name]");
        write!(f, "{}", name)
    }
    
}