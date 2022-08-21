#![feature(portable_simd)]
#![feature(link_llvm_intrinsics)]
#![feature(simd_ffi)]

extern crate core;

mod arithmetic;
mod boolean;
mod easing;
mod geometric;
mod linear;
mod transformation;

use crate::math::{cross, reverse, ror};
pub use arithmetic::*;
pub use boolean::*;
pub use easing::*;
pub use geometric::*;
pub use linear::*;

pub use transformation::*;

// temp solution.
#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.cos.v4f32"]
    fn cos_v4f32(x: std::simd::f32x4) -> std::simd::f32x4;
    #[link_name = "llvm.sin.v4f32"]
    fn sin_v4f32(x: std::simd::f32x4) -> std::simd::f32x4;
    #[link_name = "llvm.pow.v4f32"]
    fn pow_v4f32(x: std::simd::f32x4, exponent: std::simd::f32x4) -> std::simd::f32x4;
    #[link_name = "llvm.exp.v4f32"]
    fn exp_v4f32(x: std::simd::f32x4) -> std::simd::f32x4;
    #[link_name = "llvm.exp2.v4f32"]
    fn exp2_v4f32(x: std::simd::f32x4) -> std::simd::f32x4;
    #[link_name = "llvm.log.v4f32"]
    fn log_v4f32(x: std::simd::f32x4) -> std::simd::f32x4;
    #[link_name = "llvm.log2.v4f32"]
    fn log2_v4f32(x: std::simd::f32x4) -> std::simd::f32x4;
    #[link_name = "llvm.log10.v4f32"]
    fn log10_v4f32(x: std::simd::f32x4) -> std::simd::f32x4;
}

pub mod math {

    use std::simd::StdFloat;

    use crate::{
        cos_v4f32, exp2_v4f32, exp_v4f32, log10_v4f32, log2_v4f32, log_v4f32, pow_v4f32, sin_v4f32,
    };


    // todo remove std::simd::f32x4 and find a way to accept both vector 2, 3, 4 and
    //  quaternion.

    #[inline]
    pub fn sqrt(vector_simd: std::simd::f32x4) -> std::simd::f32x4 {
        vector_simd.sqrt()
    }

    #[inline]
    pub fn dot(simd_vector_1: std::simd::f32x4, simd_vector_2: std::simd::f32x4) -> f32 {
        (simd_vector_1 * simd_vector_2).reduce_sum()
    }

    #[inline]
    pub fn abs(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        simd_vector.abs()
    }

    #[inline]
    pub fn face_forward(
        normal_vector: std::simd::f32x4,
        incident_vector: std::simd::f32x4,
        geo_normal_vector: std::simd::f32x4,
    ) -> std::simd::f32x4 {
        select(
            normal_vector,
            -normal_vector,
            std::simd::mask32x4::splat(dot(incident_vector, geo_normal_vector) >= 0.0),
        )
    }

    // exp 10

    #[inline]
    pub fn exp(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        unsafe { exp_v4f32(simd_vector) }
    }

    #[inline]
    pub fn exp2(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        unsafe { exp2_v4f32(simd_vector) }
    }

    #[inline]
    pub fn cos(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        unsafe { cos_v4f32(simd_vector) }
    }

    #[inline]
    pub fn cosh(vector_simd: std::simd::f32x4) -> std::simd::f32x4 {
        let e_vec = std::simd::f32x4::splat(std::f32::consts::E);

        (pow(e_vec, vector_simd) + pow(e_vec, -vector_simd)) * std::simd::f32x4::splat(0.5)
    }

    #[inline]
    pub fn sin(vector_simd: std::simd::f32x4) -> std::simd::f32x4 {
        unsafe { sin_v4f32(vector_simd) }
    }

    // acos

    // asin


    #[inline]
    pub fn pow(vector_simd: std::simd::f32x4, exponent_simd: std::simd::f32x4) -> std::simd::f32x4 {
        unsafe { pow_v4f32(vector_simd, exponent_simd) }
    }

    #[inline]
    pub fn sinh(vector_simd: std::simd::f32x4) -> std::simd::f32x4 {
        let e_vec = std::simd::f32x4::splat(std::f32::consts::E);

        (pow(e_vec, vector_simd) - pow(e_vec, -vector_simd)) * std::simd::f32x4::splat(0.5)
    }

    // atan

    // atan2

    #[inline]
    pub fn tanh(vector_simd: std::simd::f32x4) -> std::simd::f32x4 {
        let e_vec = std::simd::f32x4::splat(std::f32::consts::E);

        let one_vec = std::simd::f32x4::splat(1.0);
        let two_vector_simd = std::simd::f32x4::splat(2.0) * vector_simd;

        (pow(e_vec, two_vector_simd) - one_vec) / (pow(e_vec, two_vector_simd) + one_vec)
    }

    #[inline]
    pub fn log(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        unsafe { log_v4f32(simd_vector) }
    }

