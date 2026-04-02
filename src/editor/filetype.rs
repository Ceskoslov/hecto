use std::fmt::{Display, Formatter, Result};

#[derive(Default, Eq, PartialEq, Debug, Clone, Copy)]
pub enum FileType {
    Rust,
    #[default]
    Text,
}

impl Display for FileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            FileType::Rust => write!(f, "Rust"),
            FileType::Text => write!(f, "Text"),
        }
    }
}