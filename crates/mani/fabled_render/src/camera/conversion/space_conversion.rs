use fabled_math::matrix4x4_math::inverse_mat4;
use fabled_math::{Matrix4x4, Vector4};

// if w is zero then it is a direction otherwise if it is a one then it is a
// point
pub fn world_to_ndc_space(target: Vector4, model_view_projection: Matrix4x4) -> Vector4 {
    let dc = model_view_projection * target;
    let scalar = dc.w().recip();
    let ndc = dc * scalar;
    ndc
}


pub fn ndc_to_world_space(target: Vector4, model_view_projection: Matrix4x4) -> Vector4 {
    let world = inverse_mat4(model_view_projection) * target;
    let scalar = world.w().recip();
    world * scalar
}
