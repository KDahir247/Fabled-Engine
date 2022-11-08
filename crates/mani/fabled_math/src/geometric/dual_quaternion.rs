use crate::{Quaternion, Vector3};

use std::ops::{Add, AddAssign, Mul, MulAssign};
use std::fmt::Display;

#[derive(Copy, Clone)]
pub struct DualQuaternion{
    pub real : Quaternion,
    pub dual : Quaternion
}

impl Default for DualQuaternion{
    fn default() -> Self {
        DualQuaternion { real: Quaternion::IDENTITY, dual: Quaternion::ZERO }
    }
}

impl DualQuaternion{
    pub const IDENTITY : DualQuaternion = DualQuaternion{ real: Quaternion::IDENTITY, dual: Quaternion::IDENTITY };
}

impl DualQuaternion{
    #[inline(always)]
    pub const fn set(real_img : Vector3, real_scalar : f32, dual_img : Vector3, dual_scalar : f32) -> DualQuaternion{
        let real_quaternion = Quaternion::set(real_img.x(), real_img.y(), real_img.z(), real_scalar);
        let dual_quaternion = Quaternion::set(dual_img.x(), dual_img.y(), dual_img.z(), dual_scalar);

        DualQuaternion { real: real_quaternion, dual: dual_quaternion }
    }

    #[inline(always)]
    pub const fn new(real : Quaternion, dual : Quaternion) -> DualQuaternion{
        DualQuaternion { real, dual }
    }

    #[inline(always)]
    pub const fn broadcast(real_val : f32, dual_val : f32) -> DualQuaternion{
        DualQuaternion { real: Quaternion::broadcast(real_val), dual: Quaternion::broadcast(dual_val) }
    }

    #[inline]
    pub const fn to_primitive(self) -> [f32; 8]{
        let real_primitve = self.real.to_primitive();
        let dual_primitive = self.dual.to_primitive();

        [
            real_primitve[0], real_primitve[1], real_primitve[2], real_primitve[3],
            dual_primitive[0], dual_primitive[1], dual_primitive[2], dual_primitive[3],
        ]
    }

    #[inline]
    pub const fn from_primitive(val : [f32; 8]) -> DualQuaternion{
        let real_primitive = [val[0], val[1], val[2], val[3]];
        let dual_primitive = [val[4], val[5], val[6], val[7]];

        DualQuaternion { real: Quaternion::from_primitive(real_primitive), dual: Quaternion::from_primitive(dual_primitive) }
    }
}

impl Display for DualQuaternion{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Real {} \nDual {}", self.real, self.dual)
    }
}

//σ = p + ǫq
//σ1 + σ2 = (p1 + p2) + ǫ(q1 + q2)
impl Add for DualQuaternion{
    type Output = DualQuaternion;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        DualQuaternion{
            real: self.real + rhs.real,
            dual: self.dual + self.dual,
        }
    }
}

impl AddAssign for DualQuaternion{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.real += rhs.real;
        self.dual += rhs.dual;
    }
}

// σ1 ⊗ σ2 = p1p2 + ǫ(p1 * q2 + q1 * p2).
impl Mul for DualQuaternion{
    type Output = DualQuaternion;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        DualQuaternion{
            real: self.real * rhs.real,
            dual: (self.real * rhs.dual) + (self.dual * rhs.real),
        }
    }
}

impl MulAssign for DualQuaternion{
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.real *= rhs.real;
        self.dual = (self.real * rhs.dual) + (self.dual * rhs.real);
    }
}

pub mod dual_quaternion_math{

    use crate::{DualQuaternion, Vector3, Quaternion, Vector4};

    use crate::quaternion_math::conjugate_quat;

    use crate::vector_math::{dot, length};

    //Q = p* + q*
    #[inline]
    pub fn conjugate_dual_quat(dual_quaternion : DualQuaternion) -> DualQuaternion{
        DualQuaternion { real: conjugate_quat(dual_quaternion.real), dual: conjugate_quat(dual_quaternion.dual) }
    }

    //Q = p - q
    #[inline]
    pub fn dual_number_conjugate_dual_quat(dual_quaternion : DualQuaternion) -> DualQuaternion{
        DualQuaternion{ real: dual_quaternion.real, dual: -dual_quaternion.dual }

    }

    //Q = p* - q*
    #[inline]
    pub fn combined_conjugate_dual_quat(dual_quaternion : DualQuaternion) -> DualQuaternion{
        DualQuaternion{ real: conjugate_quat(dual_quaternion.real), dual: -conjugate_quat(dual_quaternion.dual) }
    }

