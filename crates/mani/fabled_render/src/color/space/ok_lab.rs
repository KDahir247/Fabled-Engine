// Rough draft of oklab color space
// do I want the logic in the struct or separate like the math folder?

use fabled_math::vector_math::{length, pow};
use fabled_math::{Matrix3x3, Swizzles3, Vector3};

use std::fmt::{Display, Formatter};

// Oklab uses D65 white point.
#[derive(Copy, Clone, PartialEq)]
pub struct OkLab {
    // L : perceived lightness.
    // a : how green/red the color is
    // b : how blue/yellow the color is
    pub value: Vector3,
}

impl OkLab {
    pub const fn new(l: f32, a: f32, b: f32) -> OkLab {
        OkLab {
            value: Vector3::set(l, a, b),
        }
    }

    pub fn to_lch(self) -> Vector3 {
        let lightness = self.value.x();

        let ab_vector = Vector3 {
            value: self.value.yzx().trunc_vec2().to_simd(),
        };

        let chroma = length(ab_vector.value);
        let hue = ab_vector.y().atan2(ab_vector.x());


        Vector3::set(lightness, chroma, hue)
    }

    pub fn from_lch(lch: Vector3) -> OkLab {
        let (hue_sin, hue_cos) = lch.z().sin_cos();
        let chroma = lch.y();
        let l = lch.x();
        let a = chroma * hue_cos;
        let b = chroma * hue_sin;

        OkLab {
            value: Vector3::set(l, a, b),
        }
    }


    pub fn cie_xyz_to_oklab(xyz: Vector3) -> OkLab {
        const M1: Matrix3x3 = Matrix3x3::set(
            Vector3::set(0.8189330101, 0.0329845436, 0.0482003018),
            Vector3::set(0.3618667424, 0.9293118715, 0.2643662691),
            Vector3::set(-0.1288597137, 0.0361456387, 0.6338517070),
        );

        const M2: Matrix3x3 = Matrix3x3::set(
            Vector3::set(0.2104542553, 1.9779984951, 0.0259040371),
            Vector3::set(0.7936177850, -2.4285922050, 0.7827717662),
            Vector3::set(-0.0040720468, 0.4505937099, -0.8086757660),
        );

        let lms_cone = M1 * xyz;

        const CUBE_ROOT_EXPO: Vector3 = Vector3::broadcast(0.33333333333333333333333333333333);

        let non_linearity_lms_cone = Vector3 {
            value: pow(lms_cone.value, CUBE_ROOT_EXPO.value),
        };

        let lab_vec3 = M2 * non_linearity_lms_cone;

        OkLab { value: lab_vec3 }
    }

    pub fn oklab_to_cie_xyz(self) -> Vector3 {
        const M1_INVERSE: Matrix3x3 = Matrix3x3::set(
            Vector3::set(1.2270138, -0.040580176, -0.07638129),
            Vector3::set(-0.5578, 1.1122569, -0.42148197),
            Vector3::set(0.28125614, -0.07167668, 1.5861632),
        );

        const M2_INVERSE: Matrix3x3 = Matrix3x3::set(
            Vector3::set(1.0000001, 1.0, 1.0000001),
            Vector3::set(0.39633784, -0.105561346, -0.08948418),
            Vector3::set(0.21580376, -0.06385418, -1.2914855),
        );

        let non_linearity_lms_cone = M2_INVERSE * self.value;

        const THREE_VEC3: Vector3 = Vector3::broadcast(3.0);

        let lms_cone = Vector3 {
            value: pow(non_linearity_lms_cone.value, THREE_VEC3.value),
        };

        M1_INVERSE * lms_cone
    }

    pub fn linear_srgb_to_oklab(linear_srgb: Vector3) -> OkLab {
        let long_cone = 0.4122214708 * linear_srgb.x()
            + 0.5363325363 * linear_srgb.y()
            + 0.0514459929 * linear_srgb.z();

        let medium_cone = 0.2119034982 * linear_srgb.x()
            + 0.6806995451 * linear_srgb.y()
            + 0.1073969566 * linear_srgb.z();
        let small_cone = 0.0883024619 * linear_srgb.x()
            + 0.2817188376 * linear_srgb.y()
            + 0.6299787005 * linear_srgb.z();

        let lms_cone = Vector3::set(long_cone, medium_cone, small_cone);

        const CUBE_ROOT_EXPO: Vector3 = Vector3::broadcast(0.33333333333333333333333333333333);

        let non_linearity_lms_cone = Vector3 {
            value: pow(lms_cone.value, CUBE_ROOT_EXPO.value),
        };

        let lightness = 0.2104542553 * non_linearity_lms_cone.x()
            + 0.7936177850 * non_linearity_lms_cone.y()
            - 0.0040720468 * non_linearity_lms_cone.z();

        let a = 1.9779984951 * non_linearity_lms_cone.x()
            - 2.4285922050 * non_linearity_lms_cone.y()
            + 0.4505937099 * non_linearity_lms_cone.z();

        let b = 0.0259040371 * non_linearity_lms_cone.x()
            + 0.7827717662 * non_linearity_lms_cone.y()
            - 0.8086757660 * non_linearity_lms_cone.z();

        OkLab {
            value: Vector3::set(lightness, a, b),
        }
    }

    pub fn oklab_to_linear_srgb(ok_lab: OkLab) -> Vector3 {
        let non_linearity_large_cone =
            ok_lab.value.x() + 0.3963377774 * ok_lab.value.y() + 0.2158037573 * ok_lab.value.z();

        let non_linearity_medium_cone =
            ok_lab.value.x() - 0.1055613458 * ok_lab.value.y() - 0.0638541728 * ok_lab.value.z();


        let non_linearity_small_cone =
            ok_lab.value.x() - 0.0894841775 * ok_lab.value.y() - 1.2914855480 * ok_lab.value.z();

        let large_cone =
            non_linearity_large_cone * non_linearity_large_cone * non_linearity_medium_cone;
        let medium_cone =
            non_linearity_medium_cone * non_linearity_medium_cone * non_linearity_medium_cone;
        let small_cone =
            non_linearity_small_cone * non_linearity_small_cone * non_linearity_small_cone;

        let red =
            4.0767416621 * large_cone - 3.3077115913 * medium_cone + 0.2309699292 * small_cone;
        let green =
            -1.2684380046 * large_cone + 2.6097574011 * medium_cone - 0.3413193965 * small_cone;
        let blue =
            -0.0041960863 * large_cone - 0.7034186147 * medium_cone + 1.7076147010 * small_cone;

        Vector3::set(red, green, blue)
    }
}


impl Display for OkLab {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OkLab(L : {}, a : {}, b : {})",
            self.value.x(),
            self.value.y(),
            self.value.z()
        )
    }
}
