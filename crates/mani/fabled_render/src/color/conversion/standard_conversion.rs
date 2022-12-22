use crate::color::component::ColorSpaceAdaption;
use crate::color::{compute_adaption_matrix, eotf_s_rgb, oetf_s_rgb};
use fabled_math::vector_math::{component_sum, length, pow};
use fabled_math::{Matrix3x3, Swizzles3, Vector3};

pub const SRGB_TO_XYZ_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(0.41238656, 0.21263682, 0.01933062),
    Vector3::set(0.35759149, 0.71518298, 0.11919716),
    Vector3::set(0.18045049, 0.0721802, 0.950_372_6),
);

pub const XYZ_TO_SRGB_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(3.241_003_3, -0.969_224_3, 0.05563942),
    Vector3::set(-1.537_398_9, 1.875_93, -0.2040112),
    Vector3::set(-0.49861587, 0.04155422, 1.057_148_9),
);

pub const XYZ_TO_OKLAB_LMS_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(0.818_933, 0.361_866_74, -0.128_859_71),
    Vector3::set(0.032_984_544, 0.929_311_9, 0.036_145_64),
    Vector3::set(0.048_200_3, 0.264_366_27, 0.633_851_7),
);

pub const OKLAB_LMS_TO_XYZ_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(1.227_013_8, -0.557_8, 0.281_256_14),
    Vector3::set(-0.040_580_18, 1.112_256_9, -0.071_676_68),
    Vector3::set(-0.076_381_28, -0.421_481_97, 1.586_163_2),
);

pub const SRGB_TO_OKLAB_LMS_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(0.412_221_46, 0.211_903_5, 0.088_302_46),
    Vector3::set(0.536_332_55, 0.680_699_5, 0.281_718_85),
    Vector3::set(0.051_445_995, 0.107_396_96, 0.629_978_7),
);

pub const OKLAB_LMS_TO_SRGB_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(4.076_741_7, -1.268_438, -0.0041960863),
    Vector3::set(-3.307_711_6, 2.609_757_4, -0.703_418_6),
    Vector3::set(0.230_969_94, -0.341_319_38, 1.707_614_7),
);

pub const OKLAB_LMS_TO_OKLAB: Matrix3x3 = Matrix3x3::set(
    Vector3::set(0.210_454_26, 1.977_998_5, 0.025_904_037),
    Vector3::set(0.793_617_8, -2.428_592_2, 0.782_771_77),
    Vector3::set(-0.004_072_047, 0.450_593_7, -0.808_675_77),
);

pub const OKLAB_TO_OKLAB_LMS_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(1.0, 1.0, 1.0),
    Vector3::set(0.396_337_78, -0.105_561_346, -0.089_484_18),
    Vector3::set(0.215_803_76, -0.063_854_17, -1.291_485_5),
);

pub const S_RGB_TO_REC_2020_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(0.472_715_62, -0.110_494_494, -0.029_514_53),
    Vector3::set(0.423_636_8, 1.077_344_8, 0.145_147_47),
    Vector3::set(0.071_181_275, 0.025_413_143, 0.892_463_9),
);

pub const REC_2020_TO_SRGB: Matrix3x3 = Matrix3x3::set(
    Vector3::set(1.933_595_1, 0.197_562_66, 0.031_814_65),
    Vector3::set(-0.742_404_7, 0.855_928_4, -0.163_757_4),
    Vector3::set(-0.133_079_8, -0.040_130_015, 1.122_619),
);

// Mix from hdr to ldr
pub const REC2020_TO_OKLAB_LMS_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(2.1063697, 0.027053744, -0.0013475552),
    Vector3::set(0.19839457, 1.797914, 0.018029725),
    Vector3::set(-0.07126133, -0.0011302194, 1.4945515),
);

pub const OKLAB_LMS_TO_REC2020_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(0.5216223, 0.05233287, -0.021758718),
    Vector3::set(0.008664985, 0.6300816, 0.0021431795),
    Vector3::set(0.0000000000000000024071572, 0.0074215904, 0.67240715),
);

pub const DCI_P3_TO_SRGB_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(1.1239159, -0.28434533, 0.0074421195),
    Vector3::set(-0.2164649, 1.2084676, -0.006325802),
    Vector3::set(-0.008508628, 0.0026994809, 1.0462931),
);

pub const SRGB_TO_DCI_P3_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(1.4427968, -0.2686054, 0.009585707),
    Vector3::set(-0.3220723, 1.3536621, -0.014059037),
    Vector3::set(0.000000000000000018120205, 0.0019555648, 0.95920974),
);

// primitive conversion
pub fn xy_y_to_xyz(xy_y: Vector3) -> Vector3 {
    // Y / y
    let a = xy_y.z() / xy_y.y();
    let b = 1.0 - xy_y.x() - xy_y.y();

    // x * (Y / y)
    let x = xy_y.x() * a;
    // Z = (1-x-y)Y / y
    let z = b * a;
    // Y = Y
    let y = xy_y.z();

    Vector3::set(x, y, z)
}

