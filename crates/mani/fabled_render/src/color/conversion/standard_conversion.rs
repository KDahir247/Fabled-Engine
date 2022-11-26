use crate::color::OkLab;
use fabled_math::vector_math::component_sum;
use fabled_math::{Matrix3x3, Vector3};

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


pub const XYZ_TO_CIECAT16_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(0.401288, -0.250268, -0.002079),
    Vector3::set(0.650173, 1.204414, 0.048952),
    Vector3::set(-0.051461, 0.045854, 0.953127),
);

pub const CIECAT16_TO_XYZ_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(1.862_067_8, 0.387_526_54, -0.015_841_499),
    Vector3::set(-1.011_254_7, 0.621_447_44, -0.034_122_936),
    Vector3::set(0.149_186_78, -0.008_973_985, 1.049_964_4),
);

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

pub const SRGB_TO_CIECAT16_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(0.075_616_91, -0.093_808_874, -0.024_049_914),
    Vector3::set(0.707_644_2, 1.003_160_5, 0.202_653_62),
    Vector3::set(0.167_167_41, 0.090_648_4, 0.910_296_7),
);

pub const CIECAT16_TO_SRGB_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(7.023_401_3, 0.653_153_54, 0.040_149_838),
    Vector3::set(-4.790_22, 0.571_839_75, -0.253_862_05),
    Vector3::set(-0.812_765_84, -0.176_889_96, 1.116_449_7),
);

pub const CIECAT16_REC2020_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(3.873_352_8, 0.250_791_58, 0.014_078_325),
    Vector3::set(-2.303_318, 0.867_093_3, -0.094_438_13),
    Vector3::set(-0.347_164_1, -0.096_843_37, 0.992_797_6),
);


pub const REC2020_TO_CIECAT16_MATRIX: Matrix3x3 = Matrix3x3::set(
    Vector3::set(0.219_057_19, -0.064_389_475, -0.0092312672),
    Vector3::set(0.596_577_17, 0.990_305_1, 0.085_741_3),
    Vector3::set(0.134_794_18, 0.074_084_36, 1.012_390_4),
);

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


// OkLab conversion

pub fn srgb_to_oklab(srgb: Vector3) -> OkLab {
    let tri_stimulus = SRGB_TO_XYZ_MATRIX * srgb;

    OkLab::cie_xyz_to_oklab(tri_stimulus)
}

pub fn oklab_to_srgb(ok_lab: OkLab) -> Vector3 {
    let ok_lab_lms = OKLAB_TO_OKLAB_LMS_MATRIX * ok_lab.value;

    OKLAB_LMS_TO_SRGB_MATRIX * ok_lab_lms
}

#[cfg(test)]
mod space_conversion_test {
    use crate::color::{xy_y_to_xyz, xyz_to_xy_y};

    #[test]
    fn xyz_xyy_test() {
        let xy_y = [0.642, 0.327, 22.62];

        let xyz = xy_y_to_xyz(xy_y);

        let result_xy_y = xyz_to_xy_y(xyz);

        assert!(xy_y[0].eq(&result_xy_y[0]));
        assert!(xy_y[1].eq(&result_xy_y[1]));
        assert!(xy_y[2].eq(&result_xy_y[2]));
    }
}
