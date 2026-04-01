#[derive(Debug, Clone, Copy)]
pub enum GraphemeWidth {
    Half,
    Full,
}

impl From<GraphemeWidth> for usize {
    fn from(width: GraphemeWidth) -> Self {
        match width {
            GraphemeWidth::Half => 1,
            GraphemeWidth::Full => 2,
        }
    }
}
