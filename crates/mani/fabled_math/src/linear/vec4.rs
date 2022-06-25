use crate::Vector3;
use std::fmt::{Display, Formatter};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use std::simd::Which::*;

#[derive(Copy, Clone)]
pub struct Vector4 {
    pub value: std::simd::f32x4,
}

impl Default for Vector4 {
    fn default() -> Self {
        Self {
            value: [0.0; 4].into(),
        }
    }
}

impl Vector4 {
    pub const ZERO: Vector4 = Vector4 {
        value: std::simd::f32x4::from_array([0.0, 0.0, 0.0, 0.0]),
    };

    pub const ONE: Vector4 = Vector4 {
        value: std::simd::f32x4::from_array([1.0, 1.0, 1.0, 1.0]),
    };

    pub fn trunc_vec3(self) -> Vector3 {
        const ZERO_VEC: Vector4 = Vector4::ZERO;

        let swizzle_vec: std::simd::f32x4 = std::simd::simd_swizzle!(
            self.value,
            ZERO_VEC.value,
            [First(0), First(1), First(2), Second(0)]
        );

        Vector3 { value: swizzle_vec }
    }

    pub const fn set(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Vector4 {
            value: std::simd::f32x4::from_array([x, y, z, w]),
        }
    }

    pub const fn splat(val: f32) -> Vector4 {
        Vector4 {
            value: std::simd::f32x4::splat(val),
        }
    }
}

impl Display for Vector4 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let array_representation: [f32; 4] = *self.value.as_array();

        write!(
            f,
            "Vector4 (x : {}, y : {}, z : {}, w : {})",
            array_representation[0],
            array_representation[1],
            array_representation[2],
            array_representation[3]
        )
    }
}
// todo add assign math ops

// Component-Wise
impl Mul<Vector4> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Self::Output {
        Vector4 {
            value: self.value * rhs.value,
        }
    }
}

impl MulAssign<Vector4> for Vector4 {
    fn mul_assign(&mut self, rhs: Vector4) {
        self.value *= rhs.value;
    }
}

impl Div<Vector4> for Vector4 {
    type Output = Vector4;

    fn div(self, rhs: Vector4) -> Self::Output {
        Vector4 {
            value: self.value / rhs.value,
        }
    }
}

impl DivAssign<Vector4> for Vector4 {
    fn div_assign(&mut self, rhs: Vector4) {
        self.value /= rhs.value;
    }
}

impl Rem<Vector4> for Vector4 {
    type Output = Vector4;

    fn rem(self, rhs: Vector4) -> Self::Output {
        Vector4 {
            value: self.value % rhs.value,
        }
    }
}

impl RemAssign<Vector4> for Vector4 {
    fn rem_assign(&mut self, rhs: Vector4) {
        self.value %= rhs.value;
    }
}

impl Add<f32> for Vector4 {
    type Output = Vector4;

    fn add(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector4 {
            value: self.value + splat_f32x4,
        }
    }
}

impl AddAssign<f32> for Vector4 {
    fn add_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value += splat_f32x4;
    }
}

impl Sub<f32> for Vector4 {
    type Output = Vector4;

    fn sub(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector4 {
            value: self.value - splat_f32x4,
        }
    }
}


impl SubAssign<f32> for Vector4 {
    fn sub_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value -= splat_f32x4;
    }
}
//

// Vector-Wise

impl Mul<f32> for Vector4 {
    type Output = Vector4;

    fn mul(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector4 {
            value: self.value * splat_f32x4,
        }
    }
}

impl MulAssign<f32> for Vector4 {
    fn mul_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value *= splat_f32x4;
    }
}

impl Div<f32> for Vector4 {
    type Output = Vector4;

    fn div(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector4 {
            value: self.value / splat_f32x4,
        }
    }
}

impl DivAssign<f32> for Vector4 {
    fn div_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value /= splat_f32x4;
    }
}

impl Rem<f32> for Vector4 {
    type Output = Vector4;

    fn rem(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector4 {
            value: self.value % splat_f32x4,
        }
    }
}

impl RemAssign<f32> for Vector4 {
    fn rem_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value %= splat_f32x4;
    }
}

impl Neg for Vector4 {
    type Output = Vector4;

    fn neg(self) -> Self::Output {
        Vector4 { value: -self.value }
    }
}

impl Add<Vector4> for Vector4 {
    type Output = Vector4;

    fn add(self, rhs: Vector4) -> Self::Output {
        Vector4 {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign<Vector4> for Vector4 {
    fn add_assign(&mut self, rhs: Vector4) {
        self.value += rhs.value;
    }
}

impl Sub<Vector4> for Vector4 {
    type Output = Vector4;

    fn sub(self, rhs: Vector4) -> Self::Output {
        Vector4 {
            value: self.value - rhs.value,
        }
    }
}

impl SubAssign<Vector4> for Vector4 {
    fn sub_assign(&mut self, rhs: Vector4) {
        self.value -= rhs.value;
    }
}
// AOSOA (Array of Struct of Array) form
pub struct F32x8Vector4 {
    pub x: std::simd::f32x8,
    pub y: std::simd::f32x8,
    pub z: std::simd::f32x8,
    pub w: std::simd::f32x8,
}

pub struct SOAVector4<const N: usize> {
    pub value: [F32x8Vector4; N],
}


#[cfg(test)]
mod vector4_test {
    use crate::Vector4;

    #[test]
    fn create_vector4() {
        let one = Vector4::ONE;
        println!("{}", one);

        let custom_vec4 = Vector4::set(4.0, 1.0, 3.0, 10.0);
        println!("{}", custom_vec4)
    }


    #[test]
    fn arithmetic_operation() {}
}
