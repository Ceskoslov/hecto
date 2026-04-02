use super::{GraphemeIdx, LineIdx};

/// 文本位置：以字素簇和行号为单位的逻辑坐标
/// 与 Position（屏幕坐标）不同，Location 表示文本内容中的位置
#[derive(Copy, Clone, Default)]
pub struct Location {
    /// 当前行内的字素簇索引
    pub grapheme_idx: GraphemeIdx,
    /// 行索引
    pub line_idx: LineIdx,
}
