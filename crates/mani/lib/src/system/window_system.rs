use crate::component::window_component;

pub fn window_redraw_system(window: shipyard::UniqueView<window_component::Window>) {
    window.raw.request_redraw();
}
