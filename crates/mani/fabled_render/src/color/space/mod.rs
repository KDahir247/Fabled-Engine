mod hsl;
mod hsv;
mod okhsl;
mod okhsv;


use crate::color::{find_cusp, find_gamut_intersection};
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


pub(crate) fn get_st_mid(a: f32, b: f32) -> Vector2 {
    let s = 0.11516993
        + 1.0
            / (7.44778970
                + 4.15901240 * b
                + a * (-2.19557347
                    + 1.75198401 * b
                    + a * (-2.13704948 - 10.02301043 * b
                        + a * (-4.24894561 + 5.38770819 * b + 4.69891013 * a))));


    let t = 0.11239642
        + 1.0
            / (1.61320320 - 0.68124379 * b
                + a * (0.40370612
                    + 0.90148123 * b
                    + a * (-0.27087943
                        + 0.61223990 * b
                        + a * (0.00299215 - 0.45399568 * b - 0.14661872 * a))));


    Vector2::set(s, t)
}

pub(crate) fn get_cs(l: f32, a: f32, b: f32) -> Vector3 {
    let cusp = find_cusp(a, b);

    let c_max = find_gamut_intersection(a, b, l, 1.0, l);

    let st_max = to_st(cusp);

    let k = c_max / (l * st_max.x()).min((1.0 - l) * st_max.y());

    let c_mid = {
        let st_mid = get_st_mid(a, b);

        let inter = 1.0 - l;
        let c_a = l * st_mid.x();
        let c_b = inter * st_mid.y();

        let c_a_four = (c_a * c_a) * (c_a * c_a);
        let c_b_four = (c_b * c_b) * (c_b * c_b);

        0.9 * k * (1.0 / (1.0 / c_a_four + 1.0 / c_b_four)).sqrt().sqrt()
    };

    let c_0 = {
        let inter = 1.0 - l;
        let c_a = l * 0.4;
        let c_b = inter * 0.8;

        let c_a_two = c_a * c_a;
        let c_b_two = c_b * c_b;

        (1.0 / (1.0 / c_a_two + 1.0 / c_b_two)).sqrt()
    };


    Vector3::set(c_0, c_mid, c_max)
}
