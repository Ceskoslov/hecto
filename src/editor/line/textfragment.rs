use super::GraphemeWidth;
/// 字素簇片段：记录单个字素簇的所有显示信息
#[derive(Clone, Debug)]
pub struct TextFragment {
    pub grapheme: String,             // 原始字素簇字符串
    pub rendered_width: GraphemeWidth, // 显示宽度（半角/全角）
    pub replacement: Option<char>,    // 替换显示字符（用于 Tab、控制字符等）
    pub start: usize,                 // 在原始字符串中的字节偏移
}
