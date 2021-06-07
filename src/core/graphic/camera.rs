use super::constant;
use cgmath::{InnerSpace, SquareMatrix};
use wgpu::util::DeviceExt;

//to get up use forward.cross(right);
pub struct Camera {
    pub position: cgmath::Point3<f32>,
    forward: cgmath::Vector3<f32>,
    right: cgmath::Vector3<f32>,
    yaw: cgmath::Rad<f32>,
    pitch: cgmath::Rad<f32>,
}

impl Camera {
    pub fn new<
        V: Into<cgmath::Point3<f32>>,
        Y: Into<cgmath::Rad<f32>>,
        P: Into<cgmath::Rad<f32>>,
    >(
        position: V,
        yaw: Y,
        pitch: P,
    ) -> Camera {
        Self {
            position: position.into(),
            forward: cgmath::Vector3::unit_z() * -1.0,
            right: cgmath::Vector3::unit_x(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }

    pub fn calc_matrix(&self) -> cgmath::Matrix4<f32> {
        cgmath::Matrix4::look_to_rh(self.position, self.forward, cgmath::Vector3::unit_y())
    }
}

pub struct Projection {
    aspect: f32,
    fovy: cgmath::Rad<f32>,
    znear: f32,
    zfar: f32,
}

impl Projection {
    pub fn new<F: Into<cgmath::Rad<f32>>>(
        aspect: f32,
        fovy: F,
        znear: f32,
        zfar: f32,
    ) -> Projection {
        Self {
            aspect,
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }

    pub fn calc_matrix(&self) -> cgmath::Matrix4<f32> {
        constant::OPENGL_TO_WGPU_MATRIX
            * cgmath::perspective(self.fovy, self.aspect, self.znear, self.zfar)
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.aspect = size.width as f32 / size.height as f32
    }
}

pub struct CameraController {
    //Forward,   Right,   Up
    //Backward,  Left,    Down
    //Scalar,    Scalar,  Scalar
    amount_matrix: cgmath::Matrix3<f32>,

    //Pitch, Yaw, Roll, Scalar
    rotation: cgmath::Vector4<f32>,

    scroll: f32,
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> CameraController {
        Self {
            amount_matrix: cgmath::Matrix3::from_cols(
                cgmath::Vector3::unit_z() * speed,
                cgmath::Vector3::unit_z() * speed,
                cgmath::Vector3::unit_z() * speed,
            ),
            rotation: cgmath::Vector4::unit_w() * sensitivity,
            scroll: 0.0,
        }
    }

    pub fn process_keyboard(
        &mut self,
        key: winit::event::VirtualKeyCode,
        state: winit::event::ElementState,
    ) {
        let amount = if state == winit::event::ElementState::Pressed {
            1.0
        } else {
            0.0
        };

        match key {
            winit::event::VirtualKeyCode::W | winit::event::VirtualKeyCode::Up => {
                self.amount_matrix.x.x = amount * self.amount_matrix.y.z;
            }
            winit::event::VirtualKeyCode::S | winit::event::VirtualKeyCode::Down => {
                self.amount_matrix.x.y = amount * self.amount_matrix.y.z;
            }
            winit::event::VirtualKeyCode::D | winit::event::VirtualKeyCode::Right => {
                self.amount_matrix.y.x = amount * self.amount_matrix.y.z;
            }
            winit::event::VirtualKeyCode::A | winit::event::VirtualKeyCode::Left => {
                self.amount_matrix.y.y = amount * self.amount_matrix.y.z;
            }
            _ => {}
        }
    }

    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.rotation.x = -mouse_dy as f32 * self.rotation.w;
        self.rotation.y = mouse_dx as f32 * self.rotation.w;
    }

    pub fn process_scroll(&mut self, delta: &winit::event::MouseScrollDelta) {
        //todo change fovy
    }

    pub fn update_camera(&mut self, camera: &mut Camera, dt: std::time::Duration) {
        let dt = dt.as_secs_f32();

        let (yaw_sin, yaw_cos) = camera.yaw.0.sin_cos();
        let pitch_sin = camera.pitch.0.sin();

        camera.forward = cgmath::Vector3 {
            x: yaw_cos,
            y: pitch_sin,
            z: yaw_sin,
        }
        .normalize();

        camera.right = cgmath::Vector3 {
            x: -yaw_sin,
            y: 0.0,
            z: yaw_cos,
        }
        .normalize();

        camera.position += camera.forward * (self.amount_matrix.x.x - self.amount_matrix.x.y) * dt;
        camera.position += camera.right * (self.amount_matrix.y.x - self.amount_matrix.y.y) * dt;

        camera.yaw += cgmath::Rad(self.rotation.y) * dt;
        camera.pitch += cgmath::Rad(self.rotation.x) * dt;

        self.rotation = self.rotation.w * cgmath::Vector4::unit_w();

        if camera.pitch < -cgmath::Rad(std::f32::consts::FRAC_PI_2) {
            camera.pitch = -cgmath::Rad(std::f32::consts::FRAC_PI_2);
        } else if camera.pitch > cgmath::Rad(std::f32::consts::FRAC_PI_2) {
            camera.pitch = cgmath::Rad(std::f32::consts::FRAC_PI_2);
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct UniformLayout {
    view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
}

impl UniformLayout {
    fn new() -> Self {
        Self {
            view_position: [0.0f32; 4],
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    fn update_view_proj(&mut self, camera: &Camera, proj: &Projection) {
        self.view_position = camera.position.to_homogeneous().into();
        self.view_proj = (proj.calc_matrix() * camera.calc_matrix()).into();
    }
}

pub struct Uniform {
    pub buffer: wgpu::Buffer,
    pub group: wgpu::BindGroup,
    pub group_layout: wgpu::BindGroupLayout,
    layout: UniformLayout, // internal camera matrix container
}

impl Uniform {
    pub fn create(device: &wgpu::Device, camera: &Camera, projection: &Projection) -> Self {
        let mut uniform_layout = UniformLayout::new();
        uniform_layout.update_view_proj(camera, projection);

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
            contents: &bytemuck::cast_slice(&[uniform_layout]),
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

    pub fn update_view_proj(&mut self, camera: &Camera, projection: &Projection) {
        self.layout.update_view_proj(camera, projection);
    }

    pub fn write_buffer(&mut self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.buffer, 0, &bytemuck::cast_slice(&[self.layout]));
    }
}
