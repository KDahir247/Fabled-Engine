#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WrapMode {
    Repeat,
    ClampToEdge,
}
