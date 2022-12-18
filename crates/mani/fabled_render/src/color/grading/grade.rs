use fabled_math::vector_math::{clamp, dot, pow, saturate};
use fabled_math::Vector3;

pub fn apply_lift_gamma_gain(a: Vector3, lift: Vector3, gamma: Vector3, gain: Vector3) -> Vector3 {
    let one_vec = Vector3::broadcast(1.0).value;

    let lerp_a = saturate(pow(a.value, one_vec / gamma.value));
    let res = gain.value * lerp_a + lift.value * (one_vec - lerp_a);
    Vector3 { value: res }
}


pub fn apply_asc_cdl(a: Vector3, slope: Vector3, offset: Vector3, power: Vector3) -> Vector3 {
    Vector3 {
        value: pow((a * slope + offset).value, power.value),
    }
}


// value of channel should be greater than or equal to -2 or less than or equal
// to 2.0;
pub fn channel_mixer(c: Vector3, red: Vector3, green: Vector3, blue: Vector3) -> Vector3 {
    Vector3::set(
        dot(c.value, red.value),
        dot(c.value, green.value),
        dot(c.value, blue.value),
    )
}
