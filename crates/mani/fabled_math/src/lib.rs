#![feature(portable_simd)]
#![feature(link_llvm_intrinsics)]
#![feature(simd_ffi)]
#![feature(const_float_classify)]
#![feature(core_intrinsics)]

extern crate core;

mod boolean;
mod easing;
mod geometric;
mod linear;
mod math_trait;
mod transformation;

pub use boolean::*;
pub use easing::*;
pub use geometric::*;
pub use linear::*;

pub use transformation::*;


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

pub mod vector_math {
    use std::simd::{SimdFloat, SimdInt, SimdPartialOrd, StdFloat};

    use crate::{
        cos_v4f32, exp2_v4f32, exp_v4f32, log10_v4f32, log2_v4f32, log_v4f32, pow_v4f32, sin_v4f32,
    };

    #[inline(always)]
    pub fn degrees(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        simd_vector.to_degrees()
    }

    #[inline(always)]
    pub fn radians(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        simd_vector.to_radians()
    }

    #[inline(always)]
    pub fn rcp(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        simd_vector.recip()
    }

    #[inline(always)]
    pub fn dot(simd_vector_1: std::simd::f32x4, simd_vector_2: std::simd::f32x4) -> f32 {
        (simd_vector_1 * simd_vector_2).reduce_sum()
    }

    #[inline]
    pub fn pow(vector_simd: std::simd::f32x4, exponent_simd: std::simd::f32x4) -> std::simd::f32x4 {
        unsafe { pow_v4f32(vector_simd, exponent_simd) }
    }

    #[inline(always)]
    pub fn clamp(
        simd_vector: std::simd::f32x4,
        min: std::simd::f32x4,
        max: std::simd::f32x4,
    ) -> std::simd::f32x4 {
        simd_vector.simd_clamp(min, max)
    }

    #[inline(always)]
    pub fn length(simd_vector: std::simd::f32x4) -> f32 {
        let simd_mul = simd_vector * simd_vector;

        let dot_product = simd_mul.reduce_sum();

        dot_product.sqrt()
    }

    #[inline(always)]
    pub fn distance(simd_vector_1: std::simd::f32x4, simd_vector_2: std::simd::f32x4) -> f32 {
        let direction_vec = simd_vector_2 - simd_vector_1;

        length(direction_vec)
    }

    #[inline(always)]
    pub fn normalize(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        let length = length(simd_vector);
        let length_vector = std::simd::f32x4::from_array([length; 4]);

        simd_vector / length_vector
    }

    #[inline(always)]
    pub fn mul_add(
        simd_vector: std::simd::f32x4,
        mul_vector: std::simd::f32x4,
        add_vector: std::simd::f32x4,
    ) -> std::simd::f32x4 {
        simd_vector.mul_add(mul_vector, add_vector)
    }

    // todo add slerp
    #[inline(always)]
    pub fn slerp(src : std::simd::f32x4, dst : std::simd::f32x4 , t : f32) -> std::simd::f32x4{
        todo!()
    }

    #[inline(always)]
    pub fn lerp(
        src: std::simd::f32x4,
        dst: std::simd::f32x4,
        t: std::simd::f32x4,
    ) -> std::simd::f32x4 {
        mul_add(t, dst, mul_add(-t, src, src))
    }

    #[inline(always)]
    pub fn length_squared(simd_vector: std::simd::f32x4) -> f32 {
        let dot_product = simd_vector * simd_vector;

        dot_product.reduce_sum()
    }

    #[inline(always)]
    pub fn distance_squared(
        simd_vector_1: std::simd::f32x4,
        simd_vector_2: std::simd::f32x4,
    ) -> f32 {
        let direction_vec = simd_vector_2 - simd_vector_1;

        length_squared(direction_vec)
    }

    #[inline(always)]
    pub fn select(
        true_simd_vec: std::simd::f32x4,
        false_simd_vec: std::simd::f32x4,
        mask: std::simd::mask32x4,
    ) -> std::simd::f32x4 {
        mask.select(true_simd_vec, false_simd_vec)
    }

    #[inline(always)]
    pub fn saturate(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        const ZERO_VEC: std::simd::f32x4 = std::simd::f32x4::from_array([0.0; 4]);
        const ONE_VEC: std::simd::f32x4 = std::simd::f32x4::from_array([1.0; 4]);

        clamp(simd_vector, ZERO_VEC, ONE_VEC)
    }

    #[inline(always)]
    pub fn fract(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        simd_vector.fract()
    }

    #[inline(always)]
    pub fn round(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        simd_vector.round()
    }

    #[inline(always)]
    pub fn trunc(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        simd_vector.trunc()
    }

    #[inline(always)]
    pub fn ceil(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        simd_vector.ceil()
    }

