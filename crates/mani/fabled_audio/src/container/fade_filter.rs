#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FadeFilter {
    ABRUPT,
    FADE,
}

impl Default for FadeFilter {
    fn default() -> Self {
        Self::ABRUPT
    }
}
