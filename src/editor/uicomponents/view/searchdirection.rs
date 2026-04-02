/// 搜索方向：向前或向后搜索
#[derive(Default, Eq, PartialEq, Clone, Copy)]
pub enum SearchDirection {
    #[default]
    Forward,   // 向前搜索
    Backward,  // 向后搜索
}
