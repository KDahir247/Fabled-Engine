use crate::component::{camera_component, render_component, time_component};
use crate::util::camera;
use shipyard::IntoFastIter;

pub fn camera_update_system(
    setup: shipyard::UniqueView<render_component::RenderData>,
    mut camera: shipyard::UniqueViewMut<camera_component::Camera>,
    delta_time: shipyard::UniqueView<time_component::Time>,
    mut controller: shipyard::ViewMut<camera_component::CameraController>,
) {
    (&mut controller).fast_iter().for_each(|camera_controller| {
        camera::update_camera_orientation(&mut camera, camera_controller, delta_time.delta as f32);

        camera.uniform.raw.view_position = camera.orientation.transformation_matrix.w_axis;

        camera.uniform.raw.proj = camera::calc_proj_matrix(&camera.projection);
        camera.uniform.raw.view = camera::calc_view_matrix(&camera.orientation);
        camera.uniform.raw.inv_proj = camera.uniform.raw.proj.inverse();
        camera.uniform.raw.inv_view = camera.uniform.raw.view.inverse();

        setup.pass.queue.write_buffer(
            &camera.uniform.buffer,
            0,
            bytemuck::cast_slice(&[camera.uniform.raw]),
        )
    });
}
