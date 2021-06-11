use super::camera;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Lighting {
    direction: glam::Vec3,
    _padding: u32,
    ambient_color: glam::Vec4,
    diffuse_color: glam::Vec4,
    specular_color: glam::Vec4,
}

impl Lighting {
    pub fn new(
        direction: glam::Vec3,
        ambient_color: glam::Vec4,
        diffuse_color: glam::Vec4,
        specular_color: glam::Vec4,
    ) -> Self {
        Self {
            direction,
            _padding: 0,
            ambient_color,
            diffuse_color,
            specular_color,
        }
    }
}

pub struct Uniform {
    pub buffer: wgpu::Buffer,
    pub group: wgpu::BindGroup,
    pub group_layout: wgpu::BindGroupLayout,
    lighting: Vec<Lighting>, //currently internal
}

impl Uniform {
    //We are passing a collection of lighting that are copys which will not pass the actual lighting data into this function and lose ownership of the original.
    pub fn create(device: &wgpu::Device, lighting: Vec<Lighting>) -> Self {
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
            label: Some("lighting Buffer"),
            contents: &bytemuck::cast_slice(&lighting[..]),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST, //Copy DST for move lighting later
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("lighting Bind Group"),
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

    pub fn write_buffer(&mut self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.buffer, 0, &bytemuck::cast_slice(&self.lighting[..]));
    }
}

pub trait DrawLight<'a, 'b>
where
    'b: 'a,
{
    fn draw_light_mesh(&mut self, uniform: &'b camera::Uniform, light: &'b Uniform);
}

impl<'a, 'b> DrawLight<'a, 'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_light_mesh(&mut self, uniform: &'b camera::Uniform, light: &'b Uniform) {
        self.set_bind_group(0, &uniform.group, &[]);
        self.set_bind_group(1, &light.group, &[]);
        self.draw(0..3, 0..1);
    }
}
