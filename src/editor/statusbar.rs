use super::{
    terminal::{Terminal, Size},
    DocumentStatus,
};

pub struct StatusBar {
    current_status: DocumentStatus,
    need_redraw: bool,
    margin_bottom: usize,
    width: usize,
    position_y: usize,
}

impl StatusBar {
    pub fn new(margin_bottom: usize)-> Self {
        let size = Terminal::size().unwrap_or_default();
        Self {
            current_status: DocumentStatus::default(),
            need_redraw: true,
            margin_bottom,
            width: size.width,
            position_y: size.height.saturating_sub(margin_bottom).saturating_sub(1),
        }
    }

    pub fn resize(&mut self, size: Size){
        self.width = size.width;
        self.position_y = size.height.saturating_sub(self.margin_bottom).saturating_sub(1);
        self.need_redraw = true;
    }

    pub fn update_status(&mut self, status: DocumentStatus){
        if self.current_status != status {
            self.current_status = status;
            self.need_redraw = true;
        }
    }

    pub fn render(&mut self) {
        if !self.need_redraw {
            return;
        }
        let mut status = format!("{:?}", self.current_status);
        status.truncate(self.width);
        let result = Terminal::print_row(self.position_y, &status);
        debug_assert!(result.is_ok(), "Failed to render status bar: {result:?}");
        self.need_redraw = false;
    }
}