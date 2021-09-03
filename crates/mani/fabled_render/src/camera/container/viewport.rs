//todo will be convert to a Rect when implemented in the math module

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ViewPort {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Default for ViewPort {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 1.0,
            h: 1.0,
        }
    }
}
