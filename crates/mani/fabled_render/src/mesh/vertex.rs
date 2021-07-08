#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: glam::Vec3,
    pub tex_coord: glam::Vec2,
    pub normal: glam::Vec3,
    pub tangent: glam::Vec4,
    pub bi_tangent: glam::Vec4,
}
