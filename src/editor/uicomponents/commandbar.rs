use std::{cmp::min, io::Error};

use crate::prelude::*;

use super::super::{Line, Terminal, command::Edit};
use super::UIComponent;

/// 命令栏：底部输入区域，用于搜索和另存为时的文本输入
#[derive(Default)]
pub struct CommandBar {
    prompt: String,      // 提示文字（如 "Search: "）
    value: Line,         // 用户输入的内容
    needs_redraw: bool,  // 是否需要重绘
    size: Size,          // 组件尺寸
}

impl CommandBar {
    pub fn handle_edit_command(&mut self, command: Edit) {
        match command {
            Edit::Insert(character) => self.value.append_char(character),
            Edit::Delete | Edit::InsertNewline => {}
            Edit::DeleteBackward => self.value.delete_last(),
        }
        self.set_needs_redraw(true);
    }
    pub fn caret_position_col(&self) -> ColIdx {
        let max_width = self
            .prompt
            .len()
            .saturating_add(self.value.grapheme_count());
        min(max_width, self.size.width)
    }
    pub fn value(&self) -> String {
        self.value.to_string()
    }
    pub fn set_prompt(&mut self, prompt: &str) {
        self.prompt = prompt.to_string();
        self.set_needs_redraw(true);
    }

    pub fn clear_value(&mut self) {
        self.value = Line::default();
        self.set_needs_redraw(true);
    }
}

impl UIComponent for CommandBar {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }
    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }
    fn set_size(&mut self, size: Size) {
        self.size = size;
    }
    fn draw(&mut self, origin: RowIdx) -> Result<(), Error> {
        let area_for_value = self.size.width.saturating_sub(self.prompt.len()); //this is how much space there is between the right side of the prompt and the edge of the bar
        let value_end = self.value.width(); // we always want to show the left part of the value, therefore the end of the visible range we try to access will be equal to the full width
        let value_start = value_end.saturating_sub(area_for_value); //This should give us the start for the grapheme subrange we want to print out.
        let message = format!(
            "{}{}",
            self.prompt,
            self.value.get_visible_graphemes(value_start..value_end)
        );
        let to_print = if message.len() <= self.size.width {
            message
        } else {
            String::new()
        };
        Terminal::print_row(origin, &to_print)
    }
}
