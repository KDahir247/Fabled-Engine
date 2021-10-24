use serde::*;

#[derive(
    Debug, Copy, Clone, PartialEq, Deserialize, Serialize, bytemuck::Pod, bytemuck::Zeroable,
)]
#[repr(C)]
pub struct TextureOptions {
    pub offset: [f32; 3],
    pub scale: [f32; 3],
}

impl Default for TextureOptions {
    fn default() -> Self {
        Self {
            offset: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
        }
    }
}
