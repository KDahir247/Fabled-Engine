use crate::{Vector3, Vector4};

use crate::math_trait::Swizzles2;

use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub struct Vector2 {
    pub value: [f32; 2],
}

impl Default for Vector2 {
    fn default() -> Self {
        Self { value: [0.0; 2] }
    }
}

impl Vector2 {
    pub const UP: Vector2 = Vector2 { value: [0.0, 1.0] };

    pub const DOWN: Vector2 = Vector2 { value: [0.0, -1.0] };

    pub const RIGHT: Vector2 = Vector2 { value: [1.0, 0.0] };

    pub const ONE: Vector2 = Vector2 { value: [1.0, 1.0] };

    pub const ZERO: Vector2 = Vector2 { value: [0.0, 0.0] };

    pub const NEG_ONE: Vector2 = Vector2 {
        value: [-1.0, -1.0],
    };

    #[inline(always)]
    pub const fn set(x: f32, y: f32) -> Vector2 {
        Vector2 { value: [x, y] }
    }

    #[inline(always)]
    pub const fn broadcast(val: f32) -> Vector2 {
        Vector2 { value: [val; 2] }
    }

    #[inline(always)]
    pub const fn x(self) -> f32 {
        self.value[0]
    }

    #[inline(always)]
    pub const fn y(self) -> f32 {
        self.value[1]
    }

    #[inline]
    pub const fn to_primitive(self) -> [f32; 2] {
        self.value
    }

    #[inline]
    pub const fn from_primitive(array: [f32; 2]) -> Vector2 {
        Vector2 { value: array }
    }


    #[inline]
    pub const fn to_simd(self) -> std::simd::f32x4 {
        std::simd::f32x4::from_array([self.x(), self.y(), 0.0, 0.0])
    }
}

impl Display for Vector2 {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(_f, "Vector2 (x : {}, y : {})", self.x(), self.y())
    }
}

// Component-Wise
impl Mul<Vector2> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn mul(self, rhs: Vector2) -> Self::Output {
        let res_y = self.value[1] * rhs.value[1];
        let res_x = self.value[0] * self.value[0];

        Vector2 {
            value: [res_x, res_y],
        }
    }
}

impl MulAssign<Vector2> for Vector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Vector2) {
        self.value[1] *= rhs.value[1];
        self.value[0] *= rhs.value[0];
    }
}

impl Div<Vector2> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn div(self, rhs: Vector2) -> Self::Output {
        let res_y = self.value[1] / rhs.value[1];
        let res_x = self.value[0] / rhs.value[0];

        Vector2 {
            value: [res_x, res_y],
        }
    }
}

impl DivAssign<Vector2> for Vector2 {
    #[inline]
    fn div_assign(&mut self, rhs: Vector2) {
        self.value[1] /= rhs.value[1];
        self.value[0] /= rhs.value[0];
    }
}

impl Rem<Vector2> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn rem(self, rhs: Vector2) -> Self::Output {
        let res_y = self.value[1] % rhs.value[1];
        let res_x = self.value[0] % rhs.value[0];

        Vector2 {
            value: [res_x, res_y],
        }
    }
}

impl RemAssign<Vector2> for Vector2 {
    #[inline]
    fn rem_assign(&mut self, rhs: Vector2) {
        self.value[1] %= rhs.value[1];
        self.value[0] %= rhs.value[0];
    }
}

impl Add<f32> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        let y = self.value[1] + rhs;
        let x = self.value[0] + rhs;

        Vector2 { value: [x, y] }
    }
}

impl AddAssign<f32> for Vector2 {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.value[1] += rhs;
        self.value[0] += rhs;
    }
}

impl Sub<f32> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        let y = self.value[1] - rhs;
        let x = self.value[0] - rhs;

        Vector2 { value: [x, y] }
    }
}

impl SubAssign<f32> for Vector2 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.value[1] -= rhs;
        self.value[0] -= rhs;
    }
}

// Vector-Wise
impl Mul<f32> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        let y = self.value[1] * rhs;
        let x = self.value[0] * rhs;

        Vector2 { value: [x, y] }
    }
}

impl MulAssign<f32> for Vector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.value[1] *= rhs;
        self.value[0] *= rhs;
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        let y = self.value[1] / rhs;
        let x = self.value[0] / rhs;

        Vector2 { value: [x, y] }
    }
}

impl DivAssign<f32> for Vector2 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.value[1] /= rhs;
        self.value[0] /= rhs;
    }
}

impl Rem<f32> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn rem(self, rhs: f32) -> Self::Output {
        let y = self.value[1] % rhs;
        let x = self.value[0] % rhs;

        Vector2 { value: [x, y] }
    }
}

impl RemAssign<f32> for Vector2 {
    #[inline]
    fn rem_assign(&mut self, rhs: f32) {
        self.value[1] %= rhs;
        self.value[0] %= rhs;
    }
}


