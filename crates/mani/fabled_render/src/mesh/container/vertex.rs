#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub tangent: [f32; 4],    // x, y, z, w
    pub bi_tangent: [f32; 4], // x, y, z, w
    pub position: [f32; 3],   // x, y, z
    pub tex_coord: [f32; 2],  // u, v
    pub normal: [f32; 3],     // x, y, z
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: [0.0; 3],
            tex_coord: [0.0; 2],
            normal: [0.0; 3],
            tangent: [0.0; 4],
            bi_tangent: [0.0; 4],
        }
    }
}
