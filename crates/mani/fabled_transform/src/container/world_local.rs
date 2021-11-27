#[derive(Copy, Clone, Debug)]
pub struct WorldToLocal {
    pub value: [f32; 16],
}

impl Default for WorldToLocal {
    fn default() -> Self {
        Self {
            value: [
                1.0, 0.0, 0.0, 0.0, // col 0
                0.0, 1.0, 0.0, 0.0, // col 1
                0.0, 0.0, 1.0, 0.0, // col 2
                0.0, 0.0, 0.0, 1.0, // col 3
            ],
        }
    }
}
