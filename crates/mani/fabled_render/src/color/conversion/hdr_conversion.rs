use crate::color::{pq_eotf, pq_oetf, SRGB_TO_XYZ_MATRIX, XYZ_TO_SRGB_MATRIX};
use fabled_math::vector_math::{component_max, sqrt};
use fabled_math::{Matrix3x3, Vector3, Vector4};

// HDR Encoding decoding

const ICTCP_TO_ICTCP_NORM_LMS_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(1.0, 1.0, 1.0),
    Vector3::set(0.009, -0.009, 0.56),
    Vector3::set(0.111, -0.111, -0.321),
);

pub const ICTCP_NORM_LMS_TO_ICTCP_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(0.5, 0.5, 0.0),
    Vector3::set(1.614, -3.323, 1.710),
    Vector3::set(4.378, -4.246, -0.135),
);

const ICTCP_LMS_TO_XYZ_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(2.071, 0.365, -0.049),
    Vector3::set(-1.327, 0.681, -0.050),
    Vector3::set(0.207, -0.045, 1.188),
);

const XYZ_TO_ICTCP_LMS_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(0.359, -0.192, 0.007),
    Vector3::set(0.696, 1.100, 0.075),
    Vector3::set(-0.036, 0.075, 0.843),
);

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

// HDR conversion

// ICTCP is defined by Rec. 2100 as being derived from linear RGB
pub fn srgb_to_ictcp(linear_srgb: Vector3) -> Vector3 {
    let xyz = SRGB_TO_XYZ_MATRIX * linear_srgb;

    let lms = XYZ_TO_ICTCP_LMS_MATRIX * xyz;

    let non_linearity_lms = pq_oetf(lms);

    ICTCP_NORM_LMS_TO_ICTCP_MATRIX * non_linearity_lms
}


pub fn ictcp_to_srgb(ictcp: Vector3) -> Vector3 {
    let non_linear_lms = ICTCP_TO_ICTCP_NORM_LMS_MATRIX * ictcp;

    let lms = pq_eotf(non_linear_lms);

    let xyz = ICTCP_LMS_TO_XYZ_MATRIX * lms;

    XYZ_TO_SRGB_MATRIX * xyz
}
