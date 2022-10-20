use crate::{Vector2, Vector3};

use crate::math_trait::Vec4Swizzles;

use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use std::fmt::{Display, Formatter};

use std::simd::Which::*;

#[derive(Copy, Clone, PartialEq, Debug)]
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

    #[inline]
    pub fn trunc_vec3(self) -> Vector3 {
        const ZERO_VEC: Vector4 = Vector4::ZERO;

        let swizzle_vec: std::simd::f32x4 = std::simd::simd_swizzle!(
            self.value,
            ZERO_VEC.value,
            [First(0), First(1), First(2), Second(0)]
        );

        Vector3 { value: swizzle_vec }
    }

    #[inline(always)]
    pub const fn set(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Vector4 {
            value: std::simd::f32x4::from_array([x, y, z, w]),
        }
    }

    #[inline]
    pub const fn splat(val: f32) -> Vector4 {
        Vector4 {
            value: std::simd::f32x4::from_array([val; 4]),
        }
    }

    #[inline]
    pub const fn to_primitive(self) -> [f32; 4] {
        self.value.to_array()
    }

    #[inline]
    pub const fn from_array(array: [f32; 4]) -> Vector4 {
        Vector4 {
            value: std::simd::f32x4::from_array(array),
        }
    }

    #[inline]
    pub const fn x(self) -> f32 {
        let array_vec4: [f32; 4] = self.value.to_array();

        array_vec4[0]
    }

    #[inline]
    pub const fn y(self) -> f32 {
        let array_vec4: [f32; 4] = self.value.to_array();

        array_vec4[1]
    }

    #[inline]
    pub const fn z(self) -> f32 {
        let array_vec4: [f32; 4] = self.value.to_array();

        array_vec4[2]
    }

    #[inline]
    pub const fn w(self) -> f32 {
        let array_vec4: [f32; 4] = self.value.to_array();

        array_vec4[3]
    }
}

impl Display for Vector4 {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            _f,
            "Vector4 (x : {}, y : {}, z : {}, w : {})",
            self.x(),
            self.y(),
            self.z(),
            self.w()
        )
    }
}


// Component-Wise
impl Mul<Vector4> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn mul(self, rhs: Vector4) -> Self::Output {
        Vector4 {
            value: self.value * rhs.value,
        }
    }
}

impl MulAssign<Vector4> for Vector4 {
    #[inline]
    fn mul_assign(&mut self, rhs: Vector4) {
        self.value *= rhs.value;
    }
}

impl Div<Vector4> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn div(self, rhs: Vector4) -> Self::Output {
        Vector4 {
            value: self.value / rhs.value,
        }
    }
}

impl DivAssign<Vector4> for Vector4 {
    #[inline]
    fn div_assign(&mut self, rhs: Vector4) {
        self.value /= rhs.value;
    }
}

impl Rem<Vector4> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn rem(self, rhs: Vector4) -> Self::Output {
        Vector4 {
            value: self.value % rhs.value,
        }
    }
}

impl RemAssign<Vector4> for Vector4 {
    #[inline]
    fn rem_assign(&mut self, rhs: Vector4) {
        self.value %= rhs.value;
    }
}

impl Add<f32> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector4 {
            value: self.value + splat_f32x4,
        }
    }
}

impl AddAssign<f32> for Vector4 {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value += splat_f32x4;
    }
}

impl Sub<f32> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector4 {
            value: self.value - splat_f32x4,
        }
    }
}


impl SubAssign<f32> for Vector4 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value -= splat_f32x4;
    }
}
//

// Vector-Wise

impl Mul<f32> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector4 {
            value: self.value * splat_f32x4,
        }
    }
}

impl MulAssign<f32> for Vector4 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value *= splat_f32x4;
    }
}

impl Div<f32> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector4 {
            value: self.value / splat_f32x4,
        }
    }
}

impl DivAssign<f32> for Vector4 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value /= splat_f32x4;
    }
}

impl Rem<f32> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn rem(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector4 {
            value: self.value % splat_f32x4,
        }
    }
}

impl RemAssign<f32> for Vector4 {
    #[inline]
    fn rem_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value %= splat_f32x4;
    }
}

impl Neg for Vector4 {
    type Output = Vector4;

    #[inline]
    fn neg(self) -> Self::Output {
        Vector4 { value: -self.value }
    }
}

impl Add<Vector4> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn add(self, rhs: Vector4) -> Self::Output {
        Vector4 {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign<Vector4> for Vector4 {
    #[inline]
    fn add_assign(&mut self, rhs: Vector4) {
        self.value += rhs.value;
    }
}

impl Sub<Vector4> for Vector4 {
    type Output = Vector4;

    #[inline]
    fn sub(self, rhs: Vector4) -> Self::Output {
        Vector4 {
            value: self.value - rhs.value,
        }
    }
}

impl SubAssign<Vector4> for Vector4 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vector4) {
        self.value -= rhs.value;
    }
}

impl Vec4Swizzles for Vector4 {
    type Vec2 = Vector2;
    type Vec3 = Vector3;

    #[inline]
    fn xx(self) -> Self::Vec2 {
        Vector2::splat(self.x())
    }

    #[inline]
    fn xy(self) -> Self::Vec2 {
        Vector2::set(self.x(), self.y())
    }

