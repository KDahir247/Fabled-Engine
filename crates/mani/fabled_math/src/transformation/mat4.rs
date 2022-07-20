#[derive(Copy, Clone)]
pub struct Matrix4x4 {
    pub value: std::simd::Simd<f32, 16>,
}

#[rustfmt::skip]
impl Default for Matrix4x4 {
    fn default() -> Self {
        Self {
            value: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ]
            .into(),
        }
    }
}

impl From<[f32; 16]> for Matrix4x4 {
    fn from(matrix: [f32; 16]) -> Self {
        Self {
            value: matrix.into(),
        }
    }
}

pub struct SOAMatrix4x4<const N: usize> {
    pub value: [Matrix4x4; N],
}
