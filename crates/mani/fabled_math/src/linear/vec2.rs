use crate::{Vector3, Vector4};
use std::fmt::{Display, Formatter};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

#[derive(Copy, Clone)]
pub struct Vector2 {
    pub value: std::simd::f32x4,
}

impl Default for Vector2 {
    fn default() -> Self {
        Self {
            value: [0.0; 4].into(),
        }
    }
}

impl Vector2 {
    pub const UP: Vector2 = Vector2 {
        value: std::simd::f32x4::from_array([0.0, 1.0, 0.0, 0.0]),
    };

    pub const DOWN: Vector2 = Vector2 {
        value: std::simd::f32x4::from_array([0.0, -1.0, 0.0, 0.0]),
    };

    pub const RIGHT: Vector2 = Vector2 {
        value: std::simd::f32x4::from_array([1.0, 0.0, 0.0, 0.0]),
    };

    pub const ONE: Vector2 = Vector2 {
        value: std::simd::f32x4::from_array([1.0, 1.0, 0.0, 0.0]),
    };

    pub const ZERO: Vector2 = Vector2 {
        value: std::simd::f32x4::from_array([0.0, 0.0, 0.0, 0.0]),
    };

    pub const NEG_ZERO: Vector2 = Vector2 {
        value: std::simd::f32x4::from_array([-1.0, -1.0, 0.0, 0.0]),
    };

    pub fn extend_vec3(self) -> Vector3 {
        Vector3 { value: self.value }
    }

    pub fn extend_vec4(self) -> Vector4 {
        Vector4 { value: self.value }
    }

    pub const fn set(x: f32, y: f32) -> Vector2 {
        Vector2 {
            value: std::simd::f32x4::from_array([x, y, 0.0, 0.0]),
        }
    }
}

impl Display for Vector2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let array_representation: [f32; 4] = *self.value.as_array();

        write!(
            f,
            "Vector2 (x : {}, y : {})",
            array_representation[0], array_representation[1]
        )
    }
}

// Component-Wise
impl Mul<Vector2> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            value: self.value * rhs.value,
        }
    }
}

impl MulAssign<Vector2> for Vector2 {
    fn mul_assign(&mut self, rhs: Vector2) {
        self.value *= rhs.value;
    }
}

impl Div<Vector2> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            value: self.value / rhs.value,
        }
    }
}

impl DivAssign<Vector2> for Vector2 {
    fn div_assign(&mut self, rhs: Vector2) {
        self.value /= rhs.value;
    }
}

impl Rem<Vector2> for Vector2 {
    type Output = Vector2;

    fn rem(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            value: self.value % rhs.value,
        }
    }
}

impl RemAssign<Vector2> for Vector2 {
    fn rem_assign(&mut self, rhs: Vector2) {
        self.value %= rhs.value;
    }
}

impl Add<f32> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector2 {
            value: self.value + splat_f32x4,
        }
    }
}

impl AddAssign<f32> for Vector2 {
    fn add_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value += splat_f32x4;
    }
}

impl Sub<f32> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector2 {
            value: self.value - splat_f32x4,
        }
    }
}

impl SubAssign<f32> for Vector2 {
    fn sub_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value -= splat_f32x4;
    }
}
//

// Vector-Wise

impl Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector2 {
            value: self.value * splat_f32x4,
        }
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value *= splat_f32x4;
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector2 {
            value: self.value / splat_f32x4,
        }
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value /= splat_f32x4;
    }
}

impl Rem<f32> for Vector2 {
    type Output = Vector2;

    fn rem(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector2 {
            value: self.value % splat_f32x4,
        }
    }
}

impl RemAssign<f32> for Vector2 {
    fn rem_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value %= splat_f32x4;
    }
}


impl Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign<Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        self.value += rhs.value;
    }
}

impl Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            value: self.value - rhs.value,
        }
    }
}

impl SubAssign<Vector2> for Vector2 {
    fn sub_assign(&mut self, rhs: Vector2) {
        self.value -= rhs.value;
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Self::Output {
        Vector2 { value: -self.value }
    }
}

// AOSOA (Array of Struct of Array) form
pub struct F32X8Vector2 {
    pub x: std::simd::f32x8,
    pub y: std::simd::f32x8,
}

pub struct TiledVector2<const N: usize> {
    pub value: [F32X8Vector2; N],
}

#[cfg(test)]
mod vector2_test {
    use crate::Vector2;

    #[test]
    fn create_vector2() {}
}
