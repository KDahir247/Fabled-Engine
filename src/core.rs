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

        let event_loop = winit::event_loop::EventLoop::new();

        let window = winit::window::WindowBuilder::new()
            .with_inner_size(
                winit::dpi::Size::Physical(winit::dpi::PhysicalSize{
                    width: 800,
                    height: 600
                }))
            .with_title("BasicObjLoader")
            .build(&event_loop)
            .unwrap();



        let graphic : graphic::Graphic = graphic::Graphic::run(window)
          .expect("Failed on graphic");



        window::Window::run((graphic, event_loop)).expect("Failed on window");
    }
}