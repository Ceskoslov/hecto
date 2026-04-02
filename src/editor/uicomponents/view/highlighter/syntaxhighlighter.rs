use super::Annotation;
use super::Line;
use crate::prelude::*;

/// 语法高亮器 trait：定义高亮器的统一接口
/// 搜索结果高亮和语法高亮都实现这个 trait
pub trait SyntaxHighlighter {
    /// 对指定行进行高亮分析
    fn highlight(&mut self, idx: LineIdx, line: &Line);
    /// 获取指定行的注解列表
    fn get_annotations(&self, idx: LineIdx) -> Option<&Vec<Annotation>>;
}