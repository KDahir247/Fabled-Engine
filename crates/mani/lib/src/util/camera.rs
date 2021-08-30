use super::constant;
use crate::component::camera_component;

pub fn calc_camera_matrix(camera: &camera_component::CameraOrientation) -> glam::Mat4 {
    let position = camera.transformation_matrix.w_axis.truncate();

    //todo double check.
    //this is rhs to get lhs just negate the camera forward.
    //Matrix View = Inverse Matrix Camera.

    let f = camera.forward.normalize().truncate();
    let r = f.cross(glam::Vec3::Y).normalize();
    let u = r.cross(f);

    glam::mat4(
        glam::vec4(r.x, u.x, -f.x, 0.0),
        glam::vec4(r.y, u.y, -f.y, 0.0),
        glam::vec4(r.z, u.z, -f.z, 0.0),
        glam::vec4(-position.dot(r), -position.dot(u), position.dot(f), 1.0),
    )
}

pub fn calc_proj_matrix(projection: &camera_component::Projection) -> glam::Mat4 {
    let t = (projection.fovy * 0.5).tan();
    let sy = 1.0 / t;
    let sx = sy / projection.aspect;
    let nmf = projection.znear - projection.zfar;

    glam::mat4(
        glam::vec4(sx, 0.0, 0.0, 0.0),
        glam::vec4(0.0, sy, 0.0, 0.0),
        glam::vec4(0.0, 0.0, projection.zfar / nmf, -1.0),
        glam::vec4(0.0, 0.0, projection.znear * projection.zfar / nmf, 0.0),
    )
}

pub fn update_camera_orientation(
    camera: &mut camera_component::Camera,
    controller: &mut camera_component::CameraController,
    delta_time: f32,
) {
    camera.orientation.forward = camera.orientation.transformation_matrix.z_axis;
    camera.orientation.right = camera.orientation.transformation_matrix.x_axis;

    camera.orientation.transformation_matrix.w_axis += camera.orientation.forward
        * (controller.amount_matrix.x_axis.x - controller.amount_matrix.x_axis.y)
        * controller.amount_matrix.x_axis.z
        * delta_time;

    camera.orientation.transformation_matrix.w_axis += camera.orientation.right
        * (controller.amount_matrix.y_axis.y - controller.amount_matrix.y_axis.x)
        * controller.amount_matrix.y_axis.z
        * delta_time;

    camera.orientation.transformation_matrix.w_axis.y += (controller.amount_matrix.z_axis.x
        - controller.amount_matrix.z_axis.y)
        * controller.amount_matrix.z_axis.z
        * delta_time;

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

    let x2 = desired_rotation.x * desired_rotation.x;
    let y2 = desired_rotation.y * desired_rotation.y;
    let z2 = desired_rotation.z * desired_rotation.z;
    let xy = desired_rotation.x * desired_rotation.y;
    let xz = desired_rotation.x * desired_rotation.z;
    let yz = desired_rotation.y * desired_rotation.z;
    let wx = desired_rotation.w * desired_rotation.x;
    let wy = desired_rotation.w * desired_rotation.y;
    let wz = desired_rotation.w * desired_rotation.z;

    //todo re-look at this the yaw move the camera up and down
    let rotation = glam::mat3(
        glam::vec3(
            1.0f32 - 2.0f32 * y2 - 2.0f32 * z2,
            2.0f32 * xy + 2.0f32 * wz,
            2.0f32 * xz - 2.0f32 * wy,
        ),
        glam::vec3(
            2.0f32 * xy - 2.0f32 * wz,
            1.0f32 - 2.0f32 * x2 - 2.0f32 * z2,
            2.0f32 * yz + 2.0f32 * wx,
        ),
        glam::vec3(
            2.0f32 * xz + 2.0f32 * wy,
            2.0f32 * yz - 2.0f32 * wx,
            1.0f32 - 2.0f32 * x2 - 2.0f32 * y2,
        ),
    );

    camera.orientation.transformation_matrix.x_axis = rotation.x_axis.extend(0.0);
    camera.orientation.transformation_matrix.y_axis = rotation.y_axis.extend(0.0);
    camera.orientation.transformation_matrix.z_axis = rotation.z_axis.extend(0.0);

    controller.amount_rotation = glam::Vec4::W * controller.amount_rotation.w;

    //Projection
    camera.projection.fovy += -controller.amount_scroll.x * controller.amount_scroll.y * delta_time;

    if camera.projection.fovy < 1.0_f32.to_radians() {
        camera.projection.fovy = 1.0_f32.to_radians();
    } else if camera.projection.fovy > 179.0_f32.to_radians() {
        camera.projection.fovy = 179.0_f32.to_radians();
    }

    controller.amount_scroll.x = 0.0;
}
