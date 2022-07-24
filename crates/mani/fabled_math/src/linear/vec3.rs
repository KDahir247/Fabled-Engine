use crate::math::{component_sum, mul_add};
use crate::matrix3x3_math::transpose;
use crate::{cross, Matrix3x3, Quaternion, Vector2, Vector4};
use std::fmt::{Display, Formatter};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use std::simd::Which::*;

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub value: std::simd::f32x4,
}

impl Default for Vector3 {
    fn default() -> Self {
        Self {
            value: [0.0; 4].into(),
        }
    }
}

impl Vector3 {
    pub const BACKWARD: Vector3 = Vector3 {
        value: std::simd::f32x4::from_array([0.0, 0.0, -1.0, 0.0]),
    };

    pub const FORWARD: Vector3 = Vector3 {
        value: std::simd::f32x4::from_array([0.0, 0.0, 1.0, 0.0]),
    };

    pub const UP: Vector3 = Vector3 {
        value: std::simd::f32x4::from_array([0.0, 1.0, 0.0, 0.0]),
    };

    pub const DOWN: Vector3 = Vector3 {
        value: std::simd::f32x4::from_array([0.0, -1.0, 0.0, 0.0]),
    };

    pub const RIGHT: Vector3 = Vector3 {
        value: std::simd::f32x4::from_array([1.0, 0.0, 0.0, 0.0]),
    };

    pub const LEFT: Vector3 = Vector3 {
        value: std::simd::f32x4::from_array([-1.0, 0.0, 0.0, 0.0]),
    };

    pub const ONE: Vector3 = Vector3 {
        value: std::simd::f32x4::from_array([1.0, 1.0, 1.0, 0.0]),
    };

    pub const ZERO: Vector3 = Vector3 {
        value: std::simd::f32x4::from_array([0.0, 0.0, 0.0, 0.0]),
    };

    pub fn trunc_vec2(self) -> Vector2 {
        const ZERO_VEC: Vector3 = Vector3::ZERO;

        let swizzle_vec: std::simd::f32x4 = std::simd::simd_swizzle!(
            self.value,
            ZERO_VEC.value,
            [First(0), First(1), Second(0), Second(1)]
        );

        Vector2 { value: swizzle_vec }
    }

    pub fn extend_vec4(self) -> Vector4 {
        Vector4 { value: self.value }
    }

    pub const fn set(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 {
            value: std::simd::f32x4::from_array([x, y, z, 0.0]),
        }
    }

    pub const fn to_primitive(self) -> [f32; 4] {
        self.value.to_array()
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let array_representation: [f32; 4] = *self.value.as_array();

        write!(
            f,
            "Vector3 (x : {}, y : {}, z : {})",
            array_representation[0], array_representation[1], array_representation[2]
        )
    }
}

impl Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value.as_array()[index]
    }
}

impl Mul<Quaternion> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        let mut result = self;
        result *= rhs;
        result
    }
}

impl MulAssign<Quaternion> for Vector3 {
    fn mul_assign(&mut self, rhs: Quaternion) {
        let splat_2 = Vector4::splat(2.0);
        let quaternion_scalar_splat = Vector4::splat(rhs.value[3]);
        let t = splat_2.value * cross(rhs.value, self.value);

        self.value += mul_add(quaternion_scalar_splat.value, t, cross(rhs.value, t));
    }
}


impl Mul<Vector3> for Matrix3x3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        let row_col_matrix: [f32; 16] = transpose(self).to_primitive();

        let res: Vec<f32> = row_col_matrix
            .chunks_exact(4)
            .map(|x| {
                let row_mul_col_vec = Vector3 {
                    value: std::simd::f32x4::from_slice(x),
                } * rhs;

                component_sum(row_mul_col_vec.value)
            })
            .collect::<Vec<f32>>();

        Vector3 {
            value: std::simd::f32x4::from_slice(res.as_slice()),
        }
    }
}

impl MulAssign<Matrix3x3> for Vector3 {
    fn mul_assign(&mut self, rhs: Matrix3x3) {
        self.value = (rhs * *self).value;
    }
}


// Component-Wise
impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            value: self.value * rhs.value,
        }
    }
}

impl MulAssign<Vector3> for Vector3 {
    fn mul_assign(&mut self, rhs: Vector3) {
        self.value *= rhs.value;
    }
}

impl Div<Vector3> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            value: self.value / rhs.value,
        }
    }
}

impl DivAssign<Vector3> for Vector3 {
    fn div_assign(&mut self, rhs: Vector3) {
        self.value /= rhs.value;
    }
}

impl Rem<Vector3> for Vector3 {
    type Output = Vector3;

    fn rem(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            value: self.value % rhs.value,
        }
    }
}

impl RemAssign<Vector3> for Vector3 {
    fn rem_assign(&mut self, rhs: Vector3) {
        self.value %= rhs.value;
    }
}

impl Add<f32> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector3 {
            value: self.value + splat_f32x4,
        }
    }
}

impl AddAssign<f32> for Vector3 {
    fn add_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value += splat_f32x4;
    }
}

impl Sub<f32> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector3 {
            value: self.value - splat_f32x4,
        }
    }
}

impl SubAssign<f32> for Vector3 {
    fn sub_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value -= splat_f32x4;
    }
}
//

// Vector-Wise

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector3 {
            value: self.value * splat_f32x4,
        }
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value *= splat_f32x4;
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector3 {
            value: self.value / splat_f32x4,
        }
    }
}

impl DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value /= splat_f32x4;
    }
}

impl Rem<f32> for Vector3 {
    type Output = Vector3;

    fn rem(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector3 {
            value: self.value % splat_f32x4,
        }
    }
}

impl RemAssign<f32> for Vector3 {
    fn rem_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value %= splat_f32x4;
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3 { value: -self.value }
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.value += rhs.value;
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            value: self.value - rhs.value,
        }
    }
}

impl SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        self.value -= rhs.value;
    }
}

// AOSOA (Array of Struct of Array) form
pub struct F32X8Vector3 {
    pub x: std::simd::f32x8,
    pub y: std::simd::f32x8,
    pub z: std::simd::f32x8,
}

pub struct TiledVector3<const N: usize> {
    pub value: [F32X8Vector3; N],
}


#[cfg(test)]
mod vector3_test {
    use crate::Vector3;

    #[test]
    fn create_vector3() {}
}
