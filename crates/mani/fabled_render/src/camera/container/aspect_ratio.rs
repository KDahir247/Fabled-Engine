#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AspectRatio {
    pub horizontal: f32,
    pub vertical: f32,
}

impl Default for AspectRatio {
    fn default() -> Self {
        Self {
            horizontal: 16.0,
            vertical: 9.0,
        }
    }
}

impl AspectRatio {
    pub fn new(horizontal: f32, vertical: f32) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }
}
