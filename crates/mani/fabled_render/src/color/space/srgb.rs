use fabled_math::vector_math::{component_ge, pow, select};
use fabled_math::Vector3;

const SRGB_POW_TRANSFER_FUNCTION: f32 = 1.0 / 2.4;

const RCP_FALLOFF: f32 = 1.0 / 12.92;
const RCP_LINEAR_FALLOFF: f32 = 1.0 / 1.055;


pub fn s_rgb_to_linear(s_rgb: Vector3) -> Vector3 {
    let a = s_rgb + 0.055;

    let b = a * RCP_LINEAR_FALLOFF;
    let d = s_rgb * RCP_FALLOFF;

    let c = pow(b.value, Vector3::broadcast(2.4).value);

    let mask = component_ge(s_rgb.value, 0.04045);

    let linear = select(c, d.value, mask);

    Vector3 { value: linear }
}

pub fn linear_to_s_rgb(s_rgb: Vector3) -> Vector3 {
    let a = Vector3 {
        value: pow(
            s_rgb.value,
            Vector3::broadcast(SRGB_POW_TRANSFER_FUNCTION).value,
        ),
    };

    let b = a * 1.055;
    let d = s_rgb * 12.92;

    let c = b - 0.055;

    let mask = component_ge(s_rgb.value, 0.0031308);

    let non_linear = select(c.value, d.value, mask);

    Vector3 { value: non_linear }
}
