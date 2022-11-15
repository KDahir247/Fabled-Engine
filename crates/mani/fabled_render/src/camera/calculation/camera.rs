use fabled_math::{Matrix4x4, Quaternion, Vector3, Vector4};

use fabled_math::matrix4x4_math::{compose_trs_mat4, from_translation_mat4, inverse_mat4};
use fabled_math::quaternion_math::{forward_vec3, inverse_quat};
use fabled_math::vector_math::{cross, dot, normalize};

use crate::camera::{
    AspectRatio, ClippingPlane, FovAxis, Oblique, Orthographic, Perspective, ViewPort, SENSOR_SIZE,
};

#[derive(Copy, Clone, Default)]
pub struct MatrixDescriptor {
    pub projection: Matrix4x4,
    pub view: Matrix4x4,
    pub model: Matrix4x4,
}

pub fn project(
    target: Vector3,
    viewport: &ViewPort,
    matrix_descriptor: MatrixDescriptor,
) -> Vector3 {
    let t_mvp_target_vector = matrix_descriptor.projection
        * matrix_descriptor.view
        * matrix_descriptor.model
        * Vector4::set(target.x(), target.y(), target.z(), 1.0);

    let normalized_factor = 1.0 / t_mvp_target_vector.w();

    let x = t_mvp_target_vector.x() * normalized_factor;
    let y = -t_mvp_target_vector.y() * normalized_factor;
    let z = t_mvp_target_vector.z() * normalized_factor;

    let x_p_one = x + 1.0;
    let y_p_one = y + 1.0;
    let viewport_diff = viewport.max_depth - viewport.min_depth;

    let half_x_p_one = x_p_one * 0.5;
    let half_y_p_one = y_p_one * 0.5;
    let z_mul_viewport_diff = z * viewport_diff;

    Vector3::set(
        (half_x_p_one * viewport.w) + viewport.x,
        (half_y_p_one * viewport.h) + viewport.y,
        z_mul_viewport_diff + viewport.min_depth,
    )
}

pub fn unproject(
    target: Vector3,
    viewport: &ViewPort,
    matrix_descriptor: MatrixDescriptor,
) -> Vector3 {
    let matrix = inverse_mat4(
        matrix_descriptor.projection * (matrix_descriptor.view * matrix_descriptor.model),
    );

    let x = target.x() - viewport.x;
    let y = target.y() - viewport.y;
    let z = target.z() - viewport.min_depth;
    let depth_difference = (viewport.max_depth - viewport.min_depth).recip();

    let x_mul_two = x * 2.0;
    let y_mul_two = y * 2.0;
    let z_mul_depth_diff = z * depth_difference;

    let vector = matrix
        * Vector4::set(
            (x_mul_two / viewport.w) - 1.0,
            -((y_mul_two / viewport.h) - 1.0),
            z_mul_depth_diff,
            1.0,
        );

    vector.trunc_vec3()
}

pub fn compute_look_at_matrix(translation: Vector3, rotation: Quaternion) -> Matrix4x4 {
    const Y_VEC3: Vector3 = Vector3::UP;

    let forward = forward_vec3(rotation);

    let neg_translation = -translation;

    let z_axis = Vector3 {
        value: normalize(-forward.value),
    };
    let x_axis = Vector3 {
        value: normalize(cross(Y_VEC3.value, z_axis.value)),
    };
    let y_axis = Vector3 {
        value: cross(z_axis.value, x_axis.value),
    };

    let t_axis = Vector4::set(
        dot(neg_translation.value, x_axis.value),
        dot(neg_translation.value, y_axis.value),
        dot(neg_translation.value, z_axis.value),
        1.0,
    );

    Matrix4x4::set(
        Vector4::set(x_axis.x(), y_axis.x(), z_axis.x(), 0.0),
        Vector4::set(x_axis.y(), y_axis.y(), z_axis.z(), 0.0),
        Vector4::set(x_axis.z(), y_axis.z(), z_axis.z(), 0.0),
        t_axis,
    )
}

#[deprecated(note = "Calculate arc ball matrix function has not been
tested.")]
pub fn compute_arc_ball_matrix(
    translation: Vector3,
    rotation: Quaternion,
    center: Vector3,
) -> Matrix4x4 {
    let transformation_matrx = compose_trs_mat4(-translation, inverse_quat(rotation), Vector3::ONE);
    let translation_to_center = from_translation_mat4(-center);

    transformation_matrx * translation_to_center
}

