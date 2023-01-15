use fabled_math::{Matrix4x4, Quaternion, Vector3, Vector4};

use fabled_math::matrix4x4_math::inverse_mat4;
use fabled_math::quaternion_math::forward_vec3;
use fabled_math::vector_math::{cross, dot, normalize};

use crate::camera::{AspectRatio, ClippingPlane, Fov, FovAxis, Oblique, ViewPort};

pub fn project(
    target: Vector3,
    viewport: ViewPort,
    clipping_plane: ClippingPlane,
    model_view_projection: Matrix4x4,
) -> Vector3 {
    let t_mvp_target_vector =
        model_view_projection * Vector4::set(target.x(), target.y(), target.z(), 1.0);

    let mvp_target_scalar_rcp = 1.0 / t_mvp_target_vector.w();

    let x = t_mvp_target_vector.x() * mvp_target_scalar_rcp;
    let y = -t_mvp_target_vector.y() * mvp_target_scalar_rcp;
    let z = t_mvp_target_vector.z() * mvp_target_scalar_rcp;

    let x_p_one = x + 1.0;
    let y_p_one = y + 1.0;
    let depth_diff = clipping_plane.far - clipping_plane.near;

    let half_x_p_one = x_p_one * 0.5;
    let half_y_p_one = y_p_one * 0.5;
    let z_mul_depth_diff = z * depth_diff;

    Vector3::set(
        (half_x_p_one * viewport.rect.z()) + viewport.rect.x(),
        (half_y_p_one * viewport.rect.w()) + viewport.rect.y(),
        z_mul_depth_diff + clipping_plane.near,
    )
}

pub fn un_project(
    target: Vector3,
    viewport: ViewPort,
    clipping_plane: ClippingPlane,
    model_view_projection: Matrix4x4,
) -> Vector3 {
    let inverse_mvp = inverse_mat4(model_view_projection);

    let x = target.x() - viewport.rect.x();
    let x_mul_two = x + x;
    let y = target.y() - viewport.rect.y();
    let y_mul_two = y + y;

    let z = target.z() - clipping_plane.near;
    let depth_difference = (clipping_plane.far - clipping_plane.near).recip();

    let z_mul_depth_diff = z * depth_difference;
    let x_two_half_width = x_mul_two / viewport.rect.z();
    let y_two_half_height = y_mul_two / viewport.rect.w();

    let vector_x = x_two_half_width - 1.0;
    let vector_y = -(y_two_half_height - 1.0);

    let vector = inverse_mvp * Vector4::set(vector_x, vector_y, z_mul_depth_diff, 1.0);

    vector.trunc_vec3()
}

// todo this is +X=RIGHT, +Y=UP, +Z=BACK we want +Z=FORWARD.
pub fn compute_look_at_matrix(
    translation: Vector3,
    up_vector: Vector3,
    rotation: Quaternion,
) -> Matrix4x4 {
    let forward = forward_vec3(rotation);

    let neg_translation = -translation;

    let z_axis = Vector3 {
        value: normalize(-forward.value),
    };
    let x_axis = Vector3 {
        value: normalize(cross(up_vector.value, z_axis.value)),
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

pub fn compute_perspective_matrix(
    aspect_ratio: AspectRatio,
    fov: Fov,
    clipping_plane: ClippingPlane,
) -> Matrix4x4 {
    let mut h = 0.0;
    let mut w = 0.0;

    let inv_near_min_far = (clipping_plane.near - clipping_plane.far).recip();

    let hal_fov = fov.radian * 0.5;
    let r = clipping_plane.far * inv_near_min_far;
    let d = r * clipping_plane.near;
    let s = hal_fov.tan() * clipping_plane.near;

    let aspect_ratio = aspect_ratio.get_aspect();

    match fov.axis {
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
        Vector4::set(0.0, 0.0, d, 0.0),
    )
}

pub fn compute_infinite_perspective_matrix(
    aspect_ratio: AspectRatio,
    fov: Fov,
    clipping_plane: ClippingPlane,
) -> Matrix4x4 {
    let mut w = 0.0;
    let mut h = 0.0;

    let half_fov = fov.radian * 0.5;
    let f = clipping_plane.near * half_fov.tan();

    match fov.axis {
        FovAxis::Horizontal => {
            w = f;
            h = f * aspect_ratio.get_aspect();
        }
        FovAxis::Vertical => {
            w = f / aspect_ratio.get_aspect();
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

pub fn perspective_infinite_reverse_projection(
    aspect_ratio: AspectRatio,
    fov: Fov,
    clipping_plane: ClippingPlane,
) -> Matrix4x4 {
    let mut w = 0.0;
    let mut h = 0.0;

    let half_fov = fov.radian * 0.5;
    let f = clipping_plane.near * half_fov.tan();

    match fov.axis {
        FovAxis::Horizontal => {
            w = f;
            h = f * aspect_ratio.get_aspect();
        }
        FovAxis::Vertical => {
            w = f / aspect_ratio.get_aspect();
            h = f;
        }
    }

    Matrix4x4::set(
        Vector4::set(w, 0.0, 0.0, 0.0),
        Vector4::set(0.0, h, 0.0, 0.0),
        Vector4::set(0.0, 0.0, 0.0, -1.0),
        Vector4::set(0.0, 0.0, clipping_plane.near, 0.0),
    )
}

pub fn compute_orthographic_matrix(
    orientation: Vector4,
    clipping_plane: ClippingPlane,
) -> Matrix4x4 {
    let neg_near_plane_min_far_plane_rcp = -(clipping_plane.near - clipping_plane.far).recip();

    let right_min_left = orientation.x() - orientation.y();
    let right_min_left_mul_2 = right_min_left + right_min_left;
    let top_min_bot = orientation.z() - orientation.w();
    let top_min_bot_mul_2 = top_min_bot + top_min_bot;

    let right_plus_left = orientation.x() + orientation.y();
    let top_plus_bot = orientation.z() + orientation.w();

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
pub fn compute_oblique_projection_matrix(orientation : Vector4,clipping_plane : ClippingPlane, oblique: Oblique) -> Matrix4x4{

    let projection = compute_orthographic_matrix(orientation, clipping_plane);

    let orthographic_top_rcp = orientation.z().recip();
    
    let (oblique_angle_sin, oblique_angle_cos_cos) = oblique.value.x().sin_cos();
    
    let size = oblique.value.y() * orthographic_top_rcp;
    let rotation_x = size * -oblique_angle_sin;
    let rotation_y = -size * -oblique_angle_cos_cos;

    let depth_offset_x = -oblique.value.z() * rotation_x;
    let depth_offset_y = -oblique.value.z() * rotation_y;

    Matrix4x4::set(
        Vector4::set(projection.column_x.x(), projection.column_x.y(), rotation_x, depth_offset_x),
        Vector4::set(projection.column_y.x(), projection.column_y.y(), rotation_y, depth_offset_y),
        projection.column_z,
        projection.column_w
    )
}
