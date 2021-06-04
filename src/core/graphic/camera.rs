use super::constant;
use cgmath::SquareMatrix;
use wgpu::util::DeviceExt;

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    fn build_view_prj_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        proj * view
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct UniformLayout {
    view_proj: [[f32; 4]; 4],
}

impl UniformLayout {
    fn new() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = (constant::OPENGL_TO_WGPU_MATRIX * camera.build_view_prj_matrix()).into();
    }
}

pub struct Uniform {
    pub buffer: wgpu::Buffer,
    pub group: wgpu::BindGroup,
    pub group_layout: wgpu::BindGroupLayout,
    layout: UniformLayout, // internal camera matrix container
}

impl Uniform {
    pub fn create(device: &wgpu::Device, camera: &Camera) -> Self {
        let mut uniform_layout = UniformLayout::new();
        uniform_layout.update_view_proj(camera);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Uniform Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: &bytemuck::cast_slice(&uniform_layout.view_proj),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        Self {
            buffer: uniform_buffer,
            group: bind_group,
            group_layout: bind_group_layout,
            layout: uniform_layout,
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.layout.update_view_proj(camera);
    }

    pub fn write_buffer(&mut self, queue: &wgpu::Queue) {
        queue.write_buffer(
            &self.buffer,
            0,
            &bytemuck::cast_slice(&self.layout.view_proj),
        );
    }
}