    #[inline]
    fn xz(self) -> Self::Vec2 {
        Vector2::set(self.x(), self.z())
    }

    #[inline]
    fn xw(self) -> Self::Vec2 {
        Vector2::set(self.x(), self.w())
    }

    #[inline]
    fn yx(self) -> Self::Vec2 {
        Vector2::set(self.y(), self.x())
    }

    #[inline]
    fn yy(self) -> Self::Vec2 {
        Vector2::splat(self.y())
    }

    #[inline]
    fn yz(self) -> Self::Vec2 {
        Vector2::set(self.y(), self.z())
    }

    #[inline]
    fn yw(self) -> Self::Vec2 {
        Vector2::set(self.y(), self.w())
    }

    #[inline]
    fn zx(self) -> Self::Vec2 {
        Vector2::set(self.z(), self.x())
    }

    #[inline]
    fn zy(self) -> Self::Vec2 {
        Vector2::set(self.z(), self.y())
    }

    #[inline]
    fn zz(self) -> Self::Vec2 {
        Vector2::splat(self.z())
    }

    #[inline]
    fn zw(self) -> Self::Vec2 {
        Vector2::set(self.z(), self.w())
    }

    #[inline]
    fn wx(self) -> Self::Vec2 {
        Vector2::set(self.w(), self.x())
    }

    #[inline]
    fn wy(self) -> Self::Vec2 {
        Vector2::set(self.w(), self.y())
    }

    #[inline]
    fn wz(self) -> Self::Vec2 {
        Vector2::set(self.w(), self.z())
    }

    #[inline]
    fn ww(self) -> Self::Vec2 {
        Vector2::splat(self.w())
    }

    #[inline]
    fn xxx(self) -> Self::Vec3 {
        Vector3::splat(self.x())
    }

    #[inline]
    fn xxy(self) -> Self::Vec3 {
        Vector3::set(self.x(), self.x(), self.y())
    }

    #[inline]
    fn xxz(self) -> Self::Vec3 {
        Vector3::set(self.x(), self.x(), self.z())
    }

    #[inline]
    fn xxw(self) -> Self::Vec3 {
        Vector3::set(self.x(), self.x(), self.w())
    }

    #[inline]
    fn xyx(self) -> Self::Vec3 {
        Vector3::set(self.x(), self.y(), self.x())
    }

    #[inline]
    fn xyy(self) -> Self::Vec3 {
        Vector3::set(self.x(), self.y(), self.y())
    }

    #[inline]
    fn xyz(self) -> Self::Vec3 {
        Vector3::set(self.x(), self.y(), self.z())
    }

    #[inline]
    fn xyw(self) -> Self::Vec3 {
        Vector3::set(self.x(), self.y(), self.w())
    }

    #[inline]
    fn xzx(self) -> Self::Vec3 {
        Vector3::set(self.x(), self.z(), self.x())
    }

    #[inline]
    fn xzy(self) -> Self::Vec3 {
        Vector3::set(self.x(), self.z(), self.y())
    }

    #[inline]
    fn xzz(self) -> Self::Vec3 {
        Vector3::set(self.x(), self.z(), self.z())
    }

    #[inline]
    fn xzw(self) -> Self::Vec3 {
        Vector3::set(self.x(), self.z(), self.w())
    }

    #[inline]
    fn xwx(self) -> Self::Vec3 {
        Vector3::set(self.x(), self.w(), self.x())
    }

    #[inline]
    fn xwy(self) -> Self::Vec3 {
        Vector3::set(self.x(), self.w(), self.y())
    }

    #[inline]
    fn xwz(self) -> Self::Vec3 {
        Vector3::set(self.x(), self.w(), self.z())
    }

    #[inline]
    fn xww(self) -> Self::Vec3 {
        Vector3::set(self.x(), self.w(), self.w())
    }

    #[inline]
    fn yxx(self) -> Self::Vec3 {
        Vector3::set(self.y(), self.x(), self.x())
    }

    #[inline]
    fn yxy(self) -> Self::Vec3 {
        Vector3::set(self.y(), self.x(), self.y())
    }

    #[inline]
    fn yxz(self) -> Self::Vec3 {
        Vector3::set(self.y(), self.x(), self.z())
    }

    #[inline]
    fn yxw(self) -> Self::Vec3 {
        Vector3::set(self.y(), self.x(), self.w())
    }

    #[inline]
    fn yyx(self) -> Self::Vec3 {
        Vector3::set(self.y(), self.y(), self.x())
    }

    #[inline]
    fn yyy(self) -> Self::Vec3 {
        Vector3::splat(self.y())
    }

    #[inline]
    fn yyz(self) -> Self::Vec3 {
        Vector3::set(self.y(), self.y(), self.z())
    }

    #[inline]
    fn yyw(self) -> Self::Vec3 {
        Vector3::set(self.y(), self.y(), self.w())
    }

    #[inline]
    fn yzx(self) -> Self::Vec3 {
        Vector3::set(self.y(), self.z(), self.x())
    }

    #[inline]
    fn yzy(self) -> Self::Vec3 {
        Vector3::set(self.y(), self.z(), self.y())
    }

    #[inline]
    fn yzz(self) -> Self::Vec3 {
        Vector3::set(self.y(), self.z(), self.z())
    }

    #[inline]
    fn yzw(self) -> Self::Vec3 {
        Vector3::set(self.y(), self.z(), self.w())
    }

