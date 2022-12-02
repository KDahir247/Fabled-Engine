use fabled_math::vector_math::{component_max, sqrt};
use fabled_math::{Vector3, Vector4};

// HDR Encoding decoding
pub fn screen_referred_to_rgbe(screen_referred_color: Vector3) -> Vector4 {
    let maximum_luminance = component_max(screen_referred_color.value);

    let scalar: f32 = f32::from(u8::from(maximum_luminance > 10.0f32.powf(-38.0)));

    let exponent = maximum_luminance.log2() + 128.0;

    let rgb_intermediate = 2.0f32.powf(exponent - 128.0).recip();

    let rgb = (screen_referred_color * 256.0) * (rgb_intermediate * scalar);
    let exp = exponent * scalar;

    Vector4::set(rgb.x(), rgb.y(), rgb.z(), exp)
}


pub fn rgbe_to_screen_referred(rgbe: Vector4) -> Vector3 {
    let maximum_luminance = component_max(rgbe.value);

    let scalar = f32::from(u8::from(maximum_luminance > 10.0f32.powf(-38.0)));

    let screen_referred_intermediate = 256.0 * 2.0f32.powf(rgbe.w() - 128.0);

    ((rgbe + 0.5) * (screen_referred_intermediate * scalar)).trunc_vec3()
}


pub fn linear_to_rgb_m(linear: Vector3) -> Vector4 {
    const ONE_6_RCP: f32 = 1.0 / 6.0;
    const ONE_255_RCP: f32 = 1.0 / 255.0;

    let rgb = Vector3 {
        value: sqrt(linear.value),
    } * ONE_6_RCP;

    let maximum_luminance = component_max(rgb.value);

    let multiply = (maximum_luminance * 255.0).ceil() * ONE_255_RCP;
    let rcp_multiply = multiply.recip();

    let rgb_m = rgb * rcp_multiply;

    Vector4::set(rgb_m.x(), rgb_m.y(), rgb_m.z(), multiply)
}

pub fn rgb_m_to_linear(rgbm: Vector4) -> Vector3 {
    let intermediate_step = (rgbm.w() + rgbm.w()) + (rgbm.w() + rgbm.w()) + (rgbm.w() + rgbm.w());

    let rgb_linear = rgbm.trunc_vec3() * intermediate_step;

    rgb_linear * rgb_linear
}
