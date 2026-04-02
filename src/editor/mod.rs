//! 编辑器核心模块：协调所有 UI 组件，处理用户输入和事件循环

use crate::prelude::*;
use crossterm::event::{Event, KeyEvent, KeyEventKind, read};
use std::{
    env,
    io::Error,
    panic::{set_hook, take_hook},
};
// 带注解的字符串系统，用于语法高亮和搜索结果高亮
mod annotatedstring;
// 注解类型枚举（匹配、关键字、类型、注释等）
pub mod annotationtype;
// 命令解析模块，将键盘事件转化为编辑器命令
mod command;
// 文档状态（文件名、是否修改、光标位置等）
mod documentstatus;
// 行处理模块，处理 Unicode 字素簇的编辑、搜索和显示
mod line;
// 终端抽象层，封装 crossterm 的底层操作
mod terminal;
// UI 组件集合（视图、状态栏、命令栏、消息栏）
mod uicomponents;
pub use annotationtype::AnnotationType;
mod annotation;
pub use annotation::Annotation;

mod filetype;
use filetype::FileType;
use annotatedstring::{AnnotatedString};
use documentstatus::DocumentStatus;
use line::Line;

use terminal::Terminal;
use uicomponents::{CommandBar, MessageBar, StatusBar, UIComponent, View};

use self::command::{
    Command::{self, Edit, Move, System},
    Edit::InsertNewline,
    Move::{Down, Left, Right, Up},
    System::{Dismiss, Quit, Resize, Save, Search},
};

/// 连续退出所需的次数（未保存时防止误触退出）
const QUIT_TIMES: u8 = 3;

/// 提示模式：控制底部命令栏的用途
#[derive(Eq, PartialEq, Default)]
enum PromptType {
    Search,  // 搜索模式
    Save,    // 另存为模式
    #[default]
    None,    // 正常编辑模式
}

impl PromptType {
    fn is_none(&self) -> bool {
        *self == Self::None
    }
}

/// 编辑器主结构体：管理所有 UI 组件和编辑状态
#[derive(Default)]
pub struct Editor {
    should_quit: bool,          // 是否退出主循环
    view: View,                 // 文本编辑视图（占据主要屏幕区域）
    status_bar: StatusBar,      // 状态栏（显示文件信息）
    message_bar: MessageBar,    // 消息栏（显示帮助提示）
    command_bar: CommandBar,    // 命令栏（搜索/保存时的输入框）
    prompt_type: PromptType,    // 当前提示模式
    terminal_size: Size,        // 终端尺寸
    title: String,              // 终端标题
    quit_times: u8,             // 剩余需要按几次 Ctrl-Q 才能强制退出
}

