#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Lighting {
    position: glam::Vec3,
    _padding: u32,
    color: glam::Vec3,
}

impl Lighting {
    pub fn new(position: glam::Vec3, color: glam::Vec3) -> Self {
        Self {
            position,
            _padding: 0,
            color,
        }
    }
}

pub struct Uniform {
    pub buffer: wgpu::Buffer,
    pub group: wgpu::BindGroup,
    pub group_layout: wgpu::BindGroupLayout,
    layout: Lighting,
}
