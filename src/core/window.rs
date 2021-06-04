use super::{graphic, Command};

pub struct Window;

impl Command for Window {
    type Output = ();
    type Input = (graphic::Graphic, winit::event_loop::EventLoop<()>);

    fn run(options: Self::Input) -> anyhow::Result<Self::Output> {
        let (graphic, event_loop) = options;

        Window::handle(graphic, event_loop);

        Ok(())
    }
}

impl Window {
    fn handle(mut graphic: graphic::Graphic, event_loop: winit::event_loop::EventLoop<()>) {
        event_loop.run(move |evt, _, control_flow| match evt {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::Resized(size) => {
                    graphic.resize(size);
                }

                winit::event::WindowEvent::CloseRequested => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }

                winit::event::WindowEvent::DroppedFile(file_path) => {
                    graphic.load_obj(file_path.as_path());
                }

                winit::event::WindowEvent::KeyboardInput {
                    input:
                        winit::event::KeyboardInput {
                            virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                            state: winit::event::ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }

                winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    graphic.resize(*new_inner_size)
                }

                _ => {}
            },

            winit::event::Event::RedrawRequested(_) => {
                graphic.update();
                match graphic.render() {
                    Ok(_) => {}
                    Err(wgpu::SwapChainError::Lost) => graphic.refresh(),
                    Err(wgpu::SwapChainError::OutOfMemory) => {
                        *control_flow = winit::event_loop::ControlFlow::Exit;
                    }
                    Err(err) => {
                        eprintln!("SwapChainError : {:?}", err);
                    }
                }
            }

            winit::event::Event::RedrawEventsCleared => {
                graphic.request_redraw();
            }

            _ => {}
        })
    }
}
