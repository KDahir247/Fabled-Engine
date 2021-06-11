use super::Command;
use crate::core::graphic::light::DrawLight;
use anyhow::Context;
use model::{DrawModel, Vertex};
use wgpu::{Device, PipelineLayout, ShaderModule, SwapChainDescriptor};

mod camera;
mod constant;
mod light;
mod material;
mod model;
mod texture;

pub trait Binding {
    fn layout() -> wgpu::BindGroupLayout;
}

fn create_render_pipline(
    device: &Device,
    swap_chain_desc: &SwapChainDescriptor,
    shader_module: &ShaderModule,
    pipeline_layout: &PipelineLayout,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader_module,
            entry_point: "vs_main",
            buffers: &[model::VertexRaw::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader_module,
            entry_point: "fs_main",
            targets: &[wgpu::ColorTargetState {
                format: swap_chain_desc.format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrite::ALL,
            }],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            clamp_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
    })
}

pub struct Graphic {
    window: winit::window::Window,
    obj_model: Option<model::Model>,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    swap_chain_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    depth_texture: texture::Texture,
    material_layout: wgpu::BindGroupLayout,
    camera: camera::Camera,
    projection: camera::Projection,
    camera_controller: camera::CameraController,
    uniform: camera::Uniform,
    light: light::Uniform,
    render_pipeline: wgpu::RenderPipeline,
    light_render_pipeline: wgpu::RenderPipeline,
    mouse_pressed: bool,
}

//Trait
impl Command for Graphic {
    type Output = Self;
    type Input = winit::window::Window;

    fn run(options: Self::Input) -> anyhow::Result<Self::Output> {
        futures::executor::block_on(Self::new(options))
    }
}

//Private
impl Graphic {
    async fn new(window: winit::window::Window) -> anyhow::Result<Self> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        let surface = unsafe { instance.create_surface(&window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .context("Failed to create graphic adapter")?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Request Device"),
                    features: wgpu::Features::NON_FILL_POLYGON_MODE,
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await?;

        let swap_chain_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate,
        };

        let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

        let camera = camera::Camera::new(
            glam::Vec3::new(0.0, 2.0, 6.0),
            glam::Vec3::new(0.0, -90.0, 0.0),
        );
        let projection =
            camera::Projection::new(size.width as f32 / size.height as f32, 60.0, 0.1, 100.0);
        let camera_controller = camera::CameraController::new(1.0, 0.5, 15.0);
        let camera_uniform = camera::Uniform::create(&device, &camera, &projection);

        let depth_texture = texture::Texture::create_depth_texture(&device, size);
        let material_layout = model::Material::create_material_layout(&device);

        let lighting = light::Lighting::new(
            glam::vec3(0.5, 0.0, 0.0),
            glam::vec4(1.0, 1.0, 1.0, 0.4),
            glam::vec4(1.0, 0.0, 0.0, 1.0),
            glam::vec4(1.0, 0.0, 0.0, 1.0),
        );

        let lighting_uniform = light::Uniform::create(&device, vec![lighting]);

        let render_pipeline = {
            //Shader Module
            let shader_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some("Core Shader Module"),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                    "../../shader/shader.wgsl"
                ))),
                flags: wgpu::ShaderFlags::all(),
            });

            //Pipeline Layout
            let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Core Pipeline Layout"),
                bind_group_layouts: &[
                    &material_layout,
                    &camera_uniform.group_layout,
                    &lighting_uniform.group_layout,
                ],
                push_constant_ranges: &[],
            });

            //RenderPipeline
            create_render_pipline(&device, &swap_chain_desc, &shader_module, &pipeline_layout)
        };

        //for debugging
        let light_render_pipeline = {
            //Shader
            let shader_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: Some("Debug Lighting Shader Module"),
                source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                    "../../shader/lighting_debug.wgsl"
                ))),
                flags: wgpu::ShaderFlags::all(),
            });
            //Pipeline Layout
            let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Debug Lighting Pipeline Layout"),
                bind_group_layouts: &[&camera_uniform.group_layout, &lighting_uniform.group_layout],
                push_constant_ranges: &[],
            });

            //RenderPipeline
            create_render_pipline(&device, &swap_chain_desc, &shader_module, &pipeline_layout)
        };

        Ok(Self {
            window,
            obj_model: None,
            size,
            surface,
            device,
            queue,
            swap_chain_desc,
            swap_chain,
            depth_texture,
            material_layout,
            camera,
            projection,
            camera_controller,
            uniform: camera_uniform,
            light: lighting_uniform,
            render_pipeline,
            light_render_pipeline,
            mouse_pressed: false,
        })
    }
}

//Public
impl Graphic {
    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;

        let mut render_command =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Command"),
                });

        {
            let mut render_pass = render_command.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None, //todo here
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: constant::CLEAR_COLOR,
                            g: constant::CLEAR_COLOR,
                            b: constant::CLEAR_COLOR,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.render_pipeline);

            if self.obj_model.is_some() {
                render_pass.draw_model(
                    self.obj_model.as_ref().unwrap(),
                    &self.uniform,
                    &self.light,
                );

                render_pass.set_pipeline(&self.light_render_pipeline);
                render_pass.draw_light_mesh(&self.uniform, &self.light);
            }
        }

        self.queue.submit(std::iter::once(render_command.finish()));

        Ok(())
    }

    pub fn refresh(&mut self) {
        Graphic::resize(self, self.size);
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        if size.width != 0 && size.height != 0 {
            self.size = size;
            self.swap_chain_desc.width = self.size.width;
            self.swap_chain_desc.height = self.size.height;
            self.swap_chain = self
                .device
                .create_swap_chain(&self.surface, &self.swap_chain_desc);

            self.projection.resize(size);
            self.uniform
                .update_view_proj(&self.camera, &self.projection);

            self.depth_texture = texture::Texture::create_depth_texture(&self.device, self.size);
        }
    }

    pub fn input(&mut self, event: winit::event::DeviceEvent) {
        match event {
            winit::event::DeviceEvent::Key(winit::event::KeyboardInput {
                virtual_keycode: Some(key),
                state,
                ..
            }) => {
                self.camera_controller.process_keyboard(key, state);
            }

            winit::event::DeviceEvent::MouseMotion { delta } => {
                if self.mouse_pressed {
                    self.camera_controller.process_mouse(delta.0, delta.1);
                }
            }
            winit::event::DeviceEvent::MouseWheel { delta } => {
                self.camera_controller.process_scroll(delta);
            }
            winit::event::DeviceEvent::Button { button: 1, state } => {
                self.mouse_pressed = state == winit::event::ElementState::Pressed;
            }
            _ => {}
        }
    }

    pub fn update(&mut self, dt: std::time::Duration) {
        self.camera_controller
            .update_camera_proj(&mut self.camera, &mut self.projection, dt);
        self.uniform
            .update_view_proj(&self.camera, &self.projection);
        self.uniform.write_buffer(&self.queue);
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn load_obj<P: AsRef<std::path::Path>>(&mut self, path: P) {
        let obj_model =
            model::Model::load(&self.device, &self.queue, path, &self.material_layout).unwrap();
        self.obj_model = Some(obj_model);
    }
}