    #[inline]
    fn ywx(self) -> Self::Vec3 {
        Vector3::set(self.y(), self.w(), self.x())
    }

    #[inline]
    fn ywy(self) -> Self::Vec3 {
        Vector3::set(self.y(), self.w(), self.y())
    }

    #[inline]
    fn ywz(self) -> Self::Vec3 {
        Vector3::set(self.y(), self.w(), self.z())
    }

    #[inline]
    fn yww(self) -> Self::Vec3 {
        Vector3::set(self.y(), self.w(), self.w())
    }

    #[inline]
    fn zxx(self) -> Self::Vec3 {
        Vector3::set(self.z(), self.x(), self.x())
    }

    #[inline]
    fn zxy(self) -> Self::Vec3 {
        Vector3::set(self.z(), self.x(), self.y())
    }

    #[inline]
    fn zxz(self) -> Self::Vec3 {
        Vector3::set(self.z(), self.x(), self.z())
    }

    #[inline]
    fn zxw(self) -> Self::Vec3 {
        Vector3::set(self.z(), self.x(), self.w())
    }

    #[inline]
    fn zyx(self) -> Self::Vec3 {
        Vector3::set(self.z(), self.y(), self.x())
    }

    #[inline]
    fn zyy(self) -> Self::Vec3 {
        Vector3::set(self.z(), self.y(), self.y())
    }

    #[inline]
    fn zyz(self) -> Self::Vec3 {
        Vector3::set(self.z(), self.y(), self.z())
    }

    #[inline]
    fn zyw(self) -> Self::Vec3 {
        Vector3::set(self.z(), self.y(), self.w())
    }

    #[inline]
    fn zzx(self) -> Self::Vec3 {
        Vector3::set(self.z(), self.z(), self.x())
    }

    #[inline]
    fn zzy(self) -> Self::Vec3 {
        Vector3::set(self.z(), self.z(), self.y())
    }

    #[inline]
    fn zzz(self) -> Self::Vec3 {
        Vector3::splat(self.z())
    }

    #[inline]
    fn zzw(self) -> Self::Vec3 {
        Vector3::set(self.z(), self.z(), self.w())
    }

    #[inline]
    fn zwx(self) -> Self::Vec3 {
        Vector3::set(self.z(), self.w(), self.x())
    }

    #[inline]
    fn zwy(self) -> Self::Vec3 {
        Vector3::set(self.z(), self.w(), self.y())
    }

    #[inline]
    fn zwz(self) -> Self::Vec3 {
        Vector3::set(self.z(), self.w(), self.z())
    }

    #[inline]
    fn zww(self) -> Self::Vec3 {
        Vector3::set(self.z(), self.w(), self.w())
    }

    #[inline]
    fn wxx(self) -> Self::Vec3 {
        Vector3::set(self.w(), self.x(), self.x())
    }

    #[inline]
    fn wxy(self) -> Self::Vec3 {
        Vector3::set(self.w(), self.x(), self.y())
    }

    #[inline]
    fn wxz(self) -> Self::Vec3 {
        Vector3::set(self.w(), self.x(), self.z())
    }

    #[inline]
    fn wxw(self) -> Self::Vec3 {
        Vector3::set(self.w(), self.x(), self.w())
    }

    #[inline]
    fn wyx(self) -> Self::Vec3 {
        Vector3::set(self.w(), self.y(), self.x())
    }

    #[inline]
    fn wyy(self) -> Self::Vec3 {
        Vector3::set(self.w(), self.y(), self.y())
    }

    #[inline]
    fn wyz(self) -> Self::Vec3 {
        Vector3::set(self.w(), self.y(), self.z())
    }

    #[inline]
    fn wyw(self) -> Self::Vec3 {
        Vector3::set(self.w(), self.y(), self.w())
    }

    #[inline]
    fn wzx(self) -> Self::Vec3 {
        Vector3::set(self.w(), self.z(), self.x())
    }

    #[inline]
    fn wzy(self) -> Self::Vec3 {
        Vector3::set(self.w(), self.z(), self.y())
    }

    #[inline]
    fn wzz(self) -> Self::Vec3 {
        Vector3::set(self.w(), self.z(), self.z())
    }

    #[inline]
    fn wzw(self) -> Self::Vec3 {
        Vector3::set(self.w(), self.z(), self.w())
    }

    #[inline]
    fn wwx(self) -> Self::Vec3 {
        Vector3::set(self.w(), self.w(), self.x())
    }

    #[inline]
    fn wwy(self) -> Self::Vec3 {
        Vector3::set(self.w(), self.w(), self.y())
    }

    #[inline]
    fn wwz(self) -> Self::Vec3 {
        Vector3::set(self.w(), self.w(), self.z())
    }

    #[inline]
    fn www(self) -> Self::Vec3 {
        Vector3::splat(self.w())
    }

    #[inline]
    fn xxxx(self) -> Self {
        Vector4::splat(self.x())
    }

    #[inline]
    fn xxxy(self) -> Self {
        let xxxy = std::simd::simd_swizzle!(self.value, [0,0,0,1]);

        Vector4 { value: xxxy }
    }

    #[inline]
    fn xxxz(self) -> Self {
        let xxxz = std::simd::simd_swizzle!(self.value, [0,0,0,2]);

        Vector4 { value: xxxz }
    }

