#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FlipAxis {
    FlipX = 0,
    FlipY = 1,
    FlipZ = 2,
}

impl Default for FlipAxis {
    fn default() -> Self {
        FlipAxis::FlipY
    }
}
