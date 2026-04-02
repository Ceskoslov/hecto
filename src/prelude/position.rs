use super::{ColIdx, RowIdx};

/// 屏幕位置：以行和列为单位的终端显示坐标
/// 与 Location（逻辑坐标）不同，Position 表示屏幕上的渲染位置
#[derive(Copy, Clone, Default)]
pub struct Position {
    pub col: ColIdx,
    pub row: RowIdx,
}

impl Position {
    pub const fn saturating_sub(self, other: Self) -> Self {
        Self {
            row: self.row.saturating_sub(other.row),
            col: self.col.saturating_sub(other.col),
        }
    }
}
