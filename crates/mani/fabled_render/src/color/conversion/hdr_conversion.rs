use fabled_math::vector_math::{component_max, sqrt};
use fabled_math::{Matrix3x3, Vector3, Vector4};

// ACEScg to ACES2025-1
pub const AP1_TO_AP0_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(0.695452241358568, 0.044794563352499, -0.005525882558111),
    Vector3::set(0.140678696470730, 0.859671118442968, 0.004025210305977),
    Vector3::set(0.163869062213569, 0.095534318210286, 1.001500672251631),
);

// ACES2025-1 to ACEScg
pub const AP0_TO_AP1_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(1.451439316071658, -0.076553773314263, 0.008316148424961),
    Vector3::set(-0.236510746889360, 1.176229699811789, -0.006032449790909),
    Vector3::set(-0.214928569308364, -0.099675926450360, 0.997716301412982),
);

// SRGB to ACEScg
pub const SRGB_TO_AP1_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(0.613132422390542, 0.070124380833917, 0.020587657528185),
    Vector3::set(0.339538015799666, 0.916394011313573, 0.109574571610682),
    Vector3::set(0.047416696048269, 0.013451523958235, 0.869785404035327),
);


// ACEScg to SRGB
pub const AP1_TO_SRGB_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(1.704858676289160, -0.130076824208823, -0.023964072927574),
    Vector3::set(-0.621716021885330, 1.140735774822505, -0.128975508299318),
    Vector3::set(-0.083299371729057, -0.010559801677511, 1.153014018916862),
);

// SRGB to ACES2025-1
pub const SRGB_TO_AP0_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(0.439643004019961, 0.089715731865361, 0.017512720476296),
    Vector3::set(0.383005471371792, 0.813475053791709, 0.111551438549134),
    Vector3::set(0.177399308886895, 0.096782252404812, 0.870882792975248),
);

// ACES2025-1 to SRGB
pub const AP0_TO_SRGB_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(2.521400888578220, -0.276214061561748, -0.015320200077479),
    Vector3::set(-1.133995749382747, 1.372595566304090, -0.152992561800699),
    Vector3::set(-0.387561856768867, -0.096282355736466, 1.168387199619315),
);

// ACES2025-1 to XYZ
pub const AP0_TO_XYZ_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(0.9525523959, 0.3439664498, 0.0),
    Vector3::set(0.0, 0.7281660966, 0.0),
    Vector3::set(0.0000936786, -0.0721325464, 1.0088251844),
);

// XYZ to ACES2025-1
pub const XYZ_TO_AP0_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(1.0498110175, -0.4959030231, 0.0),
    Vector3::set(0.0, 1.3733130458, 0.0),
    Vector3::set(-0.0000974845, 0.0982400361, 0.9912520182),
);

// ACEScg to XYZ
pub const AP1_TO_XYZ_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(0.6624541811, 0.2722287168, -0.0055746495),
    Vector3::set(0.1340042065, 0.6740817658, 0.0040607335),
    Vector3::set(0.1561876870, 0.0536895174, 1.0103391003),
);

// XYZ to ACEScg
pub const XYZ_TO_AP1_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(1.6410233797, -0.6636628587, 0.0117218943),
    Vector3::set(-0.3248032942, 1.6153315917, -0.0082844420),
    Vector3::set(-0.2364246952, 0.0167563477, 0.9883948585),
);


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
