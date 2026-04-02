use crossterm::event::{
    KeyCode::{Backspace, Char, Delete, Enter, Tab},
    KeyEvent, KeyModifiers,
};

/// 编辑命令：插入字符、换行、删除等基本文本操作
#[derive(Clone, Copy)]
pub enum Edit {
    Insert(char),      // 插入字符
    InsertNewline,     // 插入新行
    Delete,            // 向前删除
    DeleteBackward,    // 向后删除（退格）
}

impl TryFrom<KeyEvent> for Edit {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        match (event.code, event.modifiers) {
            (Char(character), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                Ok(Self::Insert(character))
            }
            (Tab, KeyModifiers::NONE) => Ok(Self::Insert('\t')),
            (Enter, KeyModifiers::NONE) => Ok(Self::InsertNewline),
            (Delete, KeyModifiers::NONE) => Ok(Self::Delete),
            (Backspace, KeyModifiers::NONE) => Ok(Self::DeleteBackward),
            _ => Err(format!(
                "Unsupported key event: {:?} with modifiers {:?}",
                event.code, event.modifiers
            )),
        }
    }
}
