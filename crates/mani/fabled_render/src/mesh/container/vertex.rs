#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub tangent: [f32; 4],
    pub bi_tangent: [f32; 4],

    // todo make position and normal of [f32; 3] so the alignment works and normalize remove
    //  tangent and bi-tangent to a separate struct
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coord: [f32; 2],
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: [0.0; 3],
            normal: [0.0; 3],
            tangent: [0.0; 4],
            bi_tangent: [0.0; 4],
            tex_coord: [0.0; 2],
        }
    }
}


impl Vertex {
    pub const fn init() -> Self {
        Self {
            position: [0.0; 3],
            normal: [0.0; 3],
            tangent: [0.0; 4],
            bi_tangent: [0.0; 4],
            tex_coord: [0.0; 2],
        }
    }
}
