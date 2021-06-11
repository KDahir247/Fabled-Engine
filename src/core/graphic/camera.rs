use super::constant;
use wgpu::util::DeviceExt;
use winit::event::MouseScrollDelta;

//to get up use forward.cross(right);
pub struct Camera {
    forward: glam::Vec3,
    right: glam::Vec3,

    //X, Y, Z
    pub position: glam::Vec3,
    //Pitch, Yaw, Roll
    rotation: glam::Vec3,
    //X, Y, Z
    scale: glam::Vec3,
}

impl Camera {
    pub fn new<V: Into<glam::Vec3>, PYR: Into<glam::Vec3>>(position: V, rotation: PYR) -> Camera {
        Self {
            forward: glam::Vec3::Z * -1.0,
            right: glam::Vec3::X,
            position: position.into(),
            rotation: rotation.into() * (std::f32::consts::PI / 180.0_f32),
            scale: glam::Vec3::ONE,
        }
    }

    pub fn calc_matrix(&self) -> glam::Mat4 {
        let f = self.forward.normalize();
        let s = f.cross(glam::Vec3::Y).normalize();
        let u = s.cross(f);

        glam::mat4(
            glam::vec4(s.x, u.x, -f.x, 0.0),
            glam::vec4(s.y, u.y, -f.y, 0.0),
            glam::vec4(s.z, u.z, -f.z, 0.0),
            glam::vec4(
                -self.position.dot(s),
                -self.position.dot(u),
                self.position.dot(f),
                1.0,
            ),
        )
    }
}

pub struct Projection {
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Projection {
    pub fn new(aspect: f32, fovy: f32, znear: f32, zfar: f32) -> Projection {
        Self {
            aspect,
            fovy: fovy.to_radians(),
            znear,
            zfar,
        }
    }

    pub fn calc_matrix(&self) -> glam::Mat4 {
        constant::OPENGL_TO_WGPU_MATRIX
            * glam::Mat4::perspective_rh_gl(self.fovy, self.aspect, self.znear, self.zfar)
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.aspect = size.width as f32 / size.height as f32
    }
}

pub struct CameraController {
    //Forward,   Right,   Up
    //Backward,  Left,    Down
    //Scalar,    Scalar,  Scalar
    amount_matrix: glam::Mat3,

    //Pitch, Yaw, Roll, Scalar
    amount_rotation: glam::Vec4,

    //Amount, Scalar
    amount_scroll: glam::Vec2,
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32, scroll_factor: f32) -> CameraController {
        Self {
            amount_matrix: glam::Mat3::from_cols(
                glam::Vec3::Z * speed,
                glam::Vec3::Z * speed,
                glam::Vec3::Z * speed,
            ),
            amount_rotation: glam::Vec4::W * sensitivity,
            amount_scroll: glam::Vec2::Y * scroll_factor,
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
                self.amount_matrix.x_axis.x = amount * self.amount_matrix.x_axis.z;
            }
            winit::event::VirtualKeyCode::S | winit::event::VirtualKeyCode::Down => {
                self.amount_matrix.x_axis.y = amount * self.amount_matrix.x_axis.z;
            }
            winit::event::VirtualKeyCode::D | winit::event::VirtualKeyCode::Right => {
                self.amount_matrix.y_axis.x = amount * self.amount_matrix.y_axis.z;
            }
            winit::event::VirtualKeyCode::A | winit::event::VirtualKeyCode::Left => {
                self.amount_matrix.y_axis.y = amount * self.amount_matrix.y_axis.z;
            }
            winit::event::VirtualKeyCode::Q => {
                self.amount_matrix.z_axis.x = amount * self.amount_matrix.z_axis.z;
            }
            winit::event::VirtualKeyCode::E => {
                self.amount_matrix.z_axis.y = amount * self.amount_matrix.z_axis.z;
            }
            _ => {}
        }
    }

    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.amount_rotation.x = -mouse_dy as f32 * self.amount_rotation.w;
        self.amount_rotation.y = mouse_dx as f32 * self.amount_rotation.w;
    }

    pub fn process_scroll(&mut self, delta: winit::event::MouseScrollDelta) {
        match delta {
            MouseScrollDelta::LineDelta(_, y_position) => {
                self.amount_scroll.x = y_position * self.amount_scroll.y
            }

            MouseScrollDelta::PixelDelta(_) => {
                eprintln!("Touch pad or other device that use PixelDelta is not supported on this application");
                //todo
            }
        }
    }

    pub fn update_camera_proj(
        &mut self,
        camera: &mut Camera,
        proj: &mut Projection,
        dt: std::time::Duration,
    ) {
        let dt = dt.as_secs_f32();

        //Camera
        let (yaw_sin, yaw_cos) = camera.rotation.y.sin_cos();
        let pitch_sin = camera.rotation.x.sin();

        camera.forward = glam::Vec3::new(yaw_cos, pitch_sin, yaw_sin).normalize();

        camera.right = glam::Vec3::new(-yaw_sin, 0.0, yaw_cos).normalize();

        camera.position +=
            camera.forward * (self.amount_matrix.x_axis.x - self.amount_matrix.x_axis.y) * dt;

        camera.position +=
            camera.right * (self.amount_matrix.y_axis.x - self.amount_matrix.y_axis.y) * dt;

        camera.position.y += (self.amount_matrix.z_axis.x - self.amount_matrix.z_axis.y) * dt;

        camera.rotation.x += self.amount_rotation.x * dt;
        camera.rotation.y += self.amount_rotation.y * dt;

        self.amount_rotation = self.amount_rotation.w * glam::Vec4::W;

        if camera.rotation.x < -std::f32::consts::FRAC_2_PI {
            camera.rotation.x = -std::f32::consts::FRAC_2_PI;
        } else if camera.rotation.x > std::f32::consts::FRAC_2_PI {
            camera.rotation.x = std::f32::consts::FRAC_2_PI;
        }

        //Projection
        proj.fovy += -self.amount_scroll.x * dt;

        if proj.fovy < 1.0_f32.to_radians() {
            proj.fovy = 1.0_f32.to_radians();
        } else if proj.fovy > 179.0_f32.to_radians() {
            proj.fovy = 179.0_f32.to_radians();
        }

        self.amount_scroll.x = 0.0;
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraRaw {
    view_position: glam::Vec4,
    view_proj: glam::Mat4,
}

impl CameraRaw {
    fn new() -> Self {
        Self {
            view_position: glam::Vec4::ZERO,
            view_proj: glam::Mat4::IDENTITY,
        }
    }

    fn update_view_proj(&mut self, camera: &Camera, proj: &Projection) {
        self.view_position =
            glam::Vec4::new(camera.position.x, camera.position.y, camera.position.z, 1.0);
        self.view_proj = proj.calc_matrix() * camera.calc_matrix();
    }
}

pub struct Uniform {
    pub buffer: wgpu::Buffer,
    pub group: wgpu::BindGroup,
    pub group_layout: wgpu::BindGroupLayout,
    layout: CameraRaw, // internal camera matrix container
}

impl Uniform {
    pub fn create(device: &wgpu::Device, camera: &Camera, projection: &Projection) -> Self {
        let mut uniform_layout = CameraRaw::new();
        uniform_layout.update_view_proj(camera, projection);

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
            contents: &bytemuck::cast_slice(&[uniform_layout]),
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
