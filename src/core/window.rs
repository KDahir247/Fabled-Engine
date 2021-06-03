use super::{Command, graphic};
use winit::event::{Event, WindowEvent, KeyboardInput};

pub struct Window;

impl Command for Window{
    type Output = ();
    type Input = (graphic::Graphic, winit::event_loop::EventLoop<()>);

    fn run(options: Self::Input) -> anyhow::Result<Self::Output> {

        let (graphic, event_loop) = options;

        Self::new(graphic, event_loop);

        Ok(())
    }
}

impl Window{

    fn new(mut graphic: graphic::Graphic, event_loop : winit::event_loop::EventLoop<()>) {

        event_loop.run(move|evt, _, control_flow|{

            match evt{
                Event::WindowEvent { event, ..} => {

                        match event{
                            WindowEvent::Resized(size) => {
                               graphic.resize(size);
                            }
                            WindowEvent::CloseRequested => {
                                *control_flow = winit::event_loop::ControlFlow::Exit;
                            }
                            WindowEvent::DroppedFile(file_path) => {
                                graphic.load_obj(file_path.as_path());
                            }
                            WindowEvent::KeyboardInput { input, .. } => {
                                match input{
                                    KeyboardInput {virtual_keycode : Some(winit::event::VirtualKeyCode::Escape), state : winit::event::ElementState::Pressed, .. } => {
                                        *control_flow = winit::event_loop::ControlFlow::Exit;
                                    }
                                    _ => {}
                                }
                            }
                            WindowEvent::ScaleFactorChanged {new_inner_size, .. } => {
                                graphic.resize(*new_inner_size)
                            }
                            _ => {}
                        }
                }

                Event::RedrawRequested(_) => {
                    graphic.update();
                    match graphic.render(){
                        Ok(_) => {}
                        Err(wgpu::SwapChainError::Lost) =>{
                            graphic.refresh()
                        }
                        Err(wgpu::SwapChainError::OutOfMemory) => {
                            *control_flow = winit::event_loop::ControlFlow::Exit;
                        }
                        Err(err) => {
                            eprintln!("SwapChainError : {}", err);
                        }
                    }
                }

                Event::RedrawEventsCleared => {
                    graphic.request_redraw();
                }

                _ => {}
            }
        })
    }
}

