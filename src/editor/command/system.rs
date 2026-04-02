use crate::prelude::*;

use crossterm::event::{
    KeyCode::{self, Char},
    KeyEvent, KeyModifiers,
};

/// 系统命令：保存、退出、搜索、取消、窗口大小调整
#[derive(Clone, Copy)]
pub enum System {
    Save,          // Ctrl-S 保存
    Quit,          // Ctrl-Q 退出
    Resize(Size),  // 窗口大小变化
    Dismiss,       // Esc 取消/关闭提示
    Search,        // Ctrl-F 搜索
}

impl TryFrom<KeyEvent> for System {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        if modifiers == KeyModifiers::CONTROL {
            match code {
                Char('s') => Ok(System::Save),
                Char('q') => Ok(System::Quit),
                Char('f') => Ok(System::Search),
                _ => Err(format!("Unknown control key: {:?}", code)),
            }
        } else if modifiers == KeyModifiers::NONE && matches!(code, KeyCode::Esc) {
            Ok(System::Dismiss)
        } else {
            Err(format!("Unknown key: {:?}", code))
        }
    }
}
