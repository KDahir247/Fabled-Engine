use crate::camera::ViewPort;
use fabled_math::matrix4x4_math::inverse_mat4;
use fabled_math::{Matrix4x4, Swizzles4, Vector2, Vector3, Vector4};

pub fn world_to_ndc_space(target: Vector4, model_view_projection: Matrix4x4) -> Vector3 {
    let dc = model_view_projection * target;
    let scalar = dc.w().recip();
    let ndc = dc * scalar;
    ndc.xyz()
}


pub fn ndc_to_world_space(target: Vector4, model_view_projection: Matrix4x4) -> Vector3 {
    let world_intermediate = inverse_mat4(model_view_projection) * target;
    let scalar = world_intermediate.w().recip();
    let world = world_intermediate * scalar;
    world.xyz()
}


pub fn world_to_view(target: Vector4, view: Matrix4x4) -> Vector3 {
    let view = view * target;
    view.xyz()
}


pub fn view_to_world(target: Vector4, view: Matrix4x4) -> Vector3 {
    let world = inverse_mat4(view) * target;
    world.xyz()
}

pub fn view_to_ndc(target: Vector4, projection: Matrix4x4) -> Vector3 {
    let dc = projection * target;
    let scalar = dc.w().recip();
    let ndc = dc * scalar;
    ndc.xyz()
}


pub fn view_to_world_point(
    view_point: Vector3,
    view_projection: Matrix4x4,
    viewport: ViewPort,
) -> Vector3 {
    let point_viewport_space = view_point.trunc_vec2() / Vector2::set(viewport.w, viewport.h);

    let point_viewport_norm = (point_viewport_space + point_viewport_space) - Vector2::ONE;
    let point_cam_space = point_viewport_norm * view_point.z();

    let plane_point = Vector4::set(
        point_cam_space.x(),
        point_cam_space.y(),
        view_point.z(),
        view_point.z(),
    );

    let inv_proj_view = inverse_mat4(view_projection);

    let world_point = inv_proj_view * plane_point;

    world_point.xyz()
}

pub fn world_to_view_point(
    world_point: Vector3,
    view_projection: Matrix4x4,
    viewport: ViewPort,
) -> Vector3 {
    let point = Vector4::set(world_point.x(), world_point.y(), world_point.z(), 1.0);
    let ndc_point = world_to_ndc_space(point, view_projection);

    let viewpoint = (ndc_point.trunc_vec2() + 1.0) * 0.5 * Vector2::set(viewport.w, viewport.h);

    Vector3::set(viewpoint.x(), viewpoint.y(), world_point.z())
}

pub fn view_to_viewport_point(view_point: Vector2, viewport: ViewPort) -> Vector2 {
    let dim_vec2 = Vector2::set(viewport.w, viewport.h);

    view_point / dim_vec2
}

pub fn viewport_to_view_point(viewport_point: Vector2, viewport: ViewPort) -> Vector2 {
    let dim_vec2 = Vector2::set(viewport.w, viewport.h);

    viewport_point * dim_vec2
}

// todo don't lik how i passed view_projection and projection.
pub fn world_to_viewport_point(world_point: Vector3, view_projection: Matrix4x4) -> Vector3 {
    let point4 = Vector4::set(world_point.x(), world_point.y(), world_point.z(), 1.0);

    let res = view_projection * point4;

    let neg_w_rcp = -res.w().recip();

    let norm_res = res * neg_w_rcp;
    let view_space_intermediate = norm_res * 0.5;

    let view_space_point = view_space_intermediate + 0.5;

    let z = -res.w();

    Vector3::set(view_space_point.x(), view_space_point.y(), z)
}

pub fn viewport_to_world_point(
    viewport_point: Vector3,
    view_projection: Matrix4x4,
    projection: Matrix4x4,
) -> Vector3 {
    let proj_w = projection * Vector4::set(0.0, 0.0, viewport_point.z(), 1.0);

    let viewport_mul_2 = viewport_point + viewport_point;
    let restore_point = viewport_mul_2 - 1.0;

    let point4 = Vector4::set(
        restore_point.x(),
        restore_point.y(),
        proj_w.z() / proj_w.w(),
        1.0,
    );

    let point4_w_rcp = point4.w().recip();

    ((inverse_mat4(view_projection) * point4) * point4_w_rcp).xyz()
}