    #[inline]
    fn xxxw(self) -> Self {
        let xxxw = std::simd::simd_swizzle!(self.value, [0,0,0,3]);

        Vector4 { value: xxxw }
    }

    #[inline]
    fn xxyx(self) -> Self {
        let xxyx = std::simd::simd_swizzle!(self.value, [0,0,1,0]);

        Vector4 { value: xxyx }
    }

    #[inline]
    fn xxyy(self) -> Self {
        let xxyy = std::simd::simd_swizzle!(self.value, [0,0,1,1]);

        Vector4 { value: xxyy }
    }

    #[inline]
    fn xxyz(self) -> Self {
        let xxyz = std::simd::simd_swizzle!(self.value, [0,0,1,2]);

        Vector4 { value: xxyz }
    }

    #[inline]
    fn xxyw(self) -> Self {
        let xxyw = std::simd::simd_swizzle!(self.value, [0,0,1,3]);

        Vector4 { value: xxyw }
    }

    #[inline]
    fn xxzx(self) -> Self {
        let xxzx = std::simd::simd_swizzle!(self.value, [0,0,2,0]);

        Vector4 { value: xxzx }
    }

    #[inline]
    fn xxzy(self) -> Self {
        let xxzy = std::simd::simd_swizzle!(self.value, [0,0,2,1]);

        Vector4 { value: xxzy }
    }

    #[inline]
    fn xxzz(self) -> Self {
        let xxzz = std::simd::simd_swizzle!(self.value, [0,0,2,2]);

        Vector4 { value: xxzz }
    }

    #[inline]
    fn xxzw(self) -> Self {
        let xxzw = std::simd::simd_swizzle!(self.value, [0,0,2,3]);

        Vector4 { value: xxzw }
    }

    #[inline]
    fn xxwx(self) -> Self {
        let xxwx = std::simd::simd_swizzle!(self.value, [0,0,3,0]);

        Vector4 { value: xxwx }
    }

    #[inline]
    fn xxwy(self) -> Self {
        let xxwy = std::simd::simd_swizzle!(self.value, [0,0,3,1]);

        Vector4 { value: xxwy }
    }

    #[inline]
    fn xxwz(self) -> Self {
        let xxwz = std::simd::simd_swizzle!(self.value, [0,0,3,2]);

        Vector4 { value: xxwz }
    }

    #[inline]
    fn xxww(self) -> Self {
        let xxww = std::simd::simd_swizzle!(self.value, [0,0,3,3]);

        Vector4 { value: xxww }
    }

    #[inline]
    fn xyxx(self) -> Self {
        let xyxx = std::simd::simd_swizzle!(self.value, [0,1,0,0]);

        Vector4 { value: xyxx }
    }

    #[inline]
    fn xyxy(self) -> Self {
        let xyxy = std::simd::simd_swizzle!(self.value, [0,1,0,1]);

        Vector4 { value: xyxy }
    }

    #[inline]
    fn xyxz(self) -> Self {
        let xyxz = std::simd::simd_swizzle!(self.value, [0,1,0,2]);

        Vector4 { value: xyxz }
    }

    #[inline]
    fn xyxw(self) -> Self {
        let xyxw = std::simd::simd_swizzle!(self.value, [0,1,0,3]);

        Vector4 { value: xyxw }
    }

    #[inline]
    fn xyyx(self) -> Self {
        let xyyx = std::simd::simd_swizzle!(self.value, [0,1,1,0]);

        Vector4 { value: xyyx }
    }

    #[inline]
    fn xyyy(self) -> Self {
        let xyyy = std::simd::simd_swizzle!(self.value, [0,1,1,1]);

        Vector4 { value: xyyy }
    }

    #[inline]
    fn xyyz(self) -> Self {
        let xyyz = std::simd::simd_swizzle!(self.value, [0,1,1,2]);

        Vector4 { value: xyyz }
    }

    #[inline]
    fn xyyw(self) -> Self {
        let xyyw = std::simd::simd_swizzle!(self.value, [0,1,1,3]);

        Vector4 { value: xyyw }
    }

    #[inline]
    fn xyzx(self) -> Self {
        let xyzx = std::simd::simd_swizzle!(self.value, [0,1,2,0]);

        Vector4 { value: xyzx }
    }

    #[inline]
    fn xyzy(self) -> Self {
        let xyzy = std::simd::simd_swizzle!(self.value, [0,1,2,1]);

        Vector4 { value: xyzy }
    }

    #[inline]
    fn xyzz(self) -> Self {
        let xyzz = std::simd::simd_swizzle!(self.value, [0,1,2,2]);

        Vector4 { value: xyzz }
    }

    #[inline]
    fn xywx(self) -> Self {
        let xywx = std::simd::simd_swizzle!(self.value, [0,1,3,0]);

        Vector4 { value: xywx }
    }

    #[inline]
    fn xywy(self) -> Self {
        let xywy = std::simd::simd_swizzle!(self.value, [0,1,3,1]);

        Vector4 { value: xywy }
    }

    #[inline]
    fn xywz(self) -> Self {
        let xywz = std::simd::simd_swizzle!(self.value, [0,1,3,2]);

        Vector4 { value: xywz }
    }

    #[inline]
    fn xyww(self) -> Self {
        let xyww = std::simd::simd_swizzle!(self.value, [0,1,3,3]);

        Vector4 { value: xyww }
    }

