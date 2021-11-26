#[derive(Copy, Clone, Debug)]
pub struct Rotation {
    pub value: [f32; 4],
}

impl Default for Rotation {
    fn default() -> Self {
        Self {
            value: [0.0, 0.0, 0.0, 1.0],
        }
    }
}
