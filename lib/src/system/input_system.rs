use crate::component::{camera_component, input_component};
use shipyard::IntoFastIter;
use winit::event::MouseScrollDelta;

pub fn register_mouse_input_system(
    mouse_state: input_component::MouseClick,
    mut input_state: shipyard::UniqueViewMut<input_component::Input>,
) {
    input_state.mouse_state = Some(mouse_state);
}

pub fn register_scroll_input_system(
    scroll_state: input_component::ScrollState,
    mut camera_controller: shipyard::ViewMut<camera_component::CameraController>,
    mut input_state: shipyard::UniqueViewMut<input_component::Input>,
) {
    input_state.scroll_delta = Some(scroll_state);

    (&mut camera_controller).fast_iter().for_each(|controller|{
        match input_state.scroll_delta.as_ref().unwrap().scroll_delta{
            winit::event::MouseScrollDelta::LineDelta(_, y_position) => {
                controller.amount_scroll.x = y_position * controller.amount_scroll.y
            }

            winit::event::MouseScrollDelta::PixelDelta(_) => {
                eprintln!("Touch pad or other device that use PixelDelta is not supported on this application");
                //todo
            }
        }

    })
}

pub fn register_key_input_system(
    key_state: input_component::KeyState,
    mut camera_controller: shipyard::ViewMut<camera_component::CameraController>,
    mut input_state: shipyard::UniqueViewMut<input_component::Input>,
) {
    input_state.key_state = Some(key_state);

    //todo last
    (&mut camera_controller).fast_iter().for_each(|controller| {
        let key_state = input_state.key_state.as_ref().unwrap();

        let amount = if key_state.state == winit::event::ElementState::Pressed {
            1.0
        } else {
            0.0
        };

        match key_state.keycode {
            winit::event::VirtualKeyCode::W | winit::event::VirtualKeyCode::Up => {
                controller.amount_matrix.x_axis.x = amount * controller.amount_matrix.x_axis.z;
            }
            winit::event::VirtualKeyCode::S | winit::event::VirtualKeyCode::Down => {
                controller.amount_matrix.x_axis.y = amount * controller.amount_matrix.x_axis.z;
            }
            winit::event::VirtualKeyCode::D | winit::event::VirtualKeyCode::Right => {
                controller.amount_matrix.y_axis.x = amount * controller.amount_matrix.y_axis.z;
            }
            winit::event::VirtualKeyCode::A | winit::event::VirtualKeyCode::Left => {
                controller.amount_matrix.y_axis.y = amount * controller.amount_matrix.y_axis.z;
            }
            winit::event::VirtualKeyCode::Q => {
                controller.amount_matrix.z_axis.x = amount * controller.amount_matrix.z_axis.z;
            }
            winit::event::VirtualKeyCode::E => {
                controller.amount_matrix.z_axis.y = amount * controller.amount_matrix.z_axis.z;
            }
            _ => {}
        }
    })
}

pub fn register_mouse_motion_system(
    mouse_delta: input_component::MousePosition,
    mut camera_controller: shipyard::ViewMut<camera_component::CameraController>,
    input_state: shipyard::UniqueView<input_component::Input>,
) {
    if input_state.mouse_state.is_some() {
        let condition = input_state.mouse_state.as_ref().unwrap().button
            == winit::event::MouseButton::Left
            && input_state.mouse_state.as_ref().unwrap().state
                == winit::event::ElementState::Pressed;

        if condition {
            (&mut camera_controller).fast_iter().for_each(|controller| {
                controller.amount_rotation.x -=
                    -mouse_delta.delta.1 as f32 * controller.amount_rotation.w;
                controller.amount_rotation.y -=
                    mouse_delta.delta.0 as f32 * controller.amount_rotation.w;
            });
        }
    }
}
