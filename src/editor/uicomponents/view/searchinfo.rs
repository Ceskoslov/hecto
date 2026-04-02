use crate::editor::Line;
use crate::prelude::*;

/// 搜索信息：保存搜索状态和搜索前的光标位置（用于取消搜索时恢复）
pub struct SearchInfo {
    pub prev_location: Location,       // 搜索前的光标位置
    pub prev_scroll_offset: Position,  // 搜索前的滚动偏移
    pub query: Option<Line>,           // 搜索关键词
}
