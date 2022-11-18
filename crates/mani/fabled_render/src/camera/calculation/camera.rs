use fabled_math::{Matrix4x4, Quaternion, Vector3, Vector4};

use fabled_math::matrix4x4_math::{compose_trs_mat4, from_translation_mat4, inverse_mat4};
use fabled_math::quaternion_math::{forward_vec3, inverse_quat};
use fabled_math::vector_math::{cross, dot, normalize};

use crate::camera::{ClippingPlane, FovAxis, Oblique, Orthographic, Perspective, ViewPort};

#[derive(Copy, Clone, Default)]
pub struct MatrixDescriptor {
    pub projection: Matrix4x4,
    pub view: Matrix4x4,
    pub model: Matrix4x4,
}

// todo re-look at.
pub fn project(
    target: Vector3,
    viewport: ViewPort,
    matrix_descriptor: MatrixDescriptor,
) -> Vector3 {
    let t_mvp_target_vector = matrix_descriptor.projection
        * matrix_descriptor.view
        * matrix_descriptor.model
        * Vector4::set(target.x(), target.y(), target.z(), 1.0);

    let mvp_target_scalar_rcp = 1.0 / t_mvp_target_vector.w();

    let x = t_mvp_target_vector.x() * mvp_target_scalar_rcp;
    let y = -t_mvp_target_vector.y() * mvp_target_scalar_rcp;
    let z = t_mvp_target_vector.z() * mvp_target_scalar_rcp;

    let x_p_one = x + 1.0;
    let y_p_one = y + 1.0;
    let depth_diff = viewport.max_depth - viewport.min_depth;

    let half_x_p_one = x_p_one * 0.5;
    let half_y_p_one = y_p_one * 0.5;
    let z_mul_depth_diff = z * depth_diff;

    Vector3::set(
        (half_x_p_one * viewport.w) + viewport.x,
        (half_y_p_one * viewport.h) + viewport.y,
        z_mul_depth_diff + viewport.min_depth,
    )
}


// todo re-look at.
pub fn unproject(
    target: Vector3,
    viewport: ViewPort,
    matrix_descriptor: MatrixDescriptor,
) -> Vector3 {
    let matrix = inverse_mat4(
        matrix_descriptor.projection * (matrix_descriptor.view * matrix_descriptor.model),
    );

    let x = target.x() - viewport.x;
    let x_mul_two = x + x;
    let y = target.y() - viewport.y;
    let y_mul_two = y + y;

    let z = target.z() - viewport.min_depth;
    let depth_difference = (viewport.max_depth - viewport.min_depth).recip();

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

// todo this is +X=RIGHT, +Y=UP, +Z=BACK we want +Z=FORWARD.
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
    let transformation_matrix =
        compose_trs_mat4(-translation, inverse_quat(rotation), Vector3::ONE);
    let translation_to_center = from_translation_mat4(-center);

    transformation_matrix * translation_to_center
}

#[rustfmt::skip]
pub fn compute_perspective_matrix(perspective: Perspective, clipping_plane : ClippingPlane) -> Matrix4x4{
    let mut h = 0.0;
    let mut w = 0.0;
    
    let inv_near_min_far = (clipping_plane.near - clipping_plane.far).recip();

    let hal_fov = perspective.fov.radian * 0.5;
    let r = clipping_plane.far * inv_near_min_far;
    let d = r * clipping_plane.near;
    let s =hal_fov.tan() * clipping_plane.near;

    let aspect_ratio = perspective.aspect.get_aspect();
    
    match perspective.fov.axis {
        FovAxis::Horizontal => {
            w = s;
            h = s * aspect_ratio;
        }
        FovAxis::Vertical => {
            h = s;
            w = s / aspect_ratio;
        }
    }

    Matrix4x4::set(
        Vector4::set(w, 0.0, 0.0, 0.0),
        Vector4::set(0.0, h, 0.0, 0.0),
        Vector4::set(0.0, 0.0, r, -1.0),
        Vector4::set(0.0, 0.0, d, 0.0)
    )
}

pub fn compute_infinite_perspective_matrix(
    perspective: Perspective,
    clipping_plane: ClippingPlane,
) -> Matrix4x4 {
    let mut w = 0.0;
    let mut h = 0.0;

    let half_fov = perspective.fov.radian * 0.5;
    let f = clipping_plane.near * half_fov.tan();

    match perspective.fov.axis {
        FovAxis::Horizontal => {
            w = f;
            h = f * perspective.aspect.get_aspect();
        }
        FovAxis::Vertical => {
            w = f / perspective.aspect.get_aspect();
            h = f;
        }
    }

    Matrix4x4::set(
        Vector4::set(w, 0.0, 0.0, 0.0),
        Vector4::set(0.0, h, 0.0, 0.0),
        Vector4::set(0.0, 0.0, -1.0, -1.0),
        Vector4::set(0.0, 0.0, -clipping_plane.near, 0.0),
    )
}

// todo infinite reverse projection rhs

pub fn compute_orthographic_matrix(
    orthographic: Orthographic,
    clipping_plane: ClippingPlane,
) -> Matrix4x4 {
    let neg_near_plane_min_far_plane_rcp = -(clipping_plane.near - clipping_plane.far).recip();

    let right_min_left = orthographic.right - orthographic.left;
    let right_min_left_mul_2 = right_min_left + right_min_left;
    let top_min_bot = orthographic.top - orthographic.bottom;
    let top_min_bot_mul_2 = top_min_bot + top_min_bot;

    let right_plus_left = orthographic.right + orthographic.left;
    let top_plus_bot = orthographic.top + orthographic.bottom;

    let w_x = right_plus_left * right_min_left;
    let w_y = top_plus_bot * top_min_bot;
    let w_z = clipping_plane.far * neg_near_plane_min_far_plane_rcp;

    Matrix4x4::set(
        Vector4::set(right_min_left_mul_2, 0.0, 0.0, 0.0),
        Vector4::set(0.0, top_min_bot_mul_2, 0.0, 0.0),
        Vector4::set(0.0, 0.0, neg_near_plane_min_far_plane_rcp, 0.0),
        Vector4::set(-w_x, -w_y, w_z, 1.0),
    )
}


#[rustfmt::skip]
pub fn compute_oblique_projection_matrix(orthographic : Orthographic, oblique
: Oblique, clipping_plane: ClippingPlane) -> Matrix4x4{

    let projection = compute_orthographic_matrix(orthographic, clipping_plane);

    let ortho_top_rcp = orthographic.top.recip();
    
    let (oblique_angle_sin, oblique_angle_cos_cos) = oblique.angle_rad.sin_cos();
    
    let size = oblique.vertical_position * ortho_top_rcp;
    let rotation_x = size * -oblique_angle_sin;
    let rotation_y = -size * -oblique_angle_cos_cos;

    let depth_offset_x = -oblique.depth_offset * rotation_x;
    let depth_offset_y = -oblique.depth_offset * rotation_y;

    Matrix4x4::set(
        Vector4::set(projection.column_x.x(), projection.column_x.y(), rotation_x, depth_offset_x),
        Vector4::set(projection.column_y.x(), projection.column_y.y(), rotation_y, depth_offset_y),
        projection.column_z,
        projection.column_w
    )
}
