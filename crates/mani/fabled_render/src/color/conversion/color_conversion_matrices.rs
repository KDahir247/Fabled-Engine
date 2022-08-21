#[rustfmt::skip]
const S_RGB_TO_XYZ: [f32; 9] = [
    0.41238656, 0.21263682, 0.01933062,
    0.35759149, 0.71518298, 0.11919716,
    0.18045049, 0.0721802, 0.950_372_6
];

#[rustfmt::skip]
const XYZ_TO_S_RGB: [f32; 9] = [
    3.241_003_3, -0.969_224_3, 0.05563942,
    -1.537_398_9, 1.875_93, -0.2040112,
    -0.49861587, 0.04155422, 1.057_148_9 
];


#[rustfmt::skip]
const REC_2020_TO_XYZ : [f32; 9] = [
	0.636_953_53, 0.262_698_35, 4.994_071e-17,
    0.144_619_18, 0.678_008_8, 0.028_073_136,
    0.168_855_86, 0.059_292_894, 1.060_827_3,
];


#[rustfmt::skip]
const XYZ_TO_REC_2020 : [f32; 9] = [
    1.716_663_5, -0.66667384, 0.01764248,
    -0.355_673_3, 1.616_455_8, -0.04277698,
    -0.25336809, 0.0157683, 0.942_243_3
];


#[rustfmt::skip]
const XYZ_TO_CIECAT16 : [f32; 9] = [
    0.401288, -0.250268, -0.002079,
    0.650173, 1.204414, 0.048952,
    -0.051461, 0.045854, 0.953127,
];

#[rustfmt::skip]
const CIECAT16_TO_XYZ : [f32; 9] = [
    1.862_067_8, 0.387_526_54, -0.015_841_499,
    -1.011_254_7, 0.621_447_44, -0.034_122_936,
    0.149_186_78, -0.008_973_985, 1.049_964_4,
];


#[rustfmt::skip]
const DCI_P3_TO_XYZ : [f32; 9] = [
	4.451_698e-1, 2.094_916_9e-1, -3.634_101_2e-17,
    2.771_344_2e-1, 7.215_952e-1, 4.706_056e-2,
    1.722_826_7e-1, 6.891_306_5e-2, 9.073_553_7e-1
];

#[rustfmt::skip]
const XYZ_TO_DCI_P3: [f32; 9] = [
    2.725_394, -0.79516803, 0.04124189,
    -1.018_003, 1.689_732_1, -0.08763902,
    -0.4401632, 0.02264719, 1.100_929_4
];


#[rustfmt::skip]
const XYZ_TO_OKLAB_LMS: [f32; 9] = [
    0.818_933, 0.361_866_74, -0.128_859_71,
    0.032_984_544, 0.929_311_9, 0.036_145_64,
    0.048_200_3, 0.264_366_27, 0.633_851_7
];

#[rustfmt::skip]
const OKLAB_LMS_TO_XYZ : [f32; 9] = [
    1.227_013_8, -0.557_8, 0.281_256_14,
    -0.040_580_18, 1.112_256_9, -0.071_676_68,
    -0.076_381_28, -0.421_481_97, 1.586_163_2,
];

#[rustfmt::skip]
const LINEAR_S_RGB_TO_OKLAB_LMS : [f32; 9] = [
    0.412_221_46, 0.211_903_5, 0.088_302_46,
    0.536_332_55, 0.680_699_5, 0.281_718_85,
    0.051_445_995, 0.107_396_96, 0.629_978_7
];


#[rustfmt::skip]
const OKLAB_LMS_TO_LINEAR_S_RGB: [f32; 9] = [
    4.076_741_7, -1.268_438, -0.0041960863,
    -3.307_711_6, 2.609_757_4, -0.703_418_6,
    0.230_969_94, -0.341_319_38, 1.707_614_7
];


#[rustfmt::skip]
const OKLAB_LMS_TO_OKLAB: [f32; 9] = [
    0.210_454_26, 1.977_998_5, 0.025_904_037,
    0.793_617_8, -2.428_592_2, 0.782_771_77,
    -0.004_072_047, 0.450_593_7, -0.808_675_77,
];


#[rustfmt::skip]
const OKLAB_OKLAB_LMS : [f32; 9] = [
    1.0000000000, 1.0000000000, 1.0000000000,
    0.396_337_78, -0.105_561_346, -0.089_484_18,
    0.215_803_76, -0.063_854_17, -1.291_485_5
];

#[rustfmt::skip]
const S_RGB_TO_REC_2020 : [f32; 9] = [
    0.472_715_62, -0.110_494_494, -0.029_514_53,
    0.423_636_8, 1.077_344_8, 0.145_147_47,
    0.071_181_275, 0.025_413_143, 0.892_463_9,
];

#[rustfmt::skip]
const REC_2020_TO_S_RGB: [f32; 9] = [
    1.933_595_1, 0.197_562_66, 0.031_814_65,
    -0.742_404_7, 0.855_928_4, -0.163_757_4,
    -0.133_079_8, -0.040_130_015, 1.122_619,
];


#[rustfmt::skip]
const S_RGB_TO_CIECAT16: [f32; 9] = [
    0.075_616_91, -0.093_808_874, -0.024_049_914,
    0.707_644_2, 1.003_160_5, 0.202_653_62,
    0.167_167_41, 0.090_648_4, 0.910_296_7,
];


#[rustfmt::skip]
const CIECAT16_TO_SRGB : [f32; 9] = [
    7.023_401_3, 0.653_153_54, 0.040_149_838,
    -4.790_22, 0.571_839_75, -0.253_862_05,
    -0.812_765_84, -0.176_889_96, 1.116_449_7,
];


#[rustfmt::skip]
const CIECAT16_REC2020 : [f32; 9] = [
    3.873_352_8, 0.250_791_58, 0.014_078_325,
    -2.303_318, 0.867_093_3, -0.094_438_13,
    -0.347_164_1, -0.096_843_37, 0.992_797_6
];

#[rustfmt::skip]
const REC2020_TO_CIECAT16 : [f32; 9] = [
    0.219_057_19, -0.064_389_475, -0.0092312672,
    0.596_577_17, 0.990_305_1, 0.085_741_3,
    0.134_794_18, 0.074_084_36, 1.012_390_4
];

// rec2020 to oaklab_lms


pub fn xyz_to_xy_y(xyz: [f32; 3]) -> [f32; 3] {
    let [x, y, z] = xyz;

    let chromatic_denominator: f32 = 1.0 / (x + y + z).max(0.00001);

    

    [x * chromatic_denominator, y * chromatic_denominator, y]
}

/// Return the same nominal range as input Y
pub fn xy_y_to_xyz(xy_y: [f32; 3]) -> [f32; 3] {
    let rcp_y = xy_y[2] / xy_y[1].max(0.00001);


    [rcp_y * xy_y[0], xy_y[2], rcp_y * (1.0 - xy_y[0] - xy_y[1])]
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
