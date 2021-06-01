use super::{State,Command};
use anyhow::Context;

mod model;
mod texture;
mod camera;


pub struct Graphic{
    window : winit::window::Window,

    size : winit::dpi::PhysicalSize<u32>,
    surface : wgpu::Surface,
    device : wgpu::Device,
    queue : wgpu::Queue,
    swap_chain_desc : wgpu::SwapChainDescriptor,
    swap_chain : wgpu::SwapChain,
    render_pipeline : wgpu::RenderPipeline
}


impl Command for Graphic{
    type Output = Self;
    type Input = winit::window::Window; //todo make a struct for setting graphic parameters

    fn run(options : Self::Input) -> anyhow::Result<Self::Output> {

        let state = futures::executor::block_on(Self::new(options));
        Ok(state?)
    }
}

//Private
impl Graphic{
    async fn new(window: winit::window::Window) -> anyhow::Result<Self>{

        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        let surface = unsafe {instance.create_surface(&window)};

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions{
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface)
        }).await
            .context("Failed to create graphic adapter")?;


        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor{
            label: Some("Request Device"),
            features: wgpu::Features::NON_FILL_POLYGON_MODE,
            limits: wgpu::Limits::default()
        },None).await?;;

        let swap_chain_desc = wgpu::SwapChainDescriptor{
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo
        };

        let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

        let shader_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor{
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("../../shader/shader.wgsl"))), //todo make include_str target shader folder
            flags: wgpu::ShaderFlags::all()
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[]
        });


        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: "main",
                buffers: &[]
            },
            fragment: Some(wgpu::FragmentState{
                module: &shader_module,
                entry_point: "main",
                targets: &
                    [
                        wgpu::ColorTargetState{
                            format: swap_chain_desc.format,
                            blend: Some(wgpu::BlendState{
                                color: wgpu::BlendComponent::REPLACE,
                                alpha: wgpu::BlendComponent::REPLACE
                            }),
                            write_mask: wgpu::ColorWrite::ALL
                        }
                    ]
            }),
            primitive: wgpu::PrimitiveState{
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                clamp_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false
            },
            depth_stencil: None, //todo add depth test
            multisample: wgpu::MultisampleState{
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
        });

        Ok(
            Self{
                window,
                size,
                surface,
                device,
                queue,
                swap_chain_desc,
                swap_chain,
                render_pipeline
            }
        )

    }
}

//Public
impl Graphic{
    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError>{
        let frame = self.swap_chain
            .get_current_frame()?.output;


        let mut render_command = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
            label: Some("Render Command")
        });
        
        
        {
            let mut render_pass = render_command.begin_render_pass(&wgpu::RenderPassDescriptor{
                label: Some("Render Pass"),
                color_attachments: &
                    [
                        wgpu::RenderPassColorAttachment{
                            view: &frame.view,
                            resolve_target: None,
                            ops: wgpu::Operations{
                                load: wgpu::LoadOp::Clear(wgpu::Color{
                                    r: 0.4,
                                    g: 0.3,
                                    b: 0.2,
                                    a: 1.0
                                }),
                                store: true
                            }
                        }
                    ],
                depth_stencil_attachment: None //todo add depth test
            });

            render_pass.set_pipeline(&self.render_pipeline);

        }

        self.queue.submit(std::iter::once(render_command.finish()));

        Ok(())
    }

    pub fn refresh(&mut self){
        Self::resize(self, self.size);
    }

    pub fn resize(&mut self, size : winit::dpi::PhysicalSize<u32>){
        if size.width != 0 && size.height != 0 {
            self.size = size;
            self.swap_chain_desc.width = self.size.width;
            self.swap_chain_desc.height = self.size.height;
            self.swap_chain = self.device.create_swap_chain(&self.surface, &self.swap_chain_desc);
        }
    }

    pub fn request_redraw(&self){
        self.window.request_redraw();

    }
}