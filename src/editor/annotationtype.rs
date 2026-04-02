#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AnnotationType{
    Match,
    SelectedMatch,
    Number,
    Keyword,
    Type,
    KnownValue,
    Char,
    LifetimeSpecifier,
}