use serde::*;

#[derive(Copy, Clone, Deserialize, Serialize, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct TextureOptions {
    offset: [f32; 3],
    scale: [f32; 3],
    //turbulence: [f64; 3],
}

impl Default for TextureOptions {
    fn default() -> Self {
        Self {
            offset: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
            //turbulence: [0.0, 0.0, 0.0],
        }
    }
}
