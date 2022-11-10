use crate::{Quaternion, Vector3, DualNumber};

use std::ops::{Add, AddAssign, Mul, MulAssign, Div, DivAssign, Sub, SubAssign};
use std::fmt::Display;

#[derive(Copy, Clone, PartialEq)]
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
        write!(f, "Dual Quaternion(\nReal {} \nDual {}\n)", self.real, self.dual)
    }
}
// Component-Wise
impl Add<f32> for DualQuaternion{
    type Output = DualQuaternion;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        DualQuaternion{
            real:  self.real + rhs,
            dual: self.dual + rhs,
        }
    }
}

impl AddAssign<f32> for DualQuaternion{
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.real += rhs;
        self.dual += rhs;
    }
}

impl Sub<f32> for DualQuaternion{
    type Output = DualQuaternion;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        DualQuaternion{
            real: self.real - rhs,
            dual: self.dual - rhs,
        }
    }
}

impl SubAssign<f32> for DualQuaternion{
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.real -= rhs;
        self.dual -= rhs;
    }
}

impl Mul<f32> for DualQuaternion{
    type Output = DualQuaternion;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        DualQuaternion{
            real: self.real * rhs,
            dual: self.dual * rhs,
        }
    }
}

impl MulAssign<f32> for DualQuaternion{
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.real *= rhs;
        self.dual *= rhs;
    }
}

impl Div<f32> for DualQuaternion{
    type Output = DualQuaternion;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        DualQuaternion{
            real: self.real / rhs,
            dual: self.dual / rhs,
        }
    }
}

impl DivAssign<f32> for DualQuaternion{
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.real /= rhs;
        self.dual /= rhs;
    }
}

// DualQuaternion-Wise
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

impl Div<DualNumber> for DualQuaternion{
    type Output = DualQuaternion;

    #[inline]
    fn div(self, rhs: DualNumber) -> Self::Output {
        let s = rhs.value.x() * rhs.value.x();
        DualQuaternion{
            real:  self.real * rhs.value.x(),
            dual: (self.dual * rhs.value.x()) - (self.real * rhs.value.y()),
        } / s
    }
}

impl DivAssign<DualNumber> for DualQuaternion{
    #[inline]
    fn div_assign(&mut self, rhs: DualNumber) {
        let s = (rhs.value.x() * rhs.value.x()).recip();
        self.real = self.real * rhs.value.x();
        self.dual = (self.dual * rhs.value.x()) - (self.real * rhs.value.y());

        self.real *= s;
        self.dual *= s;
    }
}

pub mod dual_quaternion_math{

    use crate::{DualQuaternion, Vector3, Quaternion, Vector4, DualNumber};

    use crate::quaternion_math::{conjugate_quat, inverse_quat};

    use crate::vector_math::{dot, length, length_squared};

    #[inline]
    pub fn conjugate_dual_quat(dual_quaternion : DualQuaternion) -> DualQuaternion{
        DualQuaternion { real: conjugate_quat(dual_quaternion.real), dual: conjugate_quat(dual_quaternion.dual) }
    }

    #[inline]
    pub fn dual_number_conjugate_dual_quat(dual_quaternion : DualQuaternion) -> DualQuaternion{
        DualQuaternion{ real: dual_quaternion.real, dual: -dual_quaternion.dual }

    }

    #[inline]
    pub fn combined_conjugate_dual_quat(dual_quaternion : DualQuaternion) -> DualQuaternion{
        DualQuaternion{ real: conjugate_quat(dual_quaternion.real), dual: -conjugate_quat(dual_quaternion.dual) }
    }

    #[inline]
    pub fn from_translation_rotation_dual_quat(translation : Vector3, rotation : Quaternion) -> DualQuaternion{
        let dual_quaternion = Quaternion::set(translation.x(), translation.y(), translation.z(), 0.0);

        DualQuaternion{
            real: rotation,
            dual:  (dual_quaternion * rotation) * 0.5,
        }
    }

    #[inline]
    pub fn from_translation_dual_quat(translation : Vector3) -> DualQuaternion{
        let dual_quaternion = Quaternion::set(translation.x(), translation.y(), translation.z(), 0.0);

        DualQuaternion {
            real: Quaternion::IDENTITY,
            dual: dual_quaternion.scale_quaternion(Vector4::broadcast(0.5))
        }
    }

    #[inline]
    pub fn from_rotation_dual_quat(normalized_axis : Vector3, angle_radian : f32) -> DualQuaternion{
        let extended_axis = Vector4::set(normalized_axis.x(), normalized_axis.y(), normalized_axis.z(), 1.0);

        let theta = angle_radian * 0.5f32;

        let (sin_theta, cos_theta) = theta.sin_cos();

        let theta_vector = Vector4::set(sin_theta, sin_theta, sin_theta, cos_theta);

        DualQuaternion { real: Quaternion { value: (extended_axis * theta_vector).value }, dual: Quaternion::ZERO }
    }

