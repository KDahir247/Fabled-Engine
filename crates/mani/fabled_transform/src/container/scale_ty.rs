#[derive(Copy, Clone, Debug)]
pub enum Scale {
    Uniform(f32),
    NonUniform([f32; 3]),
}
