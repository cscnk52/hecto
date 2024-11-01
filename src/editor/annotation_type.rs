#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum AnnotationType {
    Match,
    SelectedMatch,
    Number,
    KeyWord,
    Type,
    KnownValue,
    Char,
    LifetimeSpecifier,
}