impl Add<Vector2> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn add(self, rhs: Vector2) -> Self::Output {
        let y = self.value[1] + rhs.value[1];
        let x = self.value[0] + rhs.value[0];

        Vector2 { value: [x, y] }
    }
}

impl AddAssign<Vector2> for Vector2 {
    #[inline]
    fn add_assign(&mut self, rhs: Vector2) {
        self.value[1] += rhs.value[1];
        self.value[0] += rhs.value[0];
    }
}

impl Sub<Vector2> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn sub(self, rhs: Vector2) -> Self::Output {
        let y = self.value[1] + rhs.value[1];
        let x = self.value[0] + rhs.value[0];

        Vector2 { value: [x, y] }
    }
}

impl SubAssign<Vector2> for Vector2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vector2) {
        self.value[1] -= rhs.value[1];
        self.value[0] -= rhs.value[0];
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    #[inline]
    fn neg(self) -> Self::Output {
        let y = -self.value[1];
        let x = -self.value[0];

        Vector2 { value: [x, y] }
    }
}

impl Swizzles2 for Vector2 {
    type Swizzle3 = Vector3;
    type Swizzle4 = Vector4;

    #[inline]
    fn xx(self) -> Self {
        Vector2::broadcast(self.x())
    }

    #[inline]
    fn yx(self) -> Self {
        Vector2::set(self.y(), self.x())
    }

    #[inline]
    fn yy(self) -> Self {
        Vector2::broadcast(self.y())
    }

    #[inline]
    fn xxx(self) -> Self::Swizzle3 {
        Vector3::broadcast(self.x())
    }

    #[inline]
    fn xxy(self) -> Self::Swizzle3 {
        Vector3::set(self.x(), self.x(), self.y())
    }

    #[inline]
    fn xyx(self) -> Self::Swizzle3 {
        Vector3::set(self.x(), self.y(), self.x())
    }

    #[inline]
    fn xyy(self) -> Self::Swizzle3 {
        Vector3::set(self.x(), self.y(), self.y())
    }

    #[inline]
    fn yxx(self) -> Self::Swizzle3 {
        Vector3::set(self.y(), self.x(), self.x())
    }

    #[inline]
    fn yxy(self) -> Self::Swizzle3 {
        Vector3::set(self.y(), self.x(), self.y())
    }

    #[inline]
    fn yyx(self) -> Self::Swizzle3 {
        Vector3::set(self.y(), self.y(), self.x())
    }

    #[inline]
    fn yyy(self) -> Self::Swizzle3 {
        Vector3::broadcast(self.y())
    }

    #[inline]
    fn xxxx(self) -> Self::Swizzle4 {
        Vector4::broadcast(self.x())
    }

    #[inline]
    fn xxxy(self) -> Self::Swizzle4 {
        Vector4::set(self.x(), self.x(), self.x(), self.y())
    }

    #[inline]
    fn xxyx(self) -> Self::Swizzle4 {
        Vector4::set(self.x(), self.x(), self.y(), self.x())
    }

    #[inline]
    fn xxyy(self) -> Self::Swizzle4 {
        Vector4::set(self.x(), self.x(), self.y(), self.y())
    }

    #[inline]
    fn xyxx(self) -> Self::Swizzle4 {
        Vector4::set(self.x(), self.y(), self.x(), self.x())
    }

    #[inline]
    fn xyxy(self) -> Self::Swizzle4 {
        Vector4::set(self.x(), self.y(), self.x(), self.y())
    }

    #[inline]
    fn xyyx(self) -> Self::Swizzle4 {
        Vector4::set(self.x(), self.y(), self.y(), self.x())
    }

    #[inline]
    fn xyyy(self) -> Self::Swizzle4 {
        Vector4::set(self.x(), self.y(), self.y(), self.y())
    }

    #[inline]
    fn yxxx(self) -> Self::Swizzle4 {
        Vector4::set(self.y(), self.x(), self.x(), self.x())
    }

    #[inline]
    fn yxxy(self) -> Self::Swizzle4 {
        Vector4::set(self.y(), self.x(), self.x(), self.y())
    }

    fn yxyx(self) -> Self::Swizzle4 {
        Vector4::set(self.y(), self.x(), self.y(), self.x())
    }

    #[inline]
    fn yxyy(self) -> Self::Swizzle4 {
        Vector4::set(self.y(), self.x(), self.y(), self.y())
    }

    #[inline]
    fn yyxx(self) -> Self::Swizzle4 {
        Vector4::set(self.y(), self.y(), self.x(), self.x())
    }

    #[inline]
    fn yyxy(self) -> Self::Swizzle4 {
        Vector4::set(self.y(), self.y(), self.x(), self.y())
    }

    #[inline]
    fn yyyx(self) -> Self::Swizzle4 {
        Vector4::set(self.y(), self.y(), self.y(), self.x())
    }

    #[inline]
    fn yyyy(self) -> Self::Swizzle4 {
        Vector4::broadcast(self.y())
    }
}
