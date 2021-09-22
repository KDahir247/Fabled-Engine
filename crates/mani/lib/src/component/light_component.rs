use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightColorRaw {
    pub ambient_color: glam::Vec4,
    pub diffuse_color: glam::Vec4,
    pub specular_color: glam::Vec4,
    pub emissive_color: glam::Vec4,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightRaw {
    pub position: glam::Vec3, // 12 bytes
    pub __padding__: u32,     // 4 bytes
    pub color: LightColorRaw, // 64 bytes
}

pub struct LightUniform {
    pub buffer: wgpu::Buffer,
    pub group: wgpu::BindGroup,
    pub group_layout: wgpu::BindGroupLayout,
    pub lighting: LightRaw,
}

impl LightUniform {
    pub fn create(device: &wgpu::Device, lighting: LightRaw) -> Self {
        let bind_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Lighting Uniform Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let light_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Lighting Buffer"),
            contents: bytemuck::cast_slice(&[lighting]),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Lighting Group"),
            layout: &bind_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: light_buffer.as_entire_binding(),
            }],
        });

        Self {
            buffer: light_buffer,
            group: bind_group,
            group_layout: bind_layout,
            lighting,
        }
    }
}