    #[inline]
    fn xzxx(self) -> Self {
        let xzxx = std::simd::simd_swizzle!(self.value, [0,2,0,0]);

        Vector4 { value: xzxx }
    }

    #[inline]
    fn xzxy(self) -> Self {
        let xzxy = std::simd::simd_swizzle!(self.value, [0,2,0,1]);

        Vector4 { value: xzxy }
    }

    #[inline]
    fn xzxz(self) -> Self {
        let xzxz = std::simd::simd_swizzle!(self.value, [0,2,0,2]);

        Vector4 { value: xzxz }
    }

    #[inline]
    fn xzxw(self) -> Self {
        let xzxw = std::simd::simd_swizzle!(self.value, [0,2,0,3]);

        Vector4 { value: xzxw }
    }

    #[inline]
    fn xzyx(self) -> Self {
        let xzyx = std::simd::simd_swizzle!(self.value, [0,2,1,0]);

        Vector4 { value: xzyx }
    }

    #[inline]
    fn xzyy(self) -> Self {
        let xzyy = std::simd::simd_swizzle!(self.value, [0,2,1,1]);

        Vector4 { value: xzyy }
    }

    #[inline]
    fn xzyz(self) -> Self {
        let xzyz = std::simd::simd_swizzle!(self.value, [0,2,1,2]);

        Vector4 { value: xzyz }
    }

    #[inline]
    fn xzyw(self) -> Self {
        let xzyw = std::simd::simd_swizzle!(self.value, [0,2,1,3]);

        Vector4 { value: xzyw }
    }

    #[inline]
    fn xzzx(self) -> Self {
        let xzzx = std::simd::simd_swizzle!(self.value, [0,2,2,0]);

        Vector4 { value: xzzx }
    }

    #[inline]
    fn xzzy(self) -> Self {
        let xzzy = std::simd::simd_swizzle!(self.value, [0,2,2,1]);

        Vector4 { value: xzzy }
    }

    #[inline]
    fn xzzz(self) -> Self {
        let xzzz = std::simd::simd_swizzle!(self.value, [0,2,2,2]);

        Vector4 { value: xzzz }
    }

    #[inline]
    fn xzzw(self) -> Self {
        let xzzw = std::simd::simd_swizzle!(self.value, [0,2,2,3]);

        Vector4 { value: xzzw }
    }

    #[inline]
    fn xzwx(self) -> Self {
        let xzwx = std::simd::simd_swizzle!(self.value, [0,2,3,0]);

        Vector4 { value: xzwx }
    }

    #[inline]
    fn xzwy(self) -> Self {
        let xzwy = std::simd::simd_swizzle!(self.value, [0,2,3,1]);

        Vector4 { value: xzwy }
    }

    #[inline]
    fn xzwz(self) -> Self {
        let xzwz = std::simd::simd_swizzle!(self.value, [0,2,3,2]);

        Vector4 { value: xzwz }
    }

    #[inline]
    fn xzww(self) -> Self {
        let xzww = std::simd::simd_swizzle!(self.value, [0,2,3,3]);

        Vector4 { value: xzww }
    }

    #[inline]
    fn xwxx(self) -> Self {
        let xwxx = std::simd::simd_swizzle!(self.value, [0,3,0,0]);

        Vector4 { value: xwxx }
    }

    #[inline]
    fn xwxy(self) -> Self {
        let xwxy = std::simd::simd_swizzle!(self.value, [0,3,0,1]);

        Vector4 { value: xwxy }
    }

    #[inline]
    fn xwxz(self) -> Self {
        let xwxz = std::simd::simd_swizzle!(self.value, [0,3,0,2]);

        Vector4 { value: xwxz }
    }

    #[inline]
    fn xwxw(self) -> Self {
        let xwxw = std::simd::simd_swizzle!(self.value, [0,3,0,3]);

        Vector4 { value: xwxw }
    }

    #[inline]
    fn xwyx(self) -> Self {
        let xwyx = std::simd::simd_swizzle!(self.value, [0,3,1,0]);

        Vector4 { value: xwyx }
    }

    #[inline]
    fn xwyy(self) -> Self {
        let xwyy = std::simd::simd_swizzle!(self.value, [0,3,1,1]);

        Vector4 { value: xwyy }
    }

    #[inline]
    fn xwyz(self) -> Self {
        let xwyz = std::simd::simd_swizzle!(self.value, [0,3,1,2]);

        Vector4 { value: xwyz }
    }

    #[inline]
    fn xwyw(self) -> Self {
        let xwyw = std::simd::simd_swizzle!(self.value, [0,3,1,3]);

        Vector4 { value: xwyw }
    }

    #[inline]
    fn xwzx(self) -> Self {
        let xwzx = std::simd::simd_swizzle!(self.value, [0,3,2,0]);

        Vector4 { value: xwzx }
    }

    #[inline]
    fn xwzy(self) -> Self {
        let xwzy = std::simd::simd_swizzle!(self.value, [0,3,2,1]);

        Vector4 { value: xwzy }
    }

    #[inline]
    fn xwzz(self) -> Self {
        let xwzz = std::simd::simd_swizzle!(self.value, [0,3,2,2]);

        Vector4 { value: xwzz }
    }

