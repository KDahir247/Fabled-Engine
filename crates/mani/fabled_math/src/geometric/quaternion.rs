#[derive(Copy, Clone, Debug)]
pub struct Quaternion {
    inner: [f32; 4],
}

impl Default for Quaternion {
    fn default() -> Self {
        Self {
            inner: [0.0, 0.0, 0.0, 1.0],
        }
    }
}
