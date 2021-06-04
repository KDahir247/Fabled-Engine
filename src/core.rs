use winit::platform::windows::{IconExtWindows, WindowBuilderExtWindows};

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
        //todo icon support doesn't work currently
        let icon = Self::request_icon();

        let icon = match icon {
            Ok(icon) => Some(icon),
            Err(err) => {
                match err {
                    winit::window::BadIcon::ByteCountNotDivisibleBy4 { .. } => {
                        println!("Error ByteCountNotDivisibleBy4");
                    }
                    winit::window::BadIcon::DimensionsVsPixelCount { .. } => {
                        println!("Error DimensionsVsPixelCount : ");
                    }
                    winit::window::BadIcon::OsError(er) => {
                        println!("Error {:?}", er);
                    }
                }
                None
            }
        };

        let event_loop = winit::event_loop::EventLoop::new();

        let window = winit::window::WindowBuilder::new()
            .with_inner_size(winit::dpi::Size::Physical(winit::dpi::PhysicalSize {
                width: 800,
                height: 600,
            }))
            .with_title("BasicObjLoader")
            .with_decorations(true)
            .with_window_icon(icon.clone())
            .with_taskbar_icon(icon)
            .build(&event_loop)
            .unwrap();

        (event_loop, window)
    }

    fn request_icon() -> Result<winit::window::Icon, winit::window::BadIcon> {
        let icon = std::env::current_dir()
            .unwrap()
            .join("img\\icon\\icon_hi.jpg");

        winit::window::Icon::from_path(
            icon.as_path(),
            Some(winit::dpi::PhysicalSize {
                width: 96,
                height: 96,
            }),
        )
    }
}