impl Editor {
    /// 创建编辑器实例：初始化终端、设置 panic hook、加载命令行指定的文件
    pub fn new() -> Result<Self, Error> {
        // 保存原始的 panic hook，并设置新的 hook 在 panic 时恢复终端状态
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut editor = Self::default();
        let size = Terminal::size().unwrap_or_default();
        editor.handle_resize_command(size);
        editor.update_message("HELP: Ctrl-F= find | Ctrl-S= save | Ctrl-Q= quit");
        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            debug_assert!(!file_name.is_empty());
            if editor.view.load(file_name).is_err() {
                editor.update_message(&format!("Error loading file: {file_name}"));
            }
        }
        editor.refresh_status();
        Ok(editor)
    }

    /// 主事件循环：不断刷新屏幕、读取用户输入、处理命令
    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                    #[cfg(not(debug_assertions))]
                    {
                        let _ = err;
                    }
                }
            }
            self.refresh_status();
        }
    }

    /// 刷新屏幕：按顺序渲染底部栏、状态栏、主视图，然后移动光标
    fn refresh_screen(&mut self) {
        if self.terminal_size.height == 0 || self.terminal_size.width == 0 {
            return;
        }
        let bottom_bar_row = self.terminal_size.height.saturating_sub(1);
        let _ = Terminal::hide_caret();
        if self.in_prompt() {
            self.command_bar.render(bottom_bar_row);
        } else {
            self.message_bar.render(bottom_bar_row);
        }
        if self.terminal_size.height > 1 {
            self.status_bar
                .render(self.terminal_size.height.saturating_sub(2));
        }
        if self.terminal_size.height > 2 {
            self.view.render(0);
        }
        let new_caret_pos = if self.in_prompt() {
            Position {
                row: bottom_bar_row,
                col: self.command_bar.caret_position_col(),
            }
        } else {
            self.view.caret_position()
        };
        debug_assert!(new_caret_pos.row <= self.terminal_size.height);
        debug_assert!(new_caret_pos.col <= self.terminal_size.width);
        let _ = Terminal::move_caret_to(new_caret_pos);
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }

    fn refresh_status(&mut self) {
        let status = self.view.get_status();
        let title = format!("{} - {NAME} ", status.file_name);
        self.status_bar.update_status(status);
        if title != self.title && matches!(Terminal::set_title(&title), Ok(_)) {
            self.title = title;
        }
    }
    /// 评估终端事件：只处理按键和窗口大小变化事件
    fn evaluate_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if should_process {
            if let Ok(command) = Command::try_from(event) {
                self.process_command(command);
            }
        }
    }

    /// 命令分发：根据当前提示模式将命令路由到不同的处理函数
    fn process_command(&mut self, command: Command) {
        if let System(Resize(size)) = command {
            self.handle_resize_command(size);
            return;
        }
        match self.prompt_type {
            PromptType::Search => self.process_command_during_search(command),
            PromptType::Save => self.process_command_during_save(command),
            PromptType::None => self.process_command_no_prompt(command),
        }
    }

    /// 处理普通模式下的命令：移动、编辑、保存、搜索、退出
    fn process_command_no_prompt(&mut self, command: Command) {
        if matches!(command, System(Quit)) {
            self.handle_quit_command();
            return;
        }
        self.reset_quit_times();
        match command {
            System(Quit | Resize(_) | Dismiss) => {}
            System(Search) => self.set_prompt(PromptType::Search),
            System(Save) => self.handle_save_command(),
            Edit(edit_command) => self.view.handle_edit_command(edit_command),
            Move(move_command) => self.view.handle_move_command(move_command),
        }
    }

    /// 处理窗口大小变化：重新计算各组件的尺寸
    fn handle_resize_command(&mut self, size: Size) {
        self.terminal_size = size;
        self.view.resize(Size {
            height: size.height.saturating_sub(2),
            width: size.width,
        });
        let bar_size = Size {
            height: 1,
            width: size.width,
        };
        self.message_bar.resize(bar_size);
        self.status_bar.resize(bar_size);
        self.command_bar.resize(bar_size);
    }
    /// 处理退出命令：未保存时需连续按 Ctrl-Q 多次确认
    #[allow(clippy::arithmetic_side_effects)]
    fn handle_quit_command(&mut self) {
        if !self.view.get_status().is_modified || self.quit_times + 1 == QUIT_TIMES {
            self.should_quit = true;
        } else if self.view.get_status().is_modified {
            self.update_message(&format!("WARNING: File has unsaved changes. Press Ctrl-Q {} more times to quit without saving.", QUIT_TIMES - self.quit_times - 1));
            self.quit_times += 1;
        }
    }

    fn reset_quit_times(&mut self) {
        if self.quit_times > 0 {
            self.quit_times = 0;
            self.update_message("");
        }
    }
    fn handle_save_command(&mut self) {
        if self.view.is_file_loaded() {
            self.save(None);
        } else {
            self.set_prompt(PromptType::Save);
        }
    }

    fn process_command_during_save(&mut self, command: Command) {
        match command {
            System(Quit | Resize(_) | Search | Save) | Move(_) => {}
            System(Dismiss) => {
                self.set_prompt(PromptType::None);
                self.update_message("Save aborted.");
            }
            Edit(InsertNewline) => {
                let file_name = self.command_bar.value();
                self.save(Some(&file_name));
                self.set_prompt(PromptType::None);
            }
            Edit(edit_command) => self.command_bar.handle_edit_command(edit_command),
        }
    }
    fn save(&mut self, file_name: Option<&str>) {
        let result = if let Some(name) = file_name {
            self.view.save_as(name)
        } else {
            self.view.save()
        };
        if result.is_ok() {
            self.update_message("File saved successfully.");
        } else {
            self.update_message(&format!("Error saving file: {result:?}"));
        }
    }

    /// 处理搜索模式下的命令：Esc 取消、Enter 确认、箭头键导航、字符输入更新搜索
    fn process_command_during_search(&mut self, command: Command) {
        match command {
            System(Dismiss) => {
                self.set_prompt(PromptType::None);
                self.view.dismiss_search();
            }
            Edit(InsertNewline) => {
                self.set_prompt(PromptType::None);
                self.view.exit_search();
            }
            Edit(edit_command) => {
                self.command_bar.handle_edit_command(edit_command);
                let query = self.command_bar.value();
                self.view.search(&query);
            }
            Move(Right | Down) => self.view.search_next(),
            Move(Left | Up) => self.view.search_prev(),
            System(Quit | Resize(_) | Save | Search) | Move(_) => {}
        }
    }

    fn update_message(&mut self, message: &str) {
        self.message_bar.update_message(message);
    }

    fn in_prompt(&self) -> bool {
        !self.prompt_type.is_none()
    }

    /// 设置提示模式并配置命令栏提示文字
    fn set_prompt(&mut self, prompt_type: PromptType) {
        self.prompt_type = prompt_type;
        self.command_bar.clear_value();
        match self.prompt_type {
            PromptType::None => self.message_bar.set_needs_redraw(true),
            PromptType::Save => self.command_bar.set_prompt("Save as: "),
            PromptType::Search => {
                self.view.enter_search();
                self.command_bar
                    .set_prompt("Search(Esc to cancel, Arrows to navigate): ");
            }
        }
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye.\r\n");
        }
    }
}
