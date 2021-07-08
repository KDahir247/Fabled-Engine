//The method are only used to construct the component.
use crate::util::camera::{calc_camera_matrix, calc_proj_matrix};
use glam::Vec4Swizzles;
use wgpu::util::DeviceExt;

pub struct Camera {
    pub orientation: CameraOrientation,
    pub projection: Projection,
    pub uniform: CameraUniform,
}

//to get up use forward.cross(right);
pub struct CameraOrientation {
    pub transformation_matrix: glam::Mat4,
    pub forward: glam::Vec4,
    pub right: glam::Vec4,
}

pub struct Projection {
    pub fovy: f32,
    pub aspect: f32,
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
    pub view: glam::Mat4,
    pub proj: glam::Mat4,
    pub inv_proj: glam::Mat4,
}

pub struct CameraUniform {
    pub raw: CameraRaw, // internal camera matrix container
    pub buffer: wgpu::Buffer,
    pub group: wgpu::BindGroup,
    pub group_layout: wgpu::BindGroupLayout,
}

impl CameraUniform {
    pub fn create(
        device: &wgpu::Device,
        orientation: &CameraOrientation,
        projection: &Projection,
    ) -> Self {
        let mut uniform_layout = CameraRaw {
            view_position: glam::Vec4::ZERO,
            view: Default::default(),
            proj: Default::default(),
            inv_proj: Default::default(),
        };

        uniform_layout.view_position = orientation.transformation_matrix.w_axis.xyz().extend(1.0);
        //Mvp = PM-1 camera
        //Mmvp = PM-1 camera * M object

        uniform_layout.proj = calc_proj_matrix(projection);
        uniform_layout.view = calc_camera_matrix(orientation);
        uniform_layout.inv_proj = uniform_layout.proj.inverse();

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
