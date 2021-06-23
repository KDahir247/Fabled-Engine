use super::constant;
use crate::component::{camera_component, time_component};

pub fn calc_camera_matrix(camera: &camera_component::CameraOrientation) -> glam::Mat4 {
    let position = camera.transformation_matrix.w_axis.truncate();

    let f = camera.forward.normalize().truncate();

    let s = f.cross(glam::Vec3::Y).normalize();
    let u = s.cross(f);

    glam::mat4(
        glam::vec4(s.x, u.x, -f.x, 0.0),
        glam::vec4(s.y, u.y, -f.y, 0.0),
        glam::vec4(s.z, u.z, -f.z, 0.0),
        glam::vec4(-position.dot(s), -position.dot(u), position.dot(f), 1.0),
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

    //test
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