    #[inline]
    fn xwzw(self) -> Self {
        let xwzw = std::simd::simd_swizzle!(self.value, [0,3,2,3]);

        Vector4 { value: xwzw }
    }

    #[inline]
    fn xwwx(self) -> Self {
        let xwwx = std::simd::simd_swizzle!(self.value, [0,3,3,0]);

        Vector4 { value: xwwx }
    }

    #[inline]
    fn xwwy(self) -> Self {
        let xwwy = std::simd::simd_swizzle!(self.value, [0,3,3,1]);

        Vector4 { value: xwwy }
    }

    #[inline]
    fn xwwz(self) -> Self {
        let xwwz = std::simd::simd_swizzle!(self.value, [0,3,3,2]);

        Vector4 { value: xwwz }
    }

    #[inline]
    fn xwww(self) -> Self {
        let xwww = std::simd::simd_swizzle!(self.value, [0,3,3,3]);

        Vector4 { value: xwww }
    }

    #[inline]
    fn yxxx(self) -> Self {
        let yxxx = std::simd::simd_swizzle!(self.value, [1,0,0,0]);

        Vector4 { value: yxxx }
    }

    #[inline]
    fn yxxy(self) -> Self {
        let yxxy = std::simd::simd_swizzle!(self.value, [1,0,0,1]);

        Vector4 { value: yxxy }
    }

    #[inline]
    fn yxxz(self) -> Self {
        let yxxz = std::simd::simd_swizzle!(self.value, [1,0,0,2]);

        Vector4 { value: yxxz }
    }

    #[inline]
    fn yxxw(self) -> Self {
        let yxxw = std::simd::simd_swizzle!(self.value, [1,0,0,3]);

        Vector4 { value: yxxw }
    }

    #[inline]
    fn yxyx(self) -> Self {
        let yxyx = std::simd::simd_swizzle!(self.value, [1,0,1,0]);

        Vector4 { value: yxyx }
    }

    #[inline]
    fn yxyy(self) -> Self {
        let yxyy = std::simd::simd_swizzle!(self.value, [1,0,1,1]);

        Vector4 { value: yxyy }
    }

    #[inline]
    fn yxyz(self) -> Self {
        let yxyz = std::simd::simd_swizzle!(self.value, [1,0,1,2]);

        Vector4 { value: yxyz }
    }

    #[inline]
    fn yxyw(self) -> Self {
        let yxyw = std::simd::simd_swizzle!(self.value, [1,0,1,3]);

        Vector4 { value: yxyw }
    }

    #[inline]
    fn yxzx(self) -> Self {
        let yxzx = std::simd::simd_swizzle!(self.value, [1,0,2,0]);

        Vector4 { value: yxzx }
    }

    #[inline]
    fn yxzy(self) -> Self {
        let yxzy = std::simd::simd_swizzle!(self.value, [1,0,2,1]);

        Vector4 { value: yxzy }
    }

    #[inline]
    fn yxzz(self) -> Self {
        let yxzz = std::simd::simd_swizzle!(self.value, [1,0,2,2]);

        Vector4 { value: yxzz }
    }

    #[inline]
    fn yxzw(self) -> Self {
        let yxzw = std::simd::simd_swizzle!(self.value, [1,0,2,3]);

        Vector4 { value: yxzw }
    }

    #[inline]
    fn yxwx(self) -> Self {
        let yxwx = std::simd::simd_swizzle!(self.value, [1,0,3,0]);

        Vector4 { value: yxwx }
    }

    #[inline]
    fn yxwy(self) -> Self {
        let yxwy = std::simd::simd_swizzle!(self.value, [1,0,3,1]);

        Vector4 { value: yxwy }
    }

    #[inline]
    fn yxwz(self) -> Self {
        let yxwz = std::simd::simd_swizzle!(self.value, [1,0,3,2]);

        Vector4 { value: yxwz }
    }

    #[inline]
    fn yxww(self) -> Self {
        let yxww = std::simd::simd_swizzle!(self.value, [1,0,3,3]);

        Vector4 { value: yxww }
    }

    #[inline]
    fn yyxx(self) -> Self {
        todo!()
    }

    #[inline]
    fn yyxy(self) -> Self {
        todo!()
    }

    #[inline]
    fn yyxz(self) -> Self {
        todo!()
    }

    #[inline]
    fn yyxw(self) -> Self {
        todo!()
    }

    #[inline]
    fn yyyx(self) -> Self {
        todo!()
    }

    #[inline]
    fn yyyy(self) -> Self {
        todo!()
    }

    #[inline]
    fn yyyz(self) -> Self {
        todo!()
    }

    #[inline]
    fn yyyw(self) -> Self {
        todo!()
    }

    #[inline]
    fn yyzx(self) -> Self {
        todo!()
    }

    #[inline]
    fn yyzy(self) -> Self {
        todo!()
    }

    #[inline]
    fn yyzz(self) -> Self {
        todo!()
    }

    #[inline]
    fn yyzw(self) -> Self {
        todo!()
    }

    #[inline]
    fn yywx(self) -> Self {
        todo!()
    }

    #[inline]
    fn yywy(self) -> Self {
        todo!()
    }

    #[inline]
    fn yywz(self) -> Self {
        todo!()
    }

    #[inline]
    fn yyww(self) -> Self {
        todo!()
    }

    #[inline]
    fn yzxx(self) -> Self {
        todo!()
    }

