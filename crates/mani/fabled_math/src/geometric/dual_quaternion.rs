use crate::{Quaternion, Vector3};

use std::{ops::{Add, AddAssign, Mul, MulAssign}, fmt::Display};

#[derive(Copy, Clone)]
pub struct DualQuaternion{
    pub real : Quaternion,
    pub dual : Quaternion
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

// σ1 ⊗ σ2 = p1p2 + ǫ(p1 . q2 + q1 . p2).
impl Mul for DualQuaternion{
    type Output = DualQuaternion;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        DualQuaternion{
            real: self.real + rhs.real,
            dual: (self.real * rhs.dual) + (self.dual * rhs.real),
        }
    }
}

impl MulAssign for DualQuaternion{
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.real += rhs.real;
        self.dual = (self.real * rhs.dual) + (self.dual * rhs.real);
    }
}

pub mod dual_quaternion_math{

    use crate::DualQuaternion;

    use crate::quaternion_math::conjugate_quat;

    pub fn conjugate(dual_quaternion : DualQuaternion) -> DualQuaternion{
        DualQuaternion { real: dual_quaternion.real, dual: conjugate_quat(dual_quaternion.dual) }
    }


}