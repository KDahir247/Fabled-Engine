//The method are only used to construct the component.
use crate::util::camera::{calc_camera_matrix, calc_proj_matrix};
use wgpu::util::DeviceExt;

pub struct Camera {
    pub orientation: CameraOrientation,
    pub projection: Projection,
    pub uniform: CameraUniform,
}

//to get up use forward.cross(right);
pub struct CameraOrientation {
    pub forward: glam::Vec3,
    pub right: glam::Vec3,

    //X, Y, Z
    pub position: glam::Vec3,
    //Pitch, Yaw, Roll
    pub rotation: glam::Vec3,
    //X, Y, Z
    #[allow(dead_code)]
    pub scale: glam::Vec3,
}

pub struct Projection {
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

pub struct CameraController {
    //Forward,   Right,   Up
    //Backward,  Left,    Down
    //Scalar,    Scalar,  Scalar
    pub amount_matrix: glam::Mat3,

    //Pitch, Yaw, Roll, Scalar
    pub amount_rotation: glam::Vec4,

    //Amount, Scalar
    pub amount_scroll: glam::Vec2,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraRaw {
    pub view_position: glam::Vec4,
    pub view_proj: glam::Mat4,
}

pub struct CameraUniform {
    pub buffer: wgpu::Buffer,
    pub group: wgpu::BindGroup,
    pub group_layout: wgpu::BindGroupLayout,
    pub raw: CameraRaw, // internal camera matrix container
}

impl CameraUniform {
    pub fn create(
        device: &wgpu::Device,
        orientation: &CameraOrientation,
        projection: &Projection,
    ) -> Self {
        let mut uniform_layout = CameraRaw {
            view_position: glam::Vec4::ZERO,
            view_proj: glam::Mat4::IDENTITY,
        };

        uniform_layout.view_position = glam::vec3(
            orientation.position.x,
            orientation.position.y,
            orientation.position.z,
        )
        .extend(1.0);
        uniform_layout.view_proj = calc_proj_matrix(projection) * calc_camera_matrix(orientation);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera Uniform Layout"),
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

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniform_layout]),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Uniform Group"),
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
            raw: uniform_layout,
        }
    }
}
