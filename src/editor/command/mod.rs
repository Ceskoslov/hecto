//! 命令模块：将终端事件解析为编辑器命令
//! 采用三层命令架构：Move（光标移动）、Edit（文本编辑）、System（系统操作）

use crate::prelude::*;
use crossterm::event::Event;

use std::convert::TryFrom;

mod movecommand;
pub use movecommand::Move;
mod system;
pub use system::System;
mod edit;
pub use edit::Edit;

/// 编辑器命令枚举：将所有操作分为三类
#[derive(Clone, Copy)]
pub enum Command {
    Move(Move),       // 光标移动命令
    Edit(Edit),       // 文本编辑命令
    System(System),   // 系统级命令（保存、退出等）
}

/// 事件转命令：依次尝试解析为 Edit -> Move -> System
#[allow(clippy::as_conversions)]
impl TryFrom<Event> for Command {
    type Error = String;
    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(key_event) => Edit::try_from(key_event)
                .map(Command::Edit)
                .or_else(|_| Move::try_from(key_event).map(Command::Move))
                .or_else(|_| System::try_from(key_event).map(Command::System))
                .map_err(|_err| format!("Event not supported: {key_event:?}")),
            Event::Resize(width_u16, height_h16) => Ok(Self::System(System::Resize(Size {
                width: width_u16 as usize,
                height: height_h16 as usize,
            }))),
            _ => Err(format!("Event not supported: {:?}", event)),
        }
    }
}
