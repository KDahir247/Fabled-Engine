use super::constant;
use crate::component::{camera_component, time_component};

pub fn calc_camera_matrix(camera: &camera_component::CameraOrientation) -> glam::Mat4 {
    let f = camera.forward.normalize();
    let s = f.cross(glam::Vec3::Y).normalize();
    let u = s.cross(f);

    glam::mat4(
        glam::vec4(s.x, u.x, -f.x, 0.0),
        glam::vec4(s.y, u.y, -f.y, 0.0),
        glam::vec4(s.z, u.z, -f.z, 0.0),
        glam::vec4(
            -camera.position.dot(s),
            -camera.position.dot(u),
            camera.position.dot(f),
            1.0,
        ),
    )
}

pub fn calc_proj_matrix(projection: &camera_component::Projection) -> glam::Mat4 {
    constant::OPENGL_TO_WGPU_MATRIX
        * glam::Mat4::perspective_rh_gl(
            projection.fovy,
            projection.aspect,
            projection.znear,
            projection.zfar,
        )
}

pub fn update_camera_orientation(
    camera: &mut camera_component::Camera,
    controller: &mut camera_component::CameraController,
    delta_time: std::time::Duration,
) {
    let dt = delta_time.as_secs_f32();

    //Camera
    let (yaw_sin, yaw_cos) = camera.orientation.rotation.y.sin_cos();
    let pitch_sin = camera.orientation.rotation.x.sin();

    camera.orientation.forward = glam::Vec3::new(yaw_cos, pitch_sin, yaw_sin).normalize();

    camera.orientation.right = glam::Vec3::new(-yaw_sin, 0.0, yaw_cos).normalize();

    camera.orientation.position += camera.orientation.forward
        * (controller.amount_matrix.x_axis.x - controller.amount_matrix.x_axis.y)
        * dt;

    camera.orientation.position += camera.orientation.right
        * (controller.amount_matrix.y_axis.x - controller.amount_matrix.y_axis.y)
        * dt;

    camera.orientation.position.y +=
        (controller.amount_matrix.z_axis.x - controller.amount_matrix.z_axis.y) * dt;

    camera.orientation.rotation.x += controller.amount_rotation.x * dt;
    camera.orientation.rotation.y += controller.amount_rotation.y * dt;

    controller.amount_rotation = controller.amount_rotation.w * glam::Vec4::W;

    if camera.orientation.rotation.x < -std::f32::consts::FRAC_2_PI {
        camera.orientation.rotation.x = -std::f32::consts::FRAC_2_PI;
    } else if camera.orientation.rotation.x > std::f32::consts::FRAC_2_PI {
        camera.orientation.rotation.x = std::f32::consts::FRAC_2_PI;
    }

    //Projection
    camera.projection.fovy += -controller.amount_scroll.x * dt;

    if camera.projection.fovy < 1.0_f32.to_radians() {
        camera.projection.fovy = 1.0_f32.to_radians();
    } else if camera.projection.fovy > 179.0_f32.to_radians() {
        camera.projection.fovy = 179.0_f32.to_radians();
    }

    controller.amount_scroll.x = 0.0;
}
