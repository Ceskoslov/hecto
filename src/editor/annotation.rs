use crate::prelude::*;

use super::AnnotationType;

/// 注解：表示文本中一个需要特殊渲染的区间（如搜索匹配、语法高亮）
/// 以字节索引定义起止位置，与 AnnotationType 配合决定显示样式
#[derive(Copy, Clone, Debug)]
#[allow(clippy::struct_field_names)]
pub struct Annotation {
    pub annotation_type: AnnotationType,
    pub start: ByteIdx,  // 注解起始字节位置
    pub end: ByteIdx,    // 注解结束字节位置
}

impl Annotation {
    /// 将注解位置整体偏移，用于处理子串中的相对位置转换
    pub fn shift(&mut self, offset: ByteIdx) {
        self.start = self.start.saturating_add(offset);
        self.end = self.end.saturating_add(offset);
    }
}
