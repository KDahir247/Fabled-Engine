#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FlipAxis {
    Skip = 0,
    FlipX = 1,
    FlipY = 2,
}

impl Default for FlipAxis {
    fn default() -> Self {
        FlipAxis::Skip
    }
}
