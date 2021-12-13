// todo remove
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(align(16))]
pub struct Orientation {
    pub forward: [f32; 3],
    pub right: [f32; 3],
}

impl Default for Orientation {
    fn default() -> Self {
        Self {
            forward: [0.0, 0.0, 1.0],
            right: [1.0, 0.0, 0.0],
        }
    }
}