pub fn xyz_to_xy_y(xyz: Vector3) -> Vector3 {
    // 1.0 / (X + Y + Z)
    let intermediate = 1.0 / (component_sum(xyz.value));

    // x = X / (X + Y + Z)
    let x = xyz.x() * intermediate;
    // y = Y / (X + Y + Z)
    let y = xyz.y() * intermediate;
    // Y = Y
    let _y = xyz.y();

    Vector3::set(x, y, _y)
}


pub fn srgb_to_xyz(
    srgb_nonlinear: Vector3,
    src_tristimulus_white_point: Vector3,
    dst_tristmulus_white_point: Vector3,
) -> Vector3 {
    let srgb_linear = eotf_s_rgb(srgb_nonlinear);

    let mut tri_stimulus = Vector3::ZERO;

    if src_tristimulus_white_point != dst_tristmulus_white_point {
        let adapted_matrix = compute_adaption_matrix(
            src_tristimulus_white_point,
            dst_tristmulus_white_point,
            ColorSpaceAdaption {
                tri_stimulus_matrix: SRGB_TO_XYZ_MATRIX,
                adaption_matrix: Default::default(),
            },
        );
        tri_stimulus = adapted_matrix * srgb_linear;
    } else {
        tri_stimulus = SRGB_TO_XYZ_MATRIX * srgb_linear;
    }

    tri_stimulus
}

pub fn xyz_to_srgb(
    tri_stimulus: Vector3,
    src_tristimulus_white_point: Vector3,
    dst_tristmulus_white_point: Vector3,
) -> Vector3 {
    let mut srgb_linear = Vector3::ZERO;

    if src_tristimulus_white_point != dst_tristmulus_white_point {
        let adapted_matrix = compute_adaption_matrix(
            src_tristimulus_white_point,
            dst_tristmulus_white_point,
            ColorSpaceAdaption {
                tri_stimulus_matrix: XYZ_TO_SRGB_MATRIX,
                adaption_matrix: Default::default(),
            },
        );

        srgb_linear = adapted_matrix * tri_stimulus;
    } else {
        srgb_linear = XYZ_TO_SRGB_MATRIX * tri_stimulus;
    }

    oetf_s_rgb(srgb_linear)
}


// OkLab conversion
pub fn oklab_to_xyz(oklab: Vector3) -> Vector3 {
    let lms_oklab = OKLAB_TO_OKLAB_LMS_MATRIX * oklab;

    let pow_3_lms_oklab = pow(lms_oklab.value, Vector3::broadcast(3.0).value);

    OKLAB_LMS_TO_XYZ_MATRIX
        * Vector3 {
            value: pow_3_lms_oklab,
        }
}

pub fn xyz_to_oklab(tri_stimulus: Vector3) -> Vector3 {
    let lms_oklab = XYZ_TO_OKLAB_LMS_MATRIX * tri_stimulus;

    let cbrt_lms_oklab = pow(
        lms_oklab.value,
        Vector3::broadcast(0.33333333333333333333333333333333).value,
    );

    OKLAB_LMS_TO_OKLAB
        * Vector3 {
            value: cbrt_lms_oklab,
        }
}

pub fn oklab_to_srgb(oklab: Vector3) -> Vector3 {
    let lms_oklab = OKLAB_TO_OKLAB_LMS_MATRIX * oklab;

    let pow_3_lms_oklab = pow(lms_oklab.value, Vector3::broadcast(3.0).value);

    OKLAB_LMS_TO_SRGB_MATRIX
        * Vector3 {
            value: pow_3_lms_oklab,
        }
}

pub fn srgb_to_oklab(srgb: Vector3) -> Vector3 {
    let lms_oklab = SRGB_TO_OKLAB_LMS_MATRIX * srgb;

    let cbrt_lms_oklab = pow(
        lms_oklab.value,
        Vector3::broadcast(0.33333333333333333333333333333333).value,
    );

    OKLAB_LMS_TO_OKLAB
        * Vector3 {
            value: cbrt_lms_oklab,
        }
}

pub fn oklab_to_lch(oklab: Vector3) -> Vector3 {
    let lightness = oklab.x();

    let ab_vector = Vector3 {
        value: oklab.yzx().trunc_vec2().to_simd(),
    };

    let chroma = length(ab_vector.value);
    let hue = ab_vector.y().atan2(ab_vector.x());


    Vector3::set(lightness, chroma, hue)
}

pub fn ok_lab_from_lch(lch: Vector3) -> Vector3 {
    let (hue_sin, hue_cos) = lch.z().sin_cos();
    let chroma = lch.y();
    let l = lch.x();
    let a = chroma * hue_cos;
    let b = chroma * hue_sin;

    Vector3::set(l, a, b)
}