#[rustfmt::skip]
pub fn compute_perspective_matrix(perspective: Perspective, clipping_plane : ClippingPlane) -> Matrix4x4{

    let far_plane = clipping_plane.far;
    let near_plane = clipping_plane.near;
    
    let near_min_far = near_plane - far_plane;

    let mut h = 0.0;
    let mut w = 0.0;

    let s =(perspective.fov.radian * 0.5).tan() * near_plane;
    
    match perspective.fov.axis {
        FovAxis::Horizontal => {
            w = s;
            h = s * perspective.aspect.get_aspect();
        }
        FovAxis::Vertical => {
            h = s;
            w = s / perspective.aspect.get_aspect();
        }
    }

    let inv_near_min_far = 1.0 / near_min_far;
    let r = far_plane * inv_near_min_far;
    Matrix4x4::set(
        Vector4::set(w, 0.0, 0.0, 0.0),
        Vector4::set(0.0, h, 0.0, 0.0),
        Vector4::set(0.0, 0.0, r, -1.0),
        Vector4::set(0.0, 0.0, r * near_plane, 0.0)
    )
}

// not sure if this works... Use the focal to fov mapping function. the use
// compute_perspective_matrix fn It is more explicit
pub fn compute_len_perspective_matrix(
    focal_length: f32,
    aspect_ratio: AspectRatio,
    clipping_plane: ClippingPlane,
) -> Matrix4x4 {
    let far_plane = clipping_plane.far;
    let near_plane = clipping_plane.near;

    let near_min_far = near_plane - far_plane;

    let h = (0.5 * clipping_plane.near) * ((SENSOR_SIZE * 1000.0) / focal_length);
    let w = h * aspect_ratio.get_aspect();

    let inv_near_min_far = 1.0 / near_min_far;
    let r = far_plane * inv_near_min_far;

    Matrix4x4::set(
        Vector4::set(w, 0.0, 0.0, 0.0),
        Vector4::set(0.0, h, 0.0, 0.0),
        Vector4::set(0.0, 0.0, r, -1.0),
        Vector4::set(0.0, 0.0, r * near_plane, 0.0),
    )
}

pub fn compute_orthographic_matrix(
    orthographic: Orthographic,
    clipping_plane: ClippingPlane,
) -> Matrix4x4 {
    let far_plane = clipping_plane.far;
    let near_plane = clipping_plane.near;

    let near_plane_min_far_plane_rcp = (near_plane - far_plane).recip();

    let right_min_left_rcp = orthographic.right - orthographic.left;
    let right_plus_left = orthographic.right + orthographic.left;

    let top_min_bot_rcp = orthographic.top - orthographic.bottom;
    let top_plus_bot = orthographic.top + orthographic.bottom;

    Matrix4x4::set(
        Vector4::set(2.0 * right_min_left_rcp, 0.0, 0.0, 0.0),
        Vector4::set(0.0, 2.0 * top_min_bot_rcp, 0.0, 0.0),
        Vector4::set(0.0, 0.0, -1.0 * near_plane_min_far_plane_rcp, 0.0),
        Vector4::set(
            -(right_plus_left * right_min_left_rcp),
            -(top_plus_bot * top_min_bot_rcp),
            -(far_plane * near_plane_min_far_plane_rcp),
            1.0,
        ),
    )
}


#[rustfmt::skip]
pub fn compute_oblique_projection_matrix(orthographic : Orthographic, oblique
: Oblique, clipping_plane: ClippingPlane) -> Matrix4x4{

    let projection = compute_orthographic_matrix(orthographic, clipping_plane);

    let size = oblique.vertical_position / orthographic.top;

    let rotation_x = size * -oblique.angle_rad.sin();
    let rotation_y = -size * -oblique.angle_rad.cos();

    let depth_offset_x = -oblique.depth_offset * rotation_x;
    let depth_offset_y = -oblique.depth_offset * rotation_y;

    Matrix4x4::set(
        Vector4::set(projection.column_x.x(), projection.column_x.y(), rotation_x, depth_offset_x),
        Vector4::set(projection.column_y.x(), projection.column_y.y(), rotation_y, depth_offset_y),
        Vector4::set(projection.column_z.x(), projection.column_z.y(), projection.column_z.z(), projection.column_z.w()),
        Vector4::set(projection.column_w.x(), projection.column_w.y(), projection.column_w.z(), projection.column_w.w())
    )
}