    #[inline(always)]
    pub fn floor(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        simd_vector.floor()
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
    pub fn angle(simd_vector: std::simd::f32x4, simd_vector_1: std::simd::f32x4) -> f32 {
        let normalized_vec = normalize(simd_vector);
        let normalized_vec1 = normalize(simd_vector_1);

        let dot_product = dot(normalized_vec, normalized_vec1);

        dot_product.acos()
    }

    #[inline(always)]
    pub fn sqrt(vector_simd: std::simd::f32x4) -> std::simd::f32x4 {
        vector_simd.sqrt()
    }

    #[inline(always)]
    pub fn abs(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        simd_vector.abs()
    }


    #[inline(always)]
    pub fn rsqrt(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        const ONE_VEC: std::simd::f32x4 = std::simd::f32x4::from_array([1.0; 4]);

        ONE_VEC / simd_vector.sqrt()
    }

    #[inline(always)]
    pub fn step(vec_simd1: std::simd::f32x4, vec_simd2: std::simd::f32x4) -> std::simd::f32x4 {
        const ZERO_VEC: std::simd::f32x4 = std::simd::f32x4::from_array([0.0; 4]);
        const ONE_VEC: std::simd::f32x4 = std::simd::f32x4::from_array([1.0; 4]);

        let mask = vec_simd1.simd_ge(vec_simd2);

        select(ZERO_VEC, ONE_VEC, mask)
    }

    #[inline]
    pub fn cross(
        simd_vector: std::simd::f32x4,
        simd_vector1: std::simd::f32x4,
    ) -> std::simd::f32x4 {
        const BIT_SHUFFLE_MASK: [usize; 4] = [1, 2, 0, 3];

        // simd_vector
        let simd_yzx = std::simd::simd_swizzle!(simd_vector, BIT_SHUFFLE_MASK);
        let simd1_yzx = std::simd::simd_swizzle!(simd_vector1, BIT_SHUFFLE_MASK);

        let b = simd_vector1 * simd_yzx;
        let a = simd_vector * simd1_yzx;
        let res = a - b;

        std::simd::simd_swizzle!(res, BIT_SHUFFLE_MASK)
    }

    #[inline(always)]
    pub fn unlerp(
        simd_vector: std::simd::f32x4,
        min_vector: std::simd::f32x4,
        max_vector: std::simd::f32x4,
    ) -> std::simd::f32x4 {
        (simd_vector - min_vector) / (max_vector - min_vector)
    }

    #[inline(always)]
    pub fn all(simd_vector: std::simd::f32x4) -> bool {
        const ZERO_VEC: std::simd::f32x4 = std::simd::f32x4::from_array([0.0; 4]);

        simd_vector.simd_gt(ZERO_VEC).all()
    }

    #[inline(always)]
    pub fn any(simd_vector: std::simd::f32x4) -> bool {
        const ZERO_VEC: std::simd::f32x4 = std::simd::f32x4::from_array([0.0; 4]);

        simd_vector.simd_gt(ZERO_VEC).any()
    }

    #[inline(always)]
    pub fn component_max(simd_vector: std::simd::f32x4) -> f32 {
        simd_vector.reduce_max()
    }

    #[inline(always)]
    pub fn component_min(simd_vector: std::simd::f32x4) -> f32 {
        simd_vector.reduce_min()
    }

    #[inline(always)]
    pub fn component_sum(simd_vector: std::simd::f32x4) -> f32 {
        simd_vector.reduce_sum()
    }

    #[inline(always)]
    pub fn component_product(simd_vector: std::simd::f32x4) -> f32 {
        simd_vector.reduce_product()
    }

    #[inline]
    pub fn sign(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        simd_vector.signum()
    }

    #[inline(always)]
    pub fn copysign(
        simd_vector: std::simd::f32x4,
        sign_vector: std::simd::f32x4,
    ) -> std::simd::f32x4 {
        simd_vector.copysign(sign_vector)
    }

    #[inline(always)]
    pub fn is_positive(simd_vector: std::simd::f32x4) -> std::simd::mask32x4 {
        simd_vector.is_sign_positive()
    }

    #[inline(always)]
    pub fn is_negative(simd_vector: std::simd::f32x4) -> std::simd::mask32x4 {
        simd_vector.is_sign_negative()
    }


    #[inline(always)]
    pub fn is_finite(simd_vector: std::simd::f32x4) -> std::simd::mask32x4 {
        simd_vector.is_finite()
    }

    #[inline(always)]
    pub fn is_infinite(simd_vector: std::simd::f32x4) -> std::simd::mask32x4 {
        simd_vector.is_infinite()
    }

    #[inline(always)]
    pub fn is_nan(simd_vector: std::simd::f32x4) -> std::simd::mask32x4 {
        simd_vector.is_nan()
    }


    #[inline]
    pub fn smooth_step(
        min_range: std::simd::f32x4,
        max_range: std::simd::f32x4,
        simd_val: std::simd::f32x4,
    ) -> std::simd::f32x4 {
        const THREE_VEC: std::simd::f32x4 = std::simd::f32x4::from_array([3.0; 4]);
        const TWO_VEC: std::simd::f32x4 = std::simd::f32x4::from_array([2.0; 4]);

        let un_clamp_time = (simd_val - min_range) / (max_range - min_range);

        let time = saturate(un_clamp_time);

        let time_sqr = time * time;
        let time_mul_two = time * TWO_VEC;

        time_sqr * (THREE_VEC - time_mul_two)
    }

    #[inline]
    pub fn sin_cos(simd_vector: std::simd::f32x4) -> (std::simd::f32x4, std::simd::f32x4){
        (sin(simd_vector), cos(simd_vector))
    }

    #[inline]
    pub fn cos(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        unsafe { cos_v4f32(simd_vector) }
    }

    #[inline]
    pub fn sin(vector_simd: std::simd::f32x4) -> std::simd::f32x4 {
        unsafe { sin_v4f32(vector_simd) }
    }

    #[inline]
    pub fn exp(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        unsafe { exp_v4f32(simd_vector) }
    }

    #[inline]
    pub fn exp2(simd_vector: std::simd::f32x4) -> std::simd::f32x4 {
        unsafe { exp2_v4f32(simd_vector) }
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
    pub fn face_forward(
        normal_vector: std::simd::f32x4,
        incident_vector: std::simd::f32x4,
        geo_normal_vector: std::simd::f32x4,
    ) -> std::simd::f32x4 {
        let dot_prod = dot(incident_vector, geo_normal_vector);
        let mask: bool = dot_prod.is_sign_positive();

        select(
            normal_vector,
            -normal_vector,
            std::simd::mask32x4::splat(mask),
        )
    }

    #[inline(always)]
    pub fn project(
        project_vector: std::simd::f32x4,
        target_vector: std::simd::f32x4,
    ) -> std::simd::f32x4 {
        let c_dot_bb = dot(project_vector, target_vector);
        let recip_target_mag_sqr = length_squared(target_vector).recip();

        let projection_factor = c_dot_bb * recip_target_mag_sqr;

        let project_factor_vector = std::simd::f32x4::from_array([projection_factor; 4]);

        target_vector * project_factor_vector
    }

    #[inline]
    pub fn reflect(
        incident_vector: std::simd::f32x4,
        normal_vector: std::simd::f32x4,
    ) -> std::simd::f32x4 {
        let dot_prod = dot(incident_vector, normal_vector);
        let dot_prod_mul_2 = 2.0 * dot_prod;

        let intermediate_step: std::simd::f32x4 = std::simd::f32x4::from_array([dot_prod_mul_2; 4]);

        incident_vector - intermediate_step * normal_vector
    }

    #[inline]
    pub fn refract(
        incident_vector: std::simd::f32x4,
        normal_vector: std::simd::f32x4,
        eta: f32,
    ) -> std::simd::f32x4 {
        let ni = dot(normal_vector, incident_vector);

        let ni_sqr = ni * ni;
        let eta_sqr = eta * eta;
        let eta_mul_ni = eta * ni;

        let k = 1.0 - eta_sqr * (1.0 - ni_sqr);
        let intermediate = eta_mul_ni + k.sqrt();

        let eta_vector = std::simd::f32x4::from_array([eta; 4]);
        let intermediate_vector = std::simd::f32x4::from_array([intermediate; 4]);

        let mask = std::simd::mask32x4::from_array([k.is_sign_positive(); 4]);


        select(
            (eta_vector * incident_vector) - (intermediate_vector * normal_vector),
            std::simd::f32x4::from_array([0.0; 4]),
            mask,
        )
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
    pub fn cosh(vector_simd: std::simd::f32x4) -> std::simd::f32x4 {
        const E_VEC: std::simd::f32x4 = std::simd::f32x4::from_array([std::f32::consts::E; 4]);
        const HALF_VEC: std::simd::f32x4 = std::simd::f32x4::from_array([0.5; 4]);

        let rhs = pow(E_VEC, vector_simd);
        let lhs = pow(E_VEC, -vector_simd);

        (rhs + lhs) * HALF_VEC
    }

    #[inline]
    pub fn sinh(vector_simd: std::simd::f32x4) -> std::simd::f32x4 {
        const E_VEC: std::simd::f32x4 = std::simd::f32x4::from_array([std::f32::consts::E; 4]);
        const HALF_VEC: std::simd::f32x4 = std::simd::f32x4::from_array([0.5; 4]);

        let rhs = pow(E_VEC, vector_simd);
        let lhs = pow(E_VEC, -vector_simd);

        (rhs - lhs) * HALF_VEC
    }

    #[inline]
    pub fn tanh(vector_simd: std::simd::f32x4) -> std::simd::f32x4 {
        const E_VEC: std::simd::f32x4 = std::simd::f32x4::from_array([std::f32::consts::E; 4]);
        const ONE_VEC: std::simd::f32x4 = std::simd::f32x4::from_array([1.0; 4]);
        const TWO_VEC: std::simd::f32x4 = std::simd::f32x4::from_array([2.0; 4]);

        let two_vector_simd = vector_simd * TWO_VEC;

        let intermediate = pow(E_VEC, two_vector_simd);

        (intermediate - ONE_VEC) / (intermediate + ONE_VEC)
    }

    #[inline]
    pub fn bitmask(simd_vector: std::simd::f32x4) -> i32 {
        simd_vector.cast::<i32>().reduce_or()
    }

    #[inline]
    pub fn ldexp(mul_vector : std::simd::f32x4, power_vector : std::simd::f32x4) -> std::simd::f32x4{
        mul_vector * exp2(power_vector)
    }
}
