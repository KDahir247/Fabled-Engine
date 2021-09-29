use lib::component::prelude::*;
use std::io::Read;
mod graphic;
mod setup;
mod window;

pub struct State;

impl State {
    pub fn run() {
        let (event_loop, window) = State::create_window();

        superluminal_perf::begin_event("Create_World_Static");

        let world = shipyard::World::new();

        world.add_unique(Window { raw: window }).unwrap();

        world
            .add_unique(Time {
                application: lib::component::time_component::Application {
                    time: std::time::Instant::now(),
                },
                last_frame: 0.0,
                delta: 0.0,
            })
            .unwrap();

        superluminal_perf::end_event();

        graphic::Graphic::run(&world).expect("Failed on graphic");

        window::Window::run((world, event_loop)).expect("Failed on window")
    }
}

impl State {
    fn create_window() -> (winit::event_loop::EventLoop<()>, winit::window::Window) {
        superluminal_perf::begin_event("Create_Window");

        let event_loop = winit::event_loop::EventLoop::with_user_event();

        let window = winit::window::WindowBuilder::new()
            .with_inner_size(winit::dpi::Size::Physical(winit::dpi::PhysicalSize {
                width: 1920,
                height: 1080,
            }))
            .with_title("Fabled Engine")
            .with_decorations(true)
            .build(&event_loop)
            .unwrap();

        window.set_cursor_visible(false);

        superluminal_perf::end_event();

        (event_loop, window)
    }
}
