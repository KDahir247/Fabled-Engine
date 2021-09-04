use crate::component::camera_component;
use glam::Vec4Swizzles;

pub fn calc_view_matrix(camera: &camera_component::CameraOrientation) -> glam::Mat4 {
    let position = camera.transformation_matrix.w_axis.truncate();

    let f = -camera.forward.normalize().truncate();
    let r = glam::Vec3::Y.cross(f).normalize();
    let u = f.cross(r);

    glam::mat4(
        glam::vec4(r.x, u.x, f.x, 0.0),
        glam::vec4(r.y, u.y, f.y, 0.0),
        glam::vec4(r.z, u.z, f.z, 0.0),
        glam::vec4(-position.dot(r), -position.dot(u), -position.dot(f), 1.0),
    )
}

pub fn calc_proj_matrix(projection: &camera_component::Projection) -> glam::Mat4 {
    let h = 1.0 / (projection.fovy * 0.5).tan(); // 1/tan(x) == cot(x)
    let w = h / projection.aspect;
    let near_min_far = projection.znear - projection.zfar;

    glam::mat4(
        glam::vec4(w, 0.0, 0.0, 0.0),
        glam::vec4(0.0, h, 0.0, 0.0),
        glam::vec4(0.0, 0.0, projection.zfar / near_min_far, -1.0),
        glam::vec4(
            0.0,
            0.0,
            projection.znear * projection.zfar / near_min_far,
            0.0,
        ),
    )
}

pub fn update_camera_orientation(
    camera: &mut camera_component::Camera,
    controller: &mut camera_component::CameraController,
    delta_time: f32,
) {
    // Rotation
    let (_, current_rotation, _) = camera
        .orientation
        .transformation_matrix
        .to_scale_rotation_translation();

    let x_rotation = glam::Quat::from_rotation_x(controller.amount_rotation.x.to_radians());
    let y_rotation = glam::Quat::from_rotation_y(controller.amount_rotation.y.to_radians());

    let target_rotation = x_rotation * y_rotation;

    let mut desired_rotation = current_rotation * target_rotation;

    desired_rotation = desired_rotation.normalize();

    controller.amount_rotation = glam::Vec4::W * controller.amount_rotation.w;

    // Translation
    camera.orientation.forward = camera.orientation.transformation_matrix.z_axis.normalize();
    camera.orientation.right = camera.orientation.transformation_matrix.x_axis.normalize();

    let mut current_translation = camera.orientation.transformation_matrix.w_axis;

    current_translation +=
        camera.orientation.forward * controller.amount_translation.z * delta_time;

    let negate_y = glam::vec4(1.0, 0.0, 1.0, 1.0) * delta_time;
    current_translation += camera.orientation.right * controller.amount_translation.x * negate_y;

    current_translation.y += controller.amount_translation.y * delta_time;

    let desire_translation = current_translation.xyz() / current_translation.w;

    let affine_transform =
        glam::Affine3A::from_rotation_translation(desired_rotation, desire_translation);

    camera.orientation.transformation_matrix = glam::Mat4::from(affine_transform);

    // Fov
    let hermite = controller.amount_scroll
        * controller.amount_scroll
        * (3.0 - 2.0 * controller.amount_scroll);

    let frac_pi_3 = std::f32::consts::FRAC_PI_3;
    let pi = std::f32::consts::PI;

    camera.projection.fovy = (1.0 - hermite) * frac_pi_3 + hermite * pi;
}
