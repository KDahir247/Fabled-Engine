pub struct MouseClick {
    pub button: winit::event::MouseButton,
    pub state: winit::event::ElementState,
}

pub struct MousePosition {
    pub delta: (f64, f64),
}

pub struct KeyState {
    pub keycode: winit::event::VirtualKeyCode,
    pub state: winit::event::ElementState,
}

pub struct ScrollState {
    pub scroll_delta: winit::event::MouseScrollDelta,
}

#[derive(Default)]
pub struct Input {
    pub mouse_state: Option<MouseClick>,
    pub scroll_delta: Option<ScrollState>,
    pub key_state: Option<KeyState>,
}
