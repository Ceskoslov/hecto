use super::AnnotationType;

/// 带注解字符串的片段：迭代器产生的单个元素
/// 每个片段包含一段文本及其可选的注解类型
#[derive(Debug)]
pub struct AnnotatedStringPart<'a> {
    pub string: &'a str,                        // 文本内容
    pub annotation_type: Option<AnnotationType>, // 注解类型（无则为普通文本）
}