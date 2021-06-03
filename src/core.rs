use winit::platform::windows::{WindowBuilderExtWindows, IconExtWindows};
use winit::window::{Icon, BadIcon};
use std::error::Error;

mod graphic;
mod window;

pub trait Command{
    type Output;
    type Input;
    fn run(options : Self::Input) -> anyhow::Result<Self::Output>;
}

//might make state hold both graphic and event_loop
pub struct State;

impl State {
    pub fn run(){

        //todo icon support doesn't work
        let icon = Self::request_icon();

        let icon =  match icon {
            Ok(icon) => {
                Some(icon)
            }
            Err(err) => {
                match err{
                    BadIcon::ByteCountNotDivisibleBy4 { .. } => {
                        println!("Error ByteCountNotDivisibleBy4");
                    }
                    BadIcon::DimensionsVsPixelCount { .. } => {
                        println!("Error DimensionsVsPixelCount : ");
                    }
                    BadIcon::OsError(er) => {
                        println!("Error {}",er);
                    }
                }
                None
            }
        };


        let event_loop = winit::event_loop::EventLoop::new();

        let window = winit::window::WindowBuilder::new()
            .with_inner_size(
                winit::dpi::Size::Physical(winit::dpi::PhysicalSize{
                    width: 800,
                    height: 600
                }))
            .with_title("BasicObjLoader")
            .with_decorations(true)
            .with_window_icon(icon.clone())
            .with_taskbar_icon(icon)
            .build(&event_loop)
            .unwrap();

        let graphic : graphic::Graphic = graphic::Graphic::run(window)
          .expect("Failed on graphic");


        window::Window::run((graphic, event_loop))
            .expect("Failed on window");
    }

    fn request_icon() -> Result<winit::window::Icon, winit::window::BadIcon>{

        let icon = std::env::current_dir().unwrap().join("img\\icon\\icon_hi.jpg");

        winit::window::Icon::from_path(icon.as_path(), Some(winit::dpi::PhysicalSize{ width: 96, height: 96 }))
    }
}