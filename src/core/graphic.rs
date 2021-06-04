use super::Command;
use anyhow::Context;
use model::{DrawModel, Vertex};

mod camera;
mod constant;
mod light;
mod model;
mod texture;

pub trait Binding {
    fn layout() -> wgpu::BindGroupLayout;
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
    tex_layout: wgpu::BindGroupLayout,
    camera: camera::Camera,
    uniform: camera::Uniform,
    render_pipeline: wgpu::RenderPipeline,
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

        let camera = camera::Camera {
            eye: cgmath::Point3 {
                x: 0.0,
                y: 2.0,
                z: 10.0,
            },
            target: cgmath::Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            up: cgmath::Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            aspect: swap_chain_desc.width as f32 / swap_chain_desc.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let uniform = camera::Uniform::create(&device, &camera);

        let depth_texture = texture::Texture::create_depth_texture(&device, size);

        let tex_layout = model::Material::create_tex_layout(&device);

        let shader_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                "../../shader/shader.wgsl"
            ))),
            flags: wgpu::ShaderFlags::all(),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&tex_layout, &uniform.group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: "vs_main",
                buffers: &[model::ModelVertex::desc()],
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
        });

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
            tex_layout,
            camera,
            uniform,
            render_pipeline,
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
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.4,
                            g: 0.3,
                            b: 0.2,
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
                render_pass.draw_model(self.obj_model.as_ref().unwrap(), &self.uniform);
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

            self.camera.resize(size.width, size.height);
            self.uniform.update_view_proj(&self.camera);

            self.depth_texture = texture::Texture::create_depth_texture(&self.device, self.size);
        }
    }

    pub fn update(&mut self) {
        self.uniform.update_view_proj(&self.camera); //used for later if the camera is dynamic
        self.uniform.write_buffer(&self.queue);
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn load_obj<P: AsRef<std::path::Path>>(&mut self, path: P) {
        let obj_model =
            model::Model::load(&self.device, &self.queue, path, &self.tex_layout).unwrap();
        self.obj_model = Some(obj_model);
    }
}
