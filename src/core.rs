use lib::component::prelude::*;

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
            .add_unique(DeltaTime {
                last_render_time: std::time::Instant::now(),
                delta: Default::default(),
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
                width: 800,
                height: 600,
            }))
            .with_title("BasicObjLoader")
            .with_decorations(true)
            //.with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)))
            .build(&event_loop)
            .unwrap();

        window.set_cursor_visible(false);

        superluminal_perf::end_event();

        (event_loop, window)
    }
}
