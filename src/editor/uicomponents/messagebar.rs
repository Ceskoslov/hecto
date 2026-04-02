use std::{
    io::Error,
    time::{Duration, Instant},
};

use crate::prelude::*;

use super::super::Terminal;

use super::UIComponent;

/// 消息默认显示时长（5 秒）
const DEFAULT_DURATION: Duration = Duration::new(5, 0);

/// 带时间戳的消息，到期后自动消失
struct Message {
    text: String,
    time: Instant,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            text: String::new(),
            time: Instant::now(),
        }
    }
}

impl Message {
    fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.time) > DEFAULT_DURATION
    }
}

/// 消息栏：显示帮助提示和警告信息，消息到期后自动清除
#[derive(Default)]
pub struct MessageBar {
    current_message: Message,
    needs_redraw: bool,
    cleared_after_expiry: bool,
}

impl MessageBar {
    pub fn update_message(&mut self, new_message: &str) {
        self.current_message = Message {
            text: new_message.to_string(),
            time: Instant::now(),
        };
        self.cleared_after_expiry = false;
        self.set_needs_redraw(true);
    }
}

impl UIComponent for MessageBar {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        (!self.cleared_after_expiry && self.current_message.is_expired()) || self.needs_redraw
    }

    fn set_size(&mut self, _: Size) {}

    fn draw(&mut self, origin: RowIdx) -> Result<(), Error> {
        if self.current_message.is_expired() {
            self.cleared_after_expiry = true;
        }
        let message = if self.current_message.is_expired() {
            ""
        } else {
            &self.current_message.text
        };
        Terminal::print_row(origin, message)
    }
}
