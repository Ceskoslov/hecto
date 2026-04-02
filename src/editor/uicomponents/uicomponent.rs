use crate::prelude::*;
use std::io::Error;

/// UI 组件 trait：所有界面组件的公共接口
/// 提供统一的重绘标记、尺寸调整和渲染机制
pub trait UIComponent {
    fn set_needs_redraw(&mut self, value: bool);
    #[allow(dead_code)]
    fn needs_redraw(&self) -> bool;
    fn resize(&mut self, size: Size) {
        self.set_size(size);
        self.set_needs_redraw(true);
    }
    fn set_size(&mut self, size: Size);

    fn render(&mut self, origin_row: RowIdx) {
        if let Err(err) = self.draw(origin_row) {
            #[cfg(debug_assertions)]
            {
                panic!("Error rendering component: {err:?}");
            }
            #[cfg(not(debug_assertions))]
            {
                let _ = err;
            }
        } else {
            self.set_needs_redraw(false);
        }
    }
    fn draw(&mut self, origin_row: RowIdx) -> Result<(), Error>;
}
