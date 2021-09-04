use lib::component::prelude::*;
use winit::event::{DeviceEvent, WindowEvent};

pub struct Window {
    focused: bool,
}

impl Window {
    pub fn run(options: (shipyard::World, winit::event_loop::EventLoop<()>)) -> anyhow::Result<()> {
        let window = Self { focused: false };

        let (world, event_loop) = options;

        shipyard::Workload::builder("window_redraw_system")
            .with_system(&lib::system::window_system::window_redraw_system)
            .add_to_world(&world)
            .unwrap();

        Window::handle(window, world, event_loop);

        Ok(())
    }
}

impl Window {
    fn handle(mut self, mut world: shipyard::World, event_loop: winit::event_loop::EventLoop<()>) {
        event_loop.run(move |evt, _, control_flow| {
            world.run_workload("update_delta_time_system").unwrap();

            match evt {
                winit::event::Event::NewEvents(winit::event::StartCause::Init) => {
                    let entity_id = world.add_entity((GridData,));

                    world.run_workload("load_grid_system").unwrap();

                    world.delete_component::<(GridData,)>(entity_id);
                }

                winit::event::Event::DeviceEvent { event, .. } => {
                    if self.focused {
                        match event {
                            DeviceEvent::MouseMotion { delta } => {
                                world
                                    .run_with_data(
                                        lib::system::input_system::register_mouse_motion_system,
                                        input_component::MousePosition { delta },
                                    )
                                    .unwrap();
                            }
                            DeviceEvent::MouseWheel { delta } => {
                                world
                                    .run_with_data(
                                        lib::system::input_system::register_scroll_input_system,
                                        input_component::ScrollState {
                                            scroll_delta: delta,
                                        },
                                    )
                                    .unwrap();
                            }

                            DeviceEvent::Button { button, state } => {
                                world
                                    .run_with_data(
                                        lib::system::input_system::register_mouse_input_system,
                                        input_component::MouseClick {
                                            button: if button == 1 {
                                                winit::event::MouseButton::Left
                                            } else if button == 3 {
                                                winit::event::MouseButton::Right
                                            } else {
                                                winit::event::MouseButton::Other(0)
                                            },
                                            state,
                                        },
                                    )
                                    .unwrap();
                            }
                            DeviceEvent::Key(winit::event::KeyboardInput {
                                virtual_keycode: Some(keycode),
                                state,
                                ..
                            }) => {
                                world
                                    .run_with_data(
                                        lib::system::input_system::register_key_input_system,
                                        input_component::KeyState { keycode, state },
                                    )
                                    .unwrap();
                            }
                            _ => {}
                        }
                    }
                }

                winit::event::Event::WindowEvent { event, .. } => {
                    match event {
                        winit::event::WindowEvent::Resized(_) => {
                            world.run_workload("render_resize_system").unwrap();
                        }

                        winit::event::WindowEvent::KeyboardInput {
                            input:
                                winit::event::KeyboardInput {
                                    virtual_keycode: Some(winit::event::VirtualKeyCode::Escape),
                                    state: winit::event::ElementState::Pressed,
                                    ..
                                },
                            ..
                        }
                        | winit::event::WindowEvent::CloseRequested => {
                            *control_flow = winit::event_loop::ControlFlow::Exit;
                        }

                        winit::event::WindowEvent::DroppedFile(file_path) => {
                            // todo add an entity to the world that will have the file_path and the
                            // string to represent the path to the shader.
                            let model_id = world.add_entity((ModelData {
                                path: file_path,
                                shader_path: "../../shader/standard.wgsl".to_string(),
                            },));

                            world.run_workload("load_model_system").unwrap();

                            world.delete_component::<(ModelData,)>(model_id);
                        }

                        winit::event::WindowEvent::ScaleFactorChanged { .. } => {
                            world.run_workload("render_resize_system").unwrap();
                        }
                        WindowEvent::Focused(focus) => self.focused = focus,

                        _ => {}
                    }
                }

                winit::event::Event::RedrawRequested(_) => {
                    world.run_workload("render_update_system").unwrap();
                }

                winit::event::Event::RedrawEventsCleared => {
                    world.run_workload("window_redraw_system").unwrap();
                }

                _ => {}
            }
        });
    }
}
