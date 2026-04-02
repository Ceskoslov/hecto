/// 尺寸：表示终端或 UI 组件的宽高
#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}