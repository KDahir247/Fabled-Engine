use fabled_math::vector_math::{component_max, exp2, ge, gt, le, log2, lt, select, sqrt};
use fabled_math::{Bool3, Matrix3x3, Vector3, Vector4};

pub const DCI_P3_TO_XYZ_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(4.451_698e-1, 2.094_916_9e-1, -3.634_101_2e-17),
    Vector3::set(2.771_344_2e-1, 7.215_952e-1, 4.706_056e-2),
    Vector3::set(1.722_826_7e-1, 6.891_306_5e-2, 9.073_553_7e-1),
);

pub const XYZ_TO_DCI_P3_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(2.725_394, -0.79516803, 0.04124189),
    Vector3::set(-1.018_003, 1.689_732_1, -0.08763902),
    Vector3::set(-0.4401632, 0.02264719, 1.100_929_4),
);

pub const REC_2020_TO_XYZ_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(0.636_953_53, 0.262_698_35, 4.994_071e-17),
    Vector3::set(0.144_619_18, 0.678_008_8, 0.028_073_136),
    Vector3::set(0.168_855_86, 0.059_292_894, 1.060_827_3),
);

pub const XYZ_TO_REC_2020_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(1.716_663_5, -0.66667384, 0.01764248),
    Vector3::set(-0.355_673_3, 1.616_455_8, -0.04277698),
    Vector3::set(-0.25336809, 0.0157683, 0.942_243_3),
);


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


// convert aces2025 to acescct
pub fn acescct_from_linear(lin: Vector3) -> Vector3 {
    const RCP_17_52: f32 = 1.0 / 17.52;

    let lin_log2 = Vector3 {
        value: log2(lin.value),
    };

    let a = lin_log2 + 9.72;

    let min = lin * 10.5402377416545;

    let max = a * RCP_17_52;

    let mask = le(lin.value, Vector3::broadcast(0.0078125).value);

    Vector3 {
        value: select((min + 0.0729055341958355).value, max.value, mask),
    }
}

pub fn linear_from_acescct(cct: Vector3) -> Vector3 {
    // not exactly 10.5 but this specific value is need for acescct conversion.
    const RCP_10_5: f32 = 1.0 / 10.5402377416545;

    let max = Vector3 {
        value: exp2((cct * 17.52 - 9.72).value),
    };

    let min = (cct - 0.0729055341958355) * RCP_10_5;

    Vector3 {
        value: select(
            max.value,
            min.value,
            gt(cct.value, Vector3::broadcast(0.155251141552511).value),
        ),
    }
}

// When ACES values are matrixed into the smaller AP1 space, colors outside
// the AP1 gamut can generate negative values even before the log encoding.
// store negative value to have a lossless conversion if needed.
pub fn acescc_from_linear(lin: Vector3) -> Vector3 {
    // we will completely disregard if lin is zero and we are getting the log2 from
    // it. This would not cause and error, but will give back inf we handle the
    // case below.
    const RCP_17_52: f32 = 1.0 / 17.52;


    let min = (Vector3 {
        value: log2((lin * 0.5 + 0.0000152587890625).value),
    } + 9.72)
        * RCP_17_52;

    let max = (Vector3 {
        value: log2(lin.value),
    } + 9.72)
        * RCP_17_52;


    let aces_cc_unhandled = select(
        min.value,
        max.value,
        lt(lin.value, Vector3::broadcast(0.000030517578125).value),
    );

    Vector3 {
        value: select(
            Vector3::broadcast(-0.3584474886).value,
            aces_cc_unhandled,
            le(lin.value, Vector3::ZERO.value),
        ),
    }
}

pub fn linear_from_acescc(cc: Vector3) -> Vector3 {
    let max = Vector3 {
        value: exp2((cc * 17.52 - 9.72).value),
    };

    let min = (max - 0.0000152587890625) + (max - 0.0000152587890625);

    let max = max;

    let linear_unhandled = select(
        min.value,
        max.value,
        lt(cc.value, Vector3::broadcast(-0.3013698630).value),
    );

    Vector3 {
        value: select(
            Vector3::broadcast(65504.0).value,
            linear_unhandled,
            ge(
                cc.value,
                Vector3::broadcast(1.4679962899543378995433789954338).value,
            ),
        ),
    }
}

pub fn linear_to_acescg(lin: Vector3) -> Vector3 {
    AP0_TO_AP1_MATRIX * lin
}

pub fn acescg_to_linear(acescg: Vector3) -> Vector3 {
    AP1_TO_AP0_MATRIX * acescg
}

pub fn standard_to_aces(srgb: Vector3) -> Vector3 {
    SRGB_TO_AP1_MATRIX * srgb
}

pub fn aces_to_standard(acescg: Vector3) -> Vector3 {
    AP1_TO_SRGB_MATRIX * acescg
}

// HDR Encoding decoding
pub fn screen_referred_to_rgbe(screen_referred_color: Vector3) -> Vector4 {
    let maximum_luminance = component_max(screen_referred_color.value);

    let log2_max_luminance = maximum_luminance.log2();

    let luminance_mask = Bool3::broadcast(maximum_luminance > 1.0e-38);

    let exponent = log2_max_luminance + 128.0;

    let rgb_intermediate = log2_max_luminance.exp2().recip();

    let rgb = screen_referred_color * 256.0 * rgb_intermediate;

    Vector4 {
        value: select(
            Vector4::set(rgb.x(), rgb.y(), rgb.z(), exponent).value,
            Vector4::ZERO.value,
            luminance_mask.value,
        ),
    }
}


pub fn rgbe_to_screen_referred(rgbe: Vector4) -> Vector3 {
    let maximum_luminance = component_max(rgbe.value);

    let luminance_mask = Bool3::broadcast(maximum_luminance > 1.0e-38);

    let modified_rgbe = rgbe + 0.5;

    let half_exposure = rgbe.w() - 128.0;

    let screen_referred_intermediate = 256.0 * half_exposure.exp2();
    let rgb_representation = (modified_rgbe * screen_referred_intermediate).trunc_vec3();

    Vector3 {
        value: select(
            rgb_representation.value,
            Vector3::ZERO.value,
            luminance_mask.value,
        ),
    }
}

pub fn linear_to_rgb_m(linear: Vector3) -> Vector4 {
    const RCP_6: f32 = 1.0 / 6.0;

    let rgb = Vector3 {
        value: sqrt(linear.value),
    } * RCP_6;

    let maximum_luminance = component_max(rgb.value).ceil();

    let rcp_maximum_luminance = maximum_luminance.recip();

    let rgb_m = rgb * rcp_maximum_luminance;

    Vector4::set(rgb_m.x(), rgb_m.y(), rgb_m.z(), maximum_luminance)
}

pub fn rgb_m_to_linear(rgbm: Vector4) -> Vector3 {
    let multiplier = rgbm.w();

    let rgb = rgbm.trunc_vec3();

    let multiplier_mul_2 = multiplier + multiplier;
    let multiplier_step = multiplier_mul_2 + multiplier_mul_2 + multiplier_mul_2;

    let rgb_linear = rgb * multiplier_step;

    rgb_linear * rgb_linear
}