    #[inline]
    pub fn log2(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        unsafe { log2_v4f32(simd_vector) }
    }

    #[inline]
    pub fn log10(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        unsafe { log10_v4f32(simd_vector) }
    }

    #[inline]
    pub fn smooth_step(
        min_range: std::simd::f32x4,
        max_range: std::simd::f32x4,
        simd_val: std::simd::f32x4,
    ) -> std::simd::f32x4 {
        let t = saturate((simd_val - min_range) / (max_range - min_range));
        t * t * (std::simd::f32x4::splat(3.0) - std::simd::f32x4::splat(2.0) * t)
    }

    #[inline]
    pub fn step(vec_simd1: std::simd::f32x4, vec_simd2: std::simd::f32x4) -> std::simd::f32x4 {
        vec_simd1
            .lanes_ge(vec_simd2)
            .select(std::simd::f32x4::splat(0.0), std::simd::f32x4::splat(1.0))
    }

    #[inline]
    pub fn bitmask(simd_vector: std::simd::f32x4) -> i32 {
        simd_vector.cast::<i32>().reduce_or()
    }

    // todo optimize
    #[inline]
    pub fn cross(
        simd_vector: std::simd::f32x4,
        simd_vector1: std::simd::f32x4,
    ) -> std::simd::f32x4 {
        let a: [f32; 4] = simd_vector.to_array();
        let b: [f32; 4] = simd_vector1.to_array();

        std::simd::f32x4::from_array([
            (a[1] * b[2] - a[2] * b[1]),
            (a[2] * b[0] - a[0] * b[2]),
            (a[0] * b[1] - a[1] * b[0]),
            0.0,
        ])
    }

    #[inline]
    pub fn unlerp(
        simd_vector: std::simd::f32x4,
        min_vector: std::simd::f32x4,
        max_vector: std::simd::f32x4,
    ) -> std::simd::f32x4 {
        (simd_vector - min_vector) / (max_vector - min_vector)
    }


    #[inline]
    pub fn rsqrt(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        std::simd::f32x4::splat(1.0) / simd_vector.sqrt()
    }

    #[inline]
    pub fn ceil(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        simd_vector.ceil()
    }

    #[inline]
    pub fn floor(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        simd_vector.floor()
    }

    #[inline]
    pub fn all(simd_vector: std::simd::f32x4) -> bool {
        simd_vector.lanes_gt(std::simd::f32x4::splat(0.0)).all()
    }

    #[inline]
    pub fn any(simd_vector: std::simd::f32x4) -> bool {
        simd_vector.lanes_gt(std::simd::f32x4::splat(0.0)).any()
    }

    #[inline]
    pub fn clamp(
        simd_vector: std::simd::f32x4,
        min: std::simd::f32x4,
        max: std::simd::f32x4,
    ) -> std::simd::f32x4 {
        simd_vector.clamp(min, max)
    }

    #[inline]
    pub fn component_max(simd_vector: std::simd::f32x4) -> f32 {
        simd_vector.reduce_max()
    }

    #[inline]
    pub fn component_min(simd_vector: std::simd::f32x4) -> f32 {
        simd_vector.reduce_min()
    }

    #[inline]
    pub fn component_sum(simd_vector: std::simd::f32x4) -> f32 {
        simd_vector.reduce_sum()
    }

    #[inline]
    pub fn copysign(
        simd_vector: std::simd::f32x4,
        sign_vector: std::simd::f32x4,
    ) -> std::simd::f32x4 {
        simd_vector.copysign(sign_vector)
    }


    #[inline]
    pub fn degrees(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        simd_vector.to_degrees()
    }

    #[inline]
    pub fn radians(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        simd_vector.to_radians()
    }

    #[inline]
    pub fn length(simd_vector: std::simd::f32x4) -> f32 {
        let dot_product = simd_vector * simd_vector;

        let sqr_length = dot_product.reduce_sum();

        sqr_length.sqrt()
    }

    #[inline]
    pub fn length_squared(simd_vector: std::simd::f32x4) -> f32 {
        let dot_product = simd_vector * simd_vector;

        dot_product.reduce_sum()
    }


    #[inline]
    pub fn distance(simd_vector_1: std::simd::f32x4, simd_vector_2: std::simd::f32x4) -> f32 {
        length(simd_vector_2 - simd_vector_1)
    }

    #[inline]
    pub fn distance_squared(
        simd_vector_1: std::simd::f32x4,
        simd_vector_2: std::simd::f32x4,
    ) -> f32 {
        length_squared(simd_vector_2 - simd_vector_1)
    }


    #[inline]
    pub fn fmod(
        simd_vector_1: std::simd::f32x4,
        simd_vector_2: std::simd::f32x4,
    ) -> std::simd::f32x4 {
        simd_vector_1 % simd_vector_2
    }

    #[inline]
    pub fn fract<const SIZE: usize>(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        simd_vector.fract()
    }

