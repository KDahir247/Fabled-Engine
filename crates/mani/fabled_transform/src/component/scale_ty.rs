#[derive(Copy, Clone, Debug)]
pub enum ScaleType {
    Uniform(f32),
    NonUniform([f32; 3]),
}

impl Default for ScaleType {
    fn default() -> Self {
        Self::Uniform(1.0)
    }
}