    //Qr = r
    //Qd = 0.5 . quat(t, 0) . r
    #[inline]
    pub fn from_translation_rotation_dual_quat(translation : Vector3, rotation : Quaternion) -> DualQuaternion{
        let dual_quaternion = Quaternion::set(translation.x(), translation.y(), translation.z(), 0.0);

        DualQuaternion{
            real: rotation,
            dual:  (dual_quaternion * rotation) * 0.5,
        }
    }

    // Q = [0,0,0,1] [tx/2, ty/2, tz/2, 0]
    #[inline]
    pub fn from_translation_dual_quat(translation : Vector3) -> DualQuaternion{
        let dual_quaternion = Quaternion::set(translation.x(), translation.y(), translation.z(), 0.0);

        DualQuaternion {
            real: Quaternion::IDENTITY,
            dual: dual_quaternion.scale_quaternion(Vector4::broadcast(0.5))
        }
    }

    // Q = [nx * sin(theta/2), ny * sin(theta/2), nz * sin(theta/2), cos(theta/2)] [0,0,0,0]
    #[inline]
    pub fn from_rotation_dual_quat(normalized_axis : Vector3, angle_radian : f32) -> DualQuaternion{
        let extended_axis = Vector4::set(normalized_axis.x(), normalized_axis.y(), normalized_axis.z(), 1.0);

        let theta = angle_radian * 0.5f32;

        let (sin_theta, cos_theta) = theta.sin_cos();

        let theta_vector = Vector4::set(sin_theta, sin_theta, sin_theta, cos_theta);

        DualQuaternion { real: Quaternion { value: (extended_axis * theta_vector).value }, dual: Quaternion::ZERO }
    }

//    //d = Qr . Q1r
//    #[inline]
//    pub fn dot_dual_quat(dual_quaternion0 : DualQuaternion, dual_quaternion1 : DualQuaternion) -> f32{
//        dot(dual_quaternion0.real.value, dual_quaternion1.real.value)
//    }

    //t = 2 * Qd * conjugate(Qr)
    #[inline]
    pub fn get_translation_dual_quat(dual_quaternion : DualQuaternion) -> Vector3{

        const TWO_VEC : Vector4 = Vector4::broadcast(2.0);

        let translation_vec = dual_quaternion.dual.scale_quaternion(TWO_VEC) * conjugate_quat(dual_quaternion.real);

        Vector3::set(translation_vec.i(), translation_vec.j(), translation_vec.k())
    }

    // R = Qr
    #[inline]
    pub fn get_rotation_dual_quat(dual_quaternion : DualQuaternion) -> Quaternion{
        dual_quaternion.real
    }

    // |q| = q.r / ||q.real|| + q.d / ||q.real||
    #[inline]
    pub fn normalize_dual_quat(dual_quaternion : DualQuaternion) -> DualQuaternion{

        let real_length_recip = length(dual_quaternion.real.value).recip();

        DualQuaternion {
            real: dual_quaternion.real * real_length_recip,
            dual: dual_quaternion.dual * real_length_recip
        }

    }

//    // |q|^2 = qq*
//    #[inline]
//    pub fn norm_dual_quat(dual_quaternion : DualQuaternion) -> DualQuaternion{
//        dual_quaternion * conjugate_dual_quat(dual_quaternion)
//    }
//
//    //||q||^2 = qq*.r scalar
//    #[inline]
//    pub fn magnitude_sqr_dual_quat(dual_quaternion : DualQuaternion) -> f32{
//        norm_dual_quat(dual_quaternion).real.w()
//    }
//
//
//    // ||q|| = sqrt(qq*.r scalar)
//    #[inline]
//    pub fn magnitude_dual_quat(dual_quaternion : DualQuaternion) -> f32{
//        norm_dual_quat(dual_quaternion).real.w().sqrt()
//    }

    // q-1 = q* / |q|^2
    #[inline]
    pub fn inverse_dual_quat(dual_quaternion : DualQuaternion) -> DualQuaternion{
        todo!()
    }

    // q` = qpq*
    #[inline]
    pub fn transform_point3_dual_quat(dual_quaternion : DualQuaternion, point3 : Vector3) -> DualQuaternion{
        let point_dual_quat = DualQuaternion{ real: Quaternion::IDENTITY, dual: Quaternion::set(point3.x(), point3.y(), point3.z(), 0.0) };

        dual_quaternion * point_dual_quat * combined_conjugate_dual_quat(dual_quaternion)
    }

    #[inline]
    pub fn linear_blending_dual_quat(start : DualQuaternion, end : DualQuaternion, t : f32) -> DualQuaternion{
        todo!()
    }

    //TODO
    // Normalize, magnitude, Get Matrix (translation, rotation, NO SCALE), maybe composition.
    // This is not set in stone and more function may be added if needed (ex rotation velocity and transitional velocity).
}