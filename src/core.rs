mod graphic;
mod window;

pub trait Command {
    type Output;
    type Input;
    fn run(options: Self::Input) -> anyhow::Result<Self::Output>;
}

pub struct State;

//Public
impl State {
    pub fn run() {
        let (event_loop, window) = State::create_window();

        let graphic: graphic::Graphic = graphic::Graphic::run(window).expect("Failed on graphic");

        window::Window::run((graphic, event_loop)).expect("Failed on window")
    }
}

//Private
impl State {
    fn create_window() -> (winit::event_loop::EventLoop<()>, winit::window::Window) {
        let event_loop = winit::event_loop::EventLoop::new();

        let window = winit::window::WindowBuilder::new()
            .with_inner_size(winit::dpi::Size::Physical(winit::dpi::PhysicalSize {
                width: 800,
                height: 600,
            }))
            .with_title("BasicObjLoader")
            .with_decorations(true)
            .build(&event_loop)
            .unwrap();

        window.set_cursor_visible(false);

        (event_loop, window)
    }
}
