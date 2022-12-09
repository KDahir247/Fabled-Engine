use fabled_math::vector_math::{clamp, pow, saturate};
use fabled_math::{Matrix3x3, Vector3};

// https://knarkowicz.wordpress.com/2016/01/06/aces-filmic-tone-mapping-curve/
// For the original ACES curve just multiply input (x) by 0.6
pub fn fast_tonemap(hdr: Vector3) -> Vector3 {
    let aces_hdr = hdr * 0.6;
    const A: Vector3 = Vector3::broadcast(2.51);
    const B: Vector3 = Vector3::broadcast(0.3);
    const C: Vector3 = Vector3::broadcast(2.43);
    const D: Vector3 = Vector3::broadcast(0.59);
    const E: Vector3 = Vector3::broadcast(0.14);

    let ldr_unclamped = (aces_hdr * (A * aces_hdr + B)) / (aces_hdr * (C * aces_hdr + D) + E);

    Vector3 {
        value: clamp(ldr_unclamped.value, Vector3::ZERO.value, Vector3::ONE.value),
    }
}

pub const ACES_INPUT_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(0.59719, 0.07600, 0.02840),
    Vector3::set(0.35458, 0.90834, 0.13383),
    Vector3::set(0.04823, 0.01566, 0.83777),
);


pub const ACES_OUTPUT_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(1.60475, -0.10208, -0.00327),
    Vector3::set(-0.53108, 1.10813, -0.07276),
    Vector3::set(-0.07367, -0.00605, 1.07602),
);

fn rrt_and_odt_fit(v: Vector3) -> Vector3 {
    let a = v * (v + Vector3::broadcast(0.0245786)) - Vector3::broadcast(0.000090537);
    let b = v * (Vector3::broadcast(0.983729) * v + Vector3::broadcast(0.4329510))
        + Vector3::broadcast(0.238081);
    a / b
}

pub fn tone_map(hdr: Vector3) -> Vector3 {
    let ldr = ACES_OUTPUT_MATRIX * rrt_and_odt_fit(ACES_INPUT_MATRIX * hdr);

    Vector3 {
        value: clamp(ldr.value, Vector3::ZERO.value, Vector3::ONE.value),
    }
}


// We will implement two LiftGammaGain Function and try both out. I will remove
// the one that I like least with reason.

// pub fn apply_lift_gamma_gain(a: Vector3, lift: Vector3, gamma: Vector3, gain:
// Vector3) -> Vector3 {     let lift = a * (Vector3::broadcast(1.5) -
// Vector3::broadcast(0.5) * lift)         + Vector3::broadcast(0.5) * lift
//         - Vector3::broadcast(0.5);
//     let lift_gain = lift * gain;
//     let lift_gain_gamma = Vector3 {
//         value: pow(lift_gain.value, (Vector3::broadcast(1.0) / gamma).value),
//     };
//
//     lift_gain_gamma
// }

pub fn apply_lift_gamma_gain(a: Vector3, lift: Vector3, gamma: Vector3, gain: Vector3) -> Vector3 {
    let one_vec = Vector3::broadcast(1.0).value;

    let lerp_a = saturate(pow(a.value, one_vec / gamma.value));
    let res = gain.value * lerp_a + lift.value * (one_vec - lerp_a);
    Vector3 { value: res }
}