    #[inline]
    fn yzxy(self) -> Self {
        todo!()
    }

    #[inline]
    fn yzxz(self) -> Self {
        todo!()
    }

    #[inline]
    fn yzxw(self) -> Self {
        todo!()
    }

    #[inline]
    fn yzyx(self) -> Self {
        todo!()
    }

    #[inline]
    fn yzyy(self) -> Self {
        todo!()
    }

    #[inline]
    fn yzyz(self) -> Self {
        todo!()
    }

    #[inline]
    fn yzyw(self) -> Self {
        todo!()
    }

    #[inline]
    fn yzzx(self) -> Self {
        todo!()
    }

    fn yzzy(self) -> Self {
        todo!()
    }

    fn yzzz(self) -> Self {
        todo!()
    }

    fn yzzw(self) -> Self {
        todo!()
    }

    fn yzwx(self) -> Self {
        todo!()
    }

    fn yzwy(self) -> Self {
        todo!()
    }

    fn yzwz(self) -> Self {
        todo!()
    }

    fn yzww(self) -> Self {
        todo!()
    }

    fn ywxx(self) -> Self {
        todo!()
    }

    fn ywxy(self) -> Self {
        todo!()
    }

    fn ywxz(self) -> Self {
        todo!()
    }

    fn ywxw(self) -> Self {
        todo!()
    }

    fn ywyx(self) -> Self {
        todo!()
    }

    fn ywyy(self) -> Self {
        todo!()
    }

    fn ywyz(self) -> Self {
        todo!()
    }

    fn ywyw(self) -> Self {
        todo!()
    }

    fn ywzx(self) -> Self {
        todo!()
    }

    fn ywzy(self) -> Self {
        todo!()
    }

    fn ywzz(self) -> Self {
        todo!()
    }

    fn ywzw(self) -> Self {
        todo!()
    }

    fn ywwx(self) -> Self {
        todo!()
    }

    fn ywwy(self) -> Self {
        todo!()
    }

    fn ywwz(self) -> Self {
        todo!()
    }

    fn ywww(self) -> Self {
        todo!()
    }

    fn zxxx(self) -> Self {
        todo!()
    }

    fn zxxy(self) -> Self {
        todo!()
    }

    fn zxxz(self) -> Self {
        todo!()
    }

    fn zxxw(self) -> Self {
        todo!()
    }

    fn zxyx(self) -> Self {
        todo!()
    }

    fn zxyy(self) -> Self {
        todo!()
    }

    fn zxyz(self) -> Self {
        todo!()
    }

    fn zxyw(self) -> Self {
        todo!()
    }

    fn zxzx(self) -> Self {
        todo!()
    }

    fn zxzy(self) -> Self {
        todo!()
    }

    fn zxzz(self) -> Self {
        todo!()
    }

    fn zxzw(self) -> Self {
        todo!()
    }

    fn zxwx(self) -> Self {
        todo!()
    }

    fn zxwy(self) -> Self {
        todo!()
    }

    fn zxwz(self) -> Self {
        todo!()
    }

    fn zxww(self) -> Self {
        todo!()
    }

    fn zyxx(self) -> Self {
        todo!()
    }

    fn zyxy(self) -> Self {
        todo!()
    }

    fn zyxz(self) -> Self {
        todo!()
    }

    fn zyxw(self) -> Self {
        todo!()
    }

    fn zyyx(self) -> Self {
        todo!()
    }

    fn zyyy(self) -> Self {
        todo!()
    }

    fn zyyz(self) -> Self {
        todo!()
    }

    fn zyyw(self) -> Self {
        todo!()
    }

    fn zyzx(self) -> Self {
        todo!()
    }

    fn zyzy(self) -> Self {
        todo!()
    }

    fn zyzz(self) -> Self {
        todo!()
    }

    fn zyzw(self) -> Self {
        todo!()
    }

    fn zywx(self) -> Self {
        todo!()
    }

    fn zywy(self) -> Self {
        todo!()
    }

    fn zywz(self) -> Self {
        todo!()
    }

    fn zyww(self) -> Self {
        todo!()
    }

    fn zzxx(self) -> Self {
        todo!()
    }

    fn zzxy(self) -> Self {
        todo!()
    }

    fn zzxz(self) -> Self {
        todo!()
    }

    fn zzxw(self) -> Self {
        todo!()
    }

    fn zzyx(self) -> Self {
        todo!()
    }

    fn zzyy(self) -> Self {
        todo!()
    }

    fn zzyz(self) -> Self {
        todo!()
    }

    fn zzyw(self) -> Self {
        todo!()
    }

    fn zzzx(self) -> Self {
        todo!()
    }

    fn zzzy(self) -> Self {
        todo!()
    }

    fn zzzz(self) -> Self {
        todo!()
    }

    fn zzzw(self) -> Self {
        todo!()
    }

    fn zzwx(self) -> Self {
        todo!()
    }

    fn zzwy(self) -> Self {
        todo!()
    }

    fn zzwz(self) -> Self {
        todo!()
    }

    fn zzww(self) -> Self {
        todo!()
    }

    fn zwxx(self) -> Self {
        todo!()
    }

    fn zwxy(self) -> Self {
        todo!()
    }

    fn zwxz(self) -> Self {
        todo!()
    }

    fn zwxw(self) -> Self {
        todo!()
    }

