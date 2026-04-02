use crossterm::event::{
    KeyCode::{Down, End, Home, Left, PageDown, PageUp, Right, Up},
    KeyEvent, KeyModifiers,
};

/// 光标移动命令：支持方向键、翻页、行首行尾
#[derive(Clone, Copy)]
pub enum Move {
    PageDown,      // 下翻一页
    PageUp,        // 上翻一页
    StartOfLine,   // 移动到行首
    EndOfLine,     // 移动到行尾
    Up,            // 上移一行
    Down,          // 下移一行
    Left,          // 左移一个字素簇
    Right,         // 右移一个字素簇
}

impl TryFrom<KeyEvent> for Move {
    type Error = String;
    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        if modifiers == KeyModifiers::NONE {
            match code {
                Up => Ok(Self::Up),
                Down => Ok(Self::Down),
                Left => Ok(Self::Left),
                Right => Ok(Self::Right),
                PageDown => Ok(Self::PageDown),
                PageUp => Ok(Self::PageUp),
                Home => Ok(Self::StartOfLine),
                End => Ok(Self::EndOfLine),
                _ => Err(format!("Unsupported key: {:?}", code)),
            }
        } else {
            Err(format!("Unsupported modifiers: {:?}", modifiers))
        }
    }
}
