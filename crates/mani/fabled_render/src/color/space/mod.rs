mod hsl;
mod hsv;
mod okhsl;
mod okhsv;


use fabled_math::{Vector2, Vector3};
pub use hsl::*;
pub use hsv::*;
pub use okhsl::*;
pub use okhsv::*;


pub(crate) fn toe(x: f32) -> f32 {
    const K_1: f32 = 0.206;
    const K_2: f32 = 0.03;
    const K_2_P_ONE_RCP: f32 = 0.97087378640776699029126213592233;

    let k_3 = (1.0 + K_1) * K_2_P_ONE_RCP;
    let d = k_3 * x - K_1;

    0.5 * (d + ((d * d) + (4.0 * K_2) * (k_3 * x)).sqrt())
}

pub(crate) fn toe_inv(x: f32) -> f32 {
    const K_1: f32 = 0.206;
    const K_2: f32 = 0.03;
    const K_2_P_ONE_RCP: f32 = 0.97087378640776699029126213592233;

    let k_3 = (1.0 + K_1) * K_2_P_ONE_RCP;

    ((x * x) + (K_1 * x)) / (k_3 * (x + K_2))
}


pub(crate) fn to_st(cusp: Vector2) -> Vector2 {
    let l = cusp.x();
    let c = cusp.y();
    Vector2::set(c / l, c / (1.0 - l))
}
