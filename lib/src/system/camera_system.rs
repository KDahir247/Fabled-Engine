use crate::component::{camera_component, render_component, time_component};
use crate::util::camera;
use shipyard::IntoFastIter;

pub fn camera_update_system(
    setup: shipyard::UniqueView<render_component::Setup>,
    mut camera: shipyard::UniqueViewMut<camera_component::Camera>,
    delta_time: shipyard::UniqueView<time_component::DeltaTime>,
    mut controller: shipyard::ViewMut<camera_component::CameraController>,
) {
    (&mut controller).fast_iter().for_each(|camera_controller| {
        camera::update_camera_orientation(&mut camera, camera_controller, delta_time.delta);

        camera.uniform.raw.view_position = glam::vec3(
            camera.orientation.position.x,
            camera.orientation.position.y,
            camera.orientation.position.z,
        )
        .extend(1.0);

        camera.uniform.raw.view_proj = camera::calc_proj_matrix(&camera.projection)
            * camera::calc_camera_matrix(&camera.orientation);

        setup.queue.write_buffer(
            &camera.uniform.buffer,
            0,
            bytemuck::cast_slice(&[camera.uniform.raw]),
        )
    });
}

pub fn process_keyboard_system() {}

pub fn process_mouse_system() {}

pub fn process_scroll_system() {}
