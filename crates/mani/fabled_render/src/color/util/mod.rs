mod blend;
mod math_op;

pub use blend::*;
use fabled_math::Vector3;
pub use math_op::*;

// 0 to 1 linear srgb
pub fn multiply_blend(a: Vector3, b: Vector3) -> Vector3 {
    a * b
}

pub fn screen_blend(a: Vector3, b: Vector3) -> Vector3 {
    let a = -(a - 1.0);
    let b = -(b - 1.0);
    1.0 - a * b
}