    #[inline]
    pub fn get_translation_dual_quat(dual_quaternion : DualQuaternion) -> Vector3{

        const TWO_VEC : Vector4 = Vector4::broadcast(2.0);

        let translation_vec = dual_quaternion.dual.scale_quaternion(TWO_VEC) * conjugate_quat(dual_quaternion.real);

        Vector3::set(translation_vec.i(), translation_vec.j(), translation_vec.k())
    }

    #[inline]
    pub fn get_rotation_dual_quat(dual_quaternion : DualQuaternion) -> Quaternion{
        dual_quaternion.real
    }

    #[inline]
    pub fn normalize_dual_quat(dual_quaternion : DualQuaternion) -> DualQuaternion{

        let real_length_recip = length(dual_quaternion.real.value).recip();

        DualQuaternion {
            real: dual_quaternion.real * real_length_recip,
            dual: dual_quaternion.dual * real_length_recip
        }
    }

    #[inline]
    pub fn length_dual_quat(dual_quaternion : DualQuaternion) -> DualNumber{
        let real = length(dual_quaternion.real.value);
        let real_pure = dual_quaternion.real.to_pure();
        let dual_pure = dual_quaternion.dual.to_pure();

        DualNumber::set(real, ((dual_quaternion.real.to_real() * dual_quaternion.dual.to_real()) + dot(real_pure.value, dual_pure.value)) / real)
    }

    #[inline]
    pub fn length_sqr_dual_quat(dual_quaternion : DualQuaternion) -> DualNumber{
        let real_pure = dual_quaternion.real.to_pure();
        let dual_pure = dual_quaternion.dual.to_pure();
        DualNumber::set(length_squared(dual_quaternion.real.value), 2.0 * (dual_quaternion.real.to_real() *dual_quaternion.dual.to_real() + dot(real_pure.value, dual_pure.value)))
    }

    #[inline]
    pub fn inverse_dual_quat(dual_quaternion : DualQuaternion) -> DualQuaternion{

        let inverse_real = inverse_quat(dual_quaternion.real);

        DualQuaternion{
            real: inverse_real,
            dual: -inverse_real * dual_quaternion.dual * inverse_real,
        }
    }

    #[inline]
    pub fn transform_point3_dual_quat(dual_quaternion : DualQuaternion, point3 : Vector3) -> DualQuaternion{
        let point_dual_quat = DualQuaternion{ real: Quaternion::IDENTITY, dual: Quaternion::set(point3.x(), point3.y(), point3.z(), 0.0) };

        dual_quaternion * point_dual_quat * combined_conjugate_dual_quat(dual_quaternion)
    }

    //https://www.cs.utah.edu/~ladislav/kavan06dual/kavan06dual.pdf
    #[inline]
    pub fn linear_blending_dual_quat(start : DualQuaternion, end : DualQuaternion, t : f32) -> DualQuaternion{
        let time = 1.0 - t;

        let combined =  start * time + end * t;
        let dual_length = length_dual_quat(combined);
        combined / dual_length
    }

    // Not 100 percent sure implementation below is valid nor invalid. Use with caution :)

    #[inline]
    pub fn exp_dual_quaternion(dual_quaternion : DualQuaternion) -> DualQuaternion{
        let real = crate::quaternion_math::exp_quat(dual_quaternion.real);
        let dual = dual_quaternion.dual * real;

        DualQuaternion {
            real,
            dual
        }
    }

    #[inline]
    pub fn pow_dual_quaternion(dual_quaternion : DualQuaternion, exponent : f32) -> DualQuaternion{

        let theta = 2.0 * dual_quaternion.real.w().acos();

        let mut dual_quat = DualQuaternion::IDENTITY;

        let half_theta = theta * 0.5;

        let rcp_half_theta_sin = half_theta.sin().recip();
        let (exp_half_theta_sin, exp_half_theta_cos) = (exponent * half_theta).sin_cos();

        if std::intrinsics::unlikely(theta < 0.0) {
            dual_quat = from_translation_dual_quat(get_translation_dual_quat(dual_quaternion) * exponent);
        }

        let s0 = dual_quaternion.real.to_pure() * rcp_half_theta_sin;
        let d = -2.0 * dual_quaternion.dual.w() * rcp_half_theta_sin;
        let half_d = d * 0.5;

        let se = (dual_quaternion.dual.to_pure() - s0 * half_d * (half_theta).cos()) * rcp_half_theta_sin;

        let real = Quaternion::from_additive_form(exp_half_theta_cos, s0 * exp_half_theta_sin);

        let dual = Quaternion::from_additive_form(-exponent * half_d * exp_half_theta_sin, s0 * exponent * half_d  * exp_half_theta_cos + se * exp_half_theta_sin);

        dual_quat = DualQuaternion::new(real, dual);

        dual_quat
    }


    #[inline]
    pub fn screw_linear_blending_dual_quat(start : DualQuaternion, end : DualQuaternion, time : f32) -> DualQuaternion{
        crate::dual_quaternion_math::pow_dual_quaternion(start * (crate::dual_quaternion_math::inverse_dual_quat(start) * end), time)
    }


    // Might Implement or not will decide tomorrow.
    #[inline]
    pub fn from_screw() -> DualQuaternion{
        todo!()
    }

    #[inline]
    pub fn to_screw() {
        todo!()
    }

}