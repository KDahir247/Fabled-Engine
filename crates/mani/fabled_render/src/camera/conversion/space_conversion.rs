use crate::camera::ViewPort;
use fabled_math::matrix4x4_math::inverse_mat4;
use fabled_math::{Matrix4x4, Swizzles4, Vector2, Vector3, Vector4};

// if w is zero then it is a direction otherwise if it is a one then it is a
// point
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
    position: Vector2,
    projection_view: Matrix4x4,
    viewport: ViewPort,
) -> Vector3 {
    let x = 2.0 * position.x() / viewport.w - 1.0;
    let y = 2.0 * position.y() / viewport.h - 1.0;

    let screen_point = Vector4::set(x, -y, -1.0, 1.0);
    let inv_proj_view = inverse_mat4(projection_view);

    let world_point = inv_proj_view * screen_point;

    world_point.xyz()
}
