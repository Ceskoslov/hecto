/// 字素簇显示宽度：半角（占 1 列）或全角（占 2 列）
#[derive(Debug, Clone, Copy)]
pub enum GraphemeWidth {
    Half,  // 半角字符，ASCII 等
    Full,  // 全角字符，中文等
}

impl From<GraphemeWidth> for usize {
    fn from(width: GraphemeWidth) -> Self {
        match width {
            GraphemeWidth::Half => 1,
            GraphemeWidth::Full => 2,
        }
    }
}
