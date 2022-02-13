#[derive(Copy, Clone, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Matrix4x4 {
    pub inner: [f32; 16],
}


impl Default for Matrix4x4 {
    fn default() -> Self {
        Self {
            inner: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
}


impl From<[f32; 16]> for Matrix4x4 {
    fn from(matrix: [f32; 16]) -> Self {
        Self { inner: matrix }
    }
}