    #[inline]
    pub fn round(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        simd_vector.round()
    }

    #[inline]
    pub fn trunc(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        simd_vector.trunc()
    }

    #[inline]
    pub fn normalize(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        let mag = length(simd_vector);
        simd_vector / std::simd::f32x4::splat(mag)
    }

    #[inline]
    pub fn mul_add(
        simd_vector: std::simd::f32x4,
        mul_vector: std::simd::f32x4,
        add_vector: std::simd::f32x4,
    ) -> std::simd::f32x4 {
        simd_vector.mul_add(mul_vector, add_vector)
    }

    #[inline]
    pub fn rcp(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        std::simd::f32x4::splat(1.0) / simd_vector
    }

    #[inline]
    pub fn is_finite(simd_vector: std::simd::f32x4) -> std::simd::mask32x4 {
        simd_vector.is_finite()
    }

    #[inline]
    pub fn is_infinite(simd_vector: std::simd::f32x4) -> std::simd::mask32x4 {
        simd_vector.is_infinite()
    }

    #[inline]
    pub fn is_nan(simd_vector: std::simd::f32x4) -> std::simd::mask32x4 {
        simd_vector.is_nan()
    }

    #[inline]

    pub fn lerp(
        src: std::simd::f32x4,
        dst: std::simd::f32x4,
        t: std::simd::f32x4,
    ) -> std::simd::f32x4 {
        mul_add(t, dst, mul_add(-t, src, src))
    }


    #[inline]
    pub fn reflect(
        incident_vector: std::simd::f32x4,
        normal_vector: std::simd::f32x4,
    ) -> std::simd::f32x4 {
        let intermediate_step = std::simd::f32x4::splat(2.0 * dot(incident_vector, normal_vector));
        incident_vector - intermediate_step * normal_vector
    }

    #[inline]
    pub fn refract(
        incident_vector: std::simd::f32x4,
        normal_vector: std::simd::f32x4,
        eta: f32,
    ) -> std::simd::f32x4 {
        let ni = dot(normal_vector, incident_vector);
        let k = 1.0 - eta * eta * (1.0 - ni * ni);

        let eta_vec = std::simd::f32x4::splat(eta);
        let k_sqrt_vec = std::simd::f32x4::splat(k.sqrt());
        let ni_vec = std::simd::f32x4::splat(ni);
        let mask = std::simd::mask32x4::splat(k >= 0.0);

        mask.select(
            std::simd::f32x4::splat(0.0),
            eta_vec * incident_vector - (eta_vec * ni_vec + k_sqrt_vec) * normal_vector,
        )
    }

    #[inline]
    pub fn project(
        project_vector: std::simd::f32x4,
        target_vector: std::simd::f32x4,
    ) -> std::simd::f32x4 {
        let c_dot_bb = dot(project_vector, target_vector);
        let rcp_c_dot_ab = dot(target_vector, target_vector).recip();

        let projection_factor = c_dot_bb * rcp_c_dot_ab;

        let splat_project_factor = std::simd::f32x4::splat(projection_factor);

        target_vector * splat_project_factor
    }

    #[inline]
    pub fn reject(
        simd_vector: std::simd::f32x4,
        simd_vector1: std::simd::f32x4,
    ) -> std::simd::f32x4 {
        let projection_vector = project(simd_vector, simd_vector1);

        simd_vector - projection_vector
    }

    #[inline]
    pub fn saturate(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        clamp(
            simd_vector,
            std::simd::f32x4::splat(0.0),
            std::simd::f32x4::splat(1.0),
        )
    }

    #[inline]
    pub fn select(
        simd_vector1: std::simd::f32x4,
        simd_vector2: std::simd::f32x4,
        mask: std::simd::mask32x4,
    ) -> std::simd::f32x4 {
        mask.select(simd_vector1, simd_vector2)
    }

    #[inline]
    pub fn angle(simd_vector: std::simd::f32x4, simd_vector_1: std::simd::f32x4) -> f32 {
        // a . b == ||a|| ||b|| cos(theta)
        let normalized_vec = normalize(simd_vector);

        let normalized_vec1 = normalize(simd_vector_1);

        let dot_product = dot(normalized_vec, normalized_vec1);

        dot_product.acos()
    }

    #[inline]
    pub fn fast_angle(
        norm_simd_vector: std::simd::f32x4,
        norm_simd_vector1: std::simd::f32x4,
    ) -> f32 {
        let dot_product = dot(norm_simd_vector, norm_simd_vector1);

        dot_product.acos()
    }

    #[inline]
    pub fn sign(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        simd_vector.signum()
    }

    #[inline]
    pub fn reverse(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        simd_vector.reverse()
    }

    #[inline]
    pub fn ror<const OFFSET: usize>(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        simd_vector.rotate_lanes_right::<OFFSET>()
    }

    #[inline]
    pub fn rol<const OFFSET: usize>(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        simd_vector.rotate_lanes_left::<OFFSET>()
    }
}