    fn zwyx(self) -> Self {
        todo!()
    }

    fn zwyy(self) -> Self {
        todo!()
    }

    fn zwyz(self) -> Self {
        todo!()
    }

    fn zwyw(self) -> Self {
        todo!()
    }

    fn zwzx(self) -> Self {
        todo!()
    }

    fn zwzy(self) -> Self {
        todo!()
    }

    fn zwzz(self) -> Self {
        todo!()
    }

    fn zwzw(self) -> Self {
        todo!()
    }

    fn zwwx(self) -> Self {
        todo!()
    }

    fn zwwy(self) -> Self {
        todo!()
    }

    fn zwwz(self) -> Self {
        todo!()
    }

    fn zwww(self) -> Self {
        todo!()
    }

    fn wxxx(self) -> Self {
        todo!()
    }

    fn wxxy(self) -> Self {
        todo!()
    }

    fn wxxz(self) -> Self {
        todo!()
    }

    fn wxxw(self) -> Self {
        todo!()
    }

    fn wxyx(self) -> Self {
        todo!()
    }

    fn wxyy(self) -> Self {
        todo!()
    }

    fn wxyz(self) -> Self {
        todo!()
    }

    fn wxyw(self) -> Self {
        todo!()
    }

    fn wxzx(self) -> Self {
        todo!()
    }

    #[inline]
    fn wxzy(self) -> Self {
        let wxzy = std::simd::simd_swizzle!(self.value, [3, 0, 2, 1]);

        Vector4 { value: wxzy }
    }

    fn wxzz(self) -> Self {
        todo!()
    }

    fn wxzw(self) -> Self {
        todo!()
    }

    fn wxwx(self) -> Self {
        todo!()
    }

    fn wxwy(self) -> Self {
        todo!()
    }

    fn wxwz(self) -> Self {
        todo!()
    }

    fn wxww(self) -> Self {
        todo!()
    }

    fn wyxx(self) -> Self {
        todo!()
    }

    fn wyxy(self) -> Self {
        todo!()
    }

    #[inline]
    fn wyxz(self) -> Self {
        let wyxz = std::simd::simd_swizzle!(self.value, [3, 1, 0, 2]);

        Vector4 { value: wyxz }
    }

    fn wyxw(self) -> Self {
        todo!()
    }

    fn wyyx(self) -> Self {
        todo!()
    }

    fn wyyy(self) -> Self {
        todo!()
    }

    fn wyyz(self) -> Self {
        todo!()
    }

    fn wyyw(self) -> Self {
        todo!()
    }

    fn wyzx(self) -> Self {
        todo!()
    }

    fn wyzy(self) -> Self {
        todo!()
    }

    fn wyzz(self) -> Self {
        todo!()
    }

    fn wyzw(self) -> Self {
        todo!()
    }

    fn wywx(self) -> Self {
        todo!()
    }

    fn wywy(self) -> Self {
        todo!()
    }

    fn wywz(self) -> Self {
        todo!()
    }

    fn wyww(self) -> Self {
        todo!()
    }

    fn wzxx(self) -> Self {
        todo!()
    }

    fn wzxy(self) -> Self {
        todo!()
    }

    fn wzxz(self) -> Self {
        todo!()
    }

    fn wzxw(self) -> Self {
        todo!()
    }

    fn wzyx(self) -> Self {
        todo!()
    }

    fn wzyy(self) -> Self {
        todo!()
    }

    fn wzyz(self) -> Self {
        todo!()
    }

    fn wzyw(self) -> Self {
        todo!()
    }

    fn wzzx(self) -> Self {
        todo!()
    }

    fn wzzy(self) -> Self {
        todo!()
    }

    fn wzzz(self) -> Self {
        todo!()
    }

    fn wzzw(self) -> Self {
        todo!()
    }

    fn wzwx(self) -> Self {
        todo!()
    }

    fn wzwy(self) -> Self {
        todo!()
    }

    fn wzwz(self) -> Self {
        todo!()
    }

    fn wzww(self) -> Self {
        todo!()
    }

    #[inline]
    fn wwxx(self) -> Self {
        todo!()
    }

    #[inline]
    fn wwxy(self) -> Self {
        todo!()
    }

    #[inline]
    fn wwxz(self) -> Self {
        todo!()
    }

    #[inline]
    fn wwxw(self) -> Self {
        todo!()
    }

    #[inline]
    fn wwyx(self) -> Self {
        todo!()
    }

    #[inline]
    fn wwyy(self) -> Self {
        todo!()
    }

    #[inline]
    fn wwyz(self) -> Self {
        todo!()
    }

    #[inline]
    fn wwyw(self) -> Self {
        todo!()
    }

    #[inline]
    fn wwzx(self) -> Self {
        todo!()
    }

    #[inline]
    fn wwzy(self) -> Self {
        todo!()
    }

    #[inline]
    fn wwzz(self) -> Self {
        todo!()
    }

    #[inline]
    fn wwzw(self) -> Self {
        todo!()
    }

    #[inline]
    fn wwwx(self) -> Self {
        todo!()
    }

    #[inline]
    fn wwwy(self) -> Self {
        todo!()
    }

    #[inline]
    fn wwwz(self) -> Self {
        todo!()
    }

    #[inline]
    fn wwww(self) -> Self {
        todo!()
    }
}
