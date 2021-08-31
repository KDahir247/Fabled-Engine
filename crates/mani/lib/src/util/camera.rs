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
    let h = 1.0 / (projection.fovy * 0.5); //1/tan(x) == cot(x)
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
    let (_, rotation, _) = camera
        .orientation
        .transformation_matrix
        .to_scale_rotation_translation();

    let desired_rotation = rotation
        * glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            controller.amount_rotation.x.to_radians(),
            controller.amount_rotation.y.to_radians(),
            0.0,
        );

    let desired_rotation = desired_rotation.normalize();

    controller.amount_rotation = glam::Vec4::W * controller.amount_rotation.w;

    camera.orientation.forward = camera.orientation.transformation_matrix.z_axis.normalize();
    camera.orientation.right = camera.orientation.transformation_matrix.x_axis.normalize();

    let mut translation = camera.orientation.transformation_matrix.w_axis;

    translation += camera.orientation.forward * controller.amount_translation.z * delta_time;

    let negate_y = glam::vec4(1.0, 0.0, 1.0, 1.0) * delta_time;
    translation += camera.orientation.right * controller.amount_translation.x * negate_y;

    translation.y += controller.amount_translation.y * delta_time;

    let affine_transform = glam::Affine3A::from_rotation_translation(
        desired_rotation,
        translation.xyz() * translation.w,
    );

    camera.orientation.transformation_matrix = glam::Mat4::from(affine_transform);

    //Projection
    camera.projection.fovy += -controller.amount_scroll.x * controller.amount_scroll.y * delta_time;

    if camera.projection.fovy < 1.0_f32.to_radians() {
        camera.projection.fovy = 1.0_f32.to_radians();
    } else if camera.projection.fovy > 179.0_f32.to_radians() {
        camera.projection.fovy = 179.0_f32.to_radians();
    }

    controller.amount_scroll.x = 0.0;
}
