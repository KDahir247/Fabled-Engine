use crate::{Vector2, Vector3, Matrix4x4};

use crate::math_trait::Swizzles4;

use crate::matrix4x4_math::transpose_mat4;
use crate::vector_math::component_sum;

use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use std::fmt::{Display, Formatter};

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
    pub const W: Vector4 = Vector4{
        value: std::simd::f32x4::from_array([0.0, 0.0, 0.0, 1.0]),
    };

    pub const NEG_W: Vector4 = Vector4{
        value: std::simd::f32x4::from_array([0.0, 0.0, 0.0, -1.0]),
    };

    pub const BACKWARD: Vector4 = Vector4{
        value: std::simd::f32x4::from_array([0.0, 0.0, -1.0, 0.0]),
    };

    pub const FORWARD: Vector4 = Vector4{
        value: std::simd::f32x4::from_array([0.0, 0.0, 1.0, 0.0]),
    };

    pub const UP: Vector4 = Vector4{
        value: std::simd::f32x4::from_array([0.0, 1.0, 0.0, 0.0]),
    };

    pub const DOWN: Vector4 = Vector4{
        value: std::simd::f32x4::from_array([0.0, -1.0, 0.0, 0.0]),
    };

    pub const RIGHT: Vector4 = Vector4{
        value: std::simd::f32x4::from_array([1.0, 0.0, 0.0, 0.0]),
    };

    pub const LEFT: Vector4 = Vector4{
        value: std::simd::f32x4::from_array([-1.0, 0.0, 0.0, 0.0]),
    };

    pub const ONE: Vector4 = Vector4 {
        value: std::simd::f32x4::from_array([1.0, 1.0, 1.0, 1.0]),
    };

    pub const NEG_ONE: Vector4 = Vector4{
        value: std::simd::f32x4::from_array([-1.0, -1.0, -1.0, -1.0]),
    };

    pub const ZERO: Vector4 = Vector4 {
        value: std::simd::f32x4::from_array([0.0, 0.0, 0.0, 0.0]),
    };

    #[inline]
    pub const fn trunc_vec3(self) -> Vector3 {
        Vector3::set(self.x(), self.y(), self.z())
    }

    #[inline(always)]
    pub const fn set(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Vector4 {
            value: std::simd::f32x4::from_array([x, y, z, w]),
        }
    }

    #[inline]
    pub const fn broadcast(val: f32) -> Vector4 {
        Vector4 {
            value: std::simd::f32x4::from_array([val; 4]),
        }
    }

    #[inline]
    pub const fn to_primitive(self) -> [f32; 4] {
        self.value.to_array()
    }

    #[inline]
    pub const fn from_primitive(array: [f32; 4]) -> Vector4 {
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


impl Mul<Vector4> for Matrix4x4{
    type Output = Vector4;

    #[inline]
    fn mul(self, rhs: Vector4) -> Self::Output {
        let transpose_matrix : Matrix4x4 = transpose_mat4(self);

        let transpose_x : Vector4 = transpose_matrix.column_x * rhs;
        let transpose_y : Vector4 = transpose_matrix.column_y * rhs;
        let transpose_z : Vector4 = transpose_matrix.column_z * rhs;
        let transpose_w : Vector4 = transpose_matrix.column_w * rhs;

        Vector4::set(
            component_sum(transpose_x.value),
            component_sum(transpose_y.value),
            component_sum(transpose_z.value),
            component_sum(transpose_w.value)
        )
    }
}


impl MulAssign<Matrix4x4> for Vector4{
    #[inline]
    fn mul_assign(&mut self, rhs: Matrix4x4) {
        self.value = (rhs * *self).value;
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
        Vector4 {
            value: self.value.neg(),
        }
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

impl Swizzles4 for Vector4 {
    type Swizzle2 = Vector2;
    type Swizzle3 = Vector3;

    #[inline]
    fn xx(self) -> Self::Swizzle2 {
        Vector2::broadcast(self.x())
    }

    #[inline]
    fn xy(self) -> Self::Swizzle2 {
        Vector2::set(self.x(), self.y())
    }

    #[inline]
    fn xz(self) -> Self::Swizzle2 {
        Vector2::set(self.x(), self.z())
    }

    #[inline]
    fn xw(self) -> Self::Swizzle2 {
        Vector2::set(self.x(), self.w())
    }

    #[inline]
    fn yx(self) -> Self::Swizzle2 {
        Vector2::set(self.y(), self.x())
    }

    #[inline]
    fn yy(self) -> Self::Swizzle2 {
        Vector2::broadcast(self.y())
    }

    #[inline]
    fn yz(self) -> Self::Swizzle2 {
        Vector2::set(self.y(), self.z())
    }

    #[inline]
    fn yw(self) -> Self::Swizzle2 {
        Vector2::set(self.y(), self.w())
    }

    #[inline]
    fn zx(self) -> Self::Swizzle2 {
        Vector2::set(self.z(), self.x())
    }

    #[inline]
    fn zy(self) -> Self::Swizzle2 {
        Vector2::set(self.z(), self.y())
    }

    #[inline]
    fn zz(self) -> Self::Swizzle2 {
        Vector2::broadcast(self.z())
    }

    #[inline]
    fn zw(self) -> Self::Swizzle2 {
        Vector2::set(self.z(), self.w())
    }

    #[inline]
    fn wx(self) -> Self::Swizzle2 {
        Vector2::set(self.w(), self.x())
    }

    #[inline]
    fn wy(self) -> Self::Swizzle2 {
        Vector2::set(self.w(), self.y())
    }

    #[inline]
    fn wz(self) -> Self::Swizzle2 {
        Vector2::set(self.w(), self.z())
    }

    #[inline]
    fn ww(self) -> Self::Swizzle2 {
        Vector2::broadcast(self.w())
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
    fn xxz(self) -> Self::Swizzle3 {
        Vector3::set(self.x(), self.x(), self.z())
    }

    #[inline]
    fn xxw(self) -> Self::Swizzle3 {
        Vector3::set(self.x(), self.x(), self.w())
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
    fn xyz(self) -> Self::Swizzle3 {
        Vector3::set(self.x(), self.y(), self.z())
    }

    #[inline]
    fn xyw(self) -> Self::Swizzle3 {
        Vector3::set(self.x(), self.y(), self.w())
    }

    #[inline]
    fn xzx(self) -> Self::Swizzle3 {
        Vector3::set(self.x(), self.z(), self.x())
    }

    #[inline]
    fn xzy(self) -> Self::Swizzle3 {
        Vector3::set(self.x(), self.z(), self.y())
    }

    #[inline]
    fn xzz(self) -> Self::Swizzle3 {
        Vector3::set(self.x(), self.z(), self.z())
    }

    #[inline]
    fn xzw(self) -> Self::Swizzle3 {
        Vector3::set(self.x(), self.z(), self.w())
    }

    #[inline]
    fn xwx(self) -> Self::Swizzle3 {
        Vector3::set(self.x(), self.w(), self.x())
    }

    #[inline]
    fn xwy(self) -> Self::Swizzle3 {
        Vector3::set(self.x(), self.w(), self.y())
    }

    #[inline]
    fn xwz(self) -> Self::Swizzle3 {
        Vector3::set(self.x(), self.w(), self.z())
    }

    #[inline]
    fn xww(self) -> Self::Swizzle3 {
        Vector3::set(self.x(), self.w(), self.w())
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
    fn yxz(self) -> Self::Swizzle3 {
        Vector3::set(self.y(), self.x(), self.z())
    }

    #[inline]
    fn yxw(self) -> Self::Swizzle3 {
        Vector3::set(self.y(), self.x(), self.w())
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
    fn yyz(self) -> Self::Swizzle3 {
        Vector3::set(self.y(), self.y(), self.z())
    }

    #[inline]
    fn yyw(self) -> Self::Swizzle3 {
        Vector3::set(self.y(), self.y(), self.w())
    }

    #[inline]
    fn yzx(self) -> Self::Swizzle3 {
        Vector3::set(self.y(), self.z(), self.x())
    }

    #[inline]
    fn yzy(self) -> Self::Swizzle3 {
        Vector3::set(self.y(), self.z(), self.y())
    }

    #[inline]
    fn yzz(self) -> Self::Swizzle3 {
        Vector3::set(self.y(), self.z(), self.z())
    }

    #[inline]
    fn yzw(self) -> Self::Swizzle3 {
        Vector3::set(self.y(), self.z(), self.w())
    }

    #[inline]
    fn ywx(self) -> Self::Swizzle3 {
        Vector3::set(self.y(), self.w(), self.x())
    }

    #[inline]
    fn ywy(self) -> Self::Swizzle3 {
        Vector3::set(self.y(), self.w(), self.y())
    }

    #[inline]
    fn ywz(self) -> Self::Swizzle3 {
        Vector3::set(self.y(), self.w(), self.z())
    }

    #[inline]
    fn yww(self) -> Self::Swizzle3 {
        Vector3::set(self.y(), self.w(), self.w())
    }

    #[inline]
    fn zxx(self) -> Self::Swizzle3 {
        Vector3::set(self.z(), self.x(), self.x())
    }

    #[inline]
    fn zxy(self) -> Self::Swizzle3 {
        Vector3::set(self.z(), self.x(), self.y())
    }

    #[inline]
    fn zxz(self) -> Self::Swizzle3 {
        Vector3::set(self.z(), self.x(), self.z())
    }

    #[inline]
    fn zxw(self) -> Self::Swizzle3 {
        Vector3::set(self.z(), self.x(), self.w())
    }

    #[inline]
    fn zyx(self) -> Self::Swizzle3 {
        Vector3::set(self.z(), self.y(), self.x())
    }

    #[inline]
    fn zyy(self) -> Self::Swizzle3 {
        Vector3::set(self.z(), self.y(), self.y())
    }

    #[inline]
    fn zyz(self) -> Self::Swizzle3 {
        Vector3::set(self.z(), self.y(), self.z())
    }

    #[inline]
    fn zyw(self) -> Self::Swizzle3 {
        Vector3::set(self.z(), self.y(), self.w())
    }

    #[inline]
    fn zzx(self) -> Self::Swizzle3 {
        Vector3::set(self.z(), self.z(), self.x())
    }

    #[inline]
    fn zzy(self) -> Self::Swizzle3 {
        Vector3::set(self.z(), self.z(), self.y())
    }

    #[inline]
    fn zzz(self) -> Self::Swizzle3 {
        Vector3::broadcast(self.z())
    }

    #[inline]
    fn zzw(self) -> Self::Swizzle3 {
        Vector3::set(self.z(), self.z(), self.w())
    }

    #[inline]
    fn zwx(self) -> Self::Swizzle3 {
        Vector3::set(self.z(), self.w(), self.x())
    }

    #[inline]
    fn zwy(self) -> Self::Swizzle3 {
        Vector3::set(self.z(), self.w(), self.y())
    }

    #[inline]
    fn zwz(self) -> Self::Swizzle3 {
        Vector3::set(self.z(), self.w(), self.z())
    }

    #[inline]
    fn zww(self) -> Self::Swizzle3 {
        Vector3::set(self.z(), self.w(), self.w())
    }

    #[inline]
    fn wxx(self) -> Self::Swizzle3 {
        Vector3::set(self.w(), self.x(), self.x())
    }

    #[inline]
    fn wxy(self) -> Self::Swizzle3 {
        Vector3::set(self.w(), self.x(), self.y())
    }

    #[inline]
    fn wxz(self) -> Self::Swizzle3 {
        Vector3::set(self.w(), self.x(), self.z())
    }

    #[inline]
    fn wxw(self) -> Self::Swizzle3 {
        Vector3::set(self.w(), self.x(), self.w())
    }

    #[inline]
    fn wyx(self) -> Self::Swizzle3 {
        Vector3::set(self.w(), self.y(), self.x())
    }

    #[inline]
    fn wyy(self) -> Self::Swizzle3 {
        Vector3::set(self.w(), self.y(), self.y())
    }

    #[inline]
    fn wyz(self) -> Self::Swizzle3 {
        Vector3::set(self.w(), self.y(), self.z())
    }

    #[inline]
    fn wyw(self) -> Self::Swizzle3 {
        Vector3::set(self.w(), self.y(), self.w())
    }

    #[inline]
    fn wzx(self) -> Self::Swizzle3 {
        Vector3::set(self.w(), self.z(), self.x())
    }

    #[inline]
    fn wzy(self) -> Self::Swizzle3 {
        Vector3::set(self.w(), self.z(), self.y())
    }

    #[inline]
    fn wzz(self) -> Self::Swizzle3 {
        Vector3::set(self.w(), self.z(), self.z())
    }

    #[inline]
    fn wzw(self) -> Self::Swizzle3 {
        Vector3::set(self.w(), self.z(), self.w())
    }

    #[inline]
    fn wwx(self) -> Self::Swizzle3 {
        Vector3::set(self.w(), self.w(), self.x())
    }

    #[inline]
    fn wwy(self) -> Self::Swizzle3 {
        Vector3::set(self.w(), self.w(), self.y())
    }

    #[inline]
    fn wwz(self) -> Self::Swizzle3 {
        Vector3::set(self.w(), self.w(), self.z())
    }

    #[inline]
    fn www(self) -> Self::Swizzle3 {
        Vector3::broadcast(self.w())
    }

    #[inline]
    fn xxxx(self) -> Self {
        Vector4::broadcast(self.x())
    }

    #[inline]
    fn xxxy(self) -> Self {
        let xxxy = std::simd::simd_swizzle!(self.value, [0, 0, 0, 1]);

        Vector4 { value: xxxy }
    }

    #[inline]
    fn xxxz(self) -> Self {
        let xxxz = std::simd::simd_swizzle!(self.value, [0, 0, 0, 2]);

        Vector4 { value: xxxz }
    }

    #[inline]
    fn xxxw(self) -> Self {
        let xxxw = std::simd::simd_swizzle!(self.value, [0, 0, 0, 3]);

        Vector4 { value: xxxw }
    }

    #[inline]
    fn xxyx(self) -> Self {
        let xxyx = std::simd::simd_swizzle!(self.value, [0, 0, 1, 0]);

        Vector4 { value: xxyx }
    }

    #[inline]
    fn xxyy(self) -> Self {
        let xxyy = std::simd::simd_swizzle!(self.value, [0, 0, 1, 1]);

        Vector4 { value: xxyy }
    }

    #[inline]
    fn xxyz(self) -> Self {
        let xxyz = std::simd::simd_swizzle!(self.value, [0, 0, 1, 2]);

        Vector4 { value: xxyz }
    }

    #[inline]
    fn xxyw(self) -> Self {
        let xxyw = std::simd::simd_swizzle!(self.value, [0, 0, 1, 3]);

        Vector4 { value: xxyw }
    }

    #[inline]
    fn xxzx(self) -> Self {
        let xxzx = std::simd::simd_swizzle!(self.value, [0, 0, 2, 0]);

        Vector4 { value: xxzx }
    }

    #[inline]
    fn xxzy(self) -> Self {
        let xxzy = std::simd::simd_swizzle!(self.value, [0, 0, 2, 1]);

        Vector4 { value: xxzy }
    }

    #[inline]
    fn xxzz(self) -> Self {
        let xxzz = std::simd::simd_swizzle!(self.value, [0, 0, 2, 2]);

        Vector4 { value: xxzz }
    }

    #[inline]
    fn xxzw(self) -> Self {
        let xxzw = std::simd::simd_swizzle!(self.value, [0, 0, 2, 3]);

        Vector4 { value: xxzw }
    }

    #[inline]
    fn xxwx(self) -> Self {
        let xxwx = std::simd::simd_swizzle!(self.value, [0, 0, 3, 0]);

        Vector4 { value: xxwx }
    }

    #[inline]
    fn xxwy(self) -> Self {
        let xxwy = std::simd::simd_swizzle!(self.value, [0, 0, 3, 1]);

        Vector4 { value: xxwy }
    }

    #[inline]
    fn xxwz(self) -> Self {
        let xxwz = std::simd::simd_swizzle!(self.value, [0, 0, 3, 2]);

        Vector4 { value: xxwz }
    }

    #[inline]
    fn xxww(self) -> Self {
        let xxww = std::simd::simd_swizzle!(self.value, [0, 0, 3, 3]);

        Vector4 { value: xxww }
    }

    #[inline]
    fn xyxx(self) -> Self {
        let xyxx = std::simd::simd_swizzle!(self.value, [0, 1, 0, 0]);

        Vector4 { value: xyxx }
    }

    #[inline]
    fn xyxy(self) -> Self {
        let xyxy = std::simd::simd_swizzle!(self.value, [0, 1, 0, 1]);

        Vector4 { value: xyxy }
    }

    #[inline]
    fn xyxz(self) -> Self {
        let xyxz = std::simd::simd_swizzle!(self.value, [0, 1, 0, 2]);

        Vector4 { value: xyxz }
    }

    #[inline]
    fn xyxw(self) -> Self {
        let xyxw = std::simd::simd_swizzle!(self.value, [0, 1, 0, 3]);

        Vector4 { value: xyxw }
    }

    #[inline]
    fn xyyx(self) -> Self {
        let xyyx = std::simd::simd_swizzle!(self.value, [0, 1, 1, 0]);

        Vector4 { value: xyyx }
    }

    #[inline]
    fn xyyy(self) -> Self {
        let xyyy = std::simd::simd_swizzle!(self.value, [0, 1, 1, 1]);

        Vector4 { value: xyyy }
    }

    #[inline]
    fn xyyz(self) -> Self {
        let xyyz = std::simd::simd_swizzle!(self.value, [0, 1, 1, 2]);

        Vector4 { value: xyyz }
    }

    #[inline]
    fn xyyw(self) -> Self {
        let xyyw = std::simd::simd_swizzle!(self.value, [0, 1, 1, 3]);

        Vector4 { value: xyyw }
    }

    #[inline]
    fn xyzx(self) -> Self {
        let xyzx = std::simd::simd_swizzle!(self.value, [0, 1, 2, 0]);

        Vector4 { value: xyzx }
    }

    #[inline]
    fn xyzy(self) -> Self {
        let xyzy = std::simd::simd_swizzle!(self.value, [0, 1, 2, 1]);

        Vector4 { value: xyzy }
    }

    #[inline]
    fn xyzz(self) -> Self {
        let xyzz = std::simd::simd_swizzle!(self.value, [0, 1, 2, 2]);

        Vector4 { value: xyzz }
    }

    #[inline]
    fn xywx(self) -> Self {
        let xywx = std::simd::simd_swizzle!(self.value, [0, 1, 3, 0]);

        Vector4 { value: xywx }
    }

    #[inline]
    fn xywy(self) -> Self {
        let xywy = std::simd::simd_swizzle!(self.value, [0, 1, 3, 1]);

        Vector4 { value: xywy }
    }

    #[inline]
    fn xywz(self) -> Self {
        let xywz = std::simd::simd_swizzle!(self.value, [0, 1, 3, 2]);

        Vector4 { value: xywz }
    }

    #[inline]
    fn xyww(self) -> Self {
        let xyww = std::simd::simd_swizzle!(self.value, [0, 1, 3, 3]);

        Vector4 { value: xyww }
    }

    #[inline]
    fn xzxx(self) -> Self {
        let xzxx = std::simd::simd_swizzle!(self.value, [0, 2, 0, 0]);

        Vector4 { value: xzxx }
    }

    #[inline]
    fn xzxy(self) -> Self {
        let xzxy = std::simd::simd_swizzle!(self.value, [0, 2, 0, 1]);

        Vector4 { value: xzxy }
    }

    #[inline]
    fn xzxz(self) -> Self {
        let xzxz = std::simd::simd_swizzle!(self.value, [0, 2, 0, 2]);

        Vector4 { value: xzxz }
    }

    #[inline]
    fn xzxw(self) -> Self {
        let xzxw = std::simd::simd_swizzle!(self.value, [0, 2, 0, 3]);

        Vector4 { value: xzxw }
    }

    #[inline]
    fn xzyx(self) -> Self {
        let xzyx = std::simd::simd_swizzle!(self.value, [0, 2, 1, 0]);

        Vector4 { value: xzyx }
    }

    #[inline]
    fn xzyy(self) -> Self {
        let xzyy = std::simd::simd_swizzle!(self.value, [0, 2, 1, 1]);

        Vector4 { value: xzyy }
    }

    #[inline]
    fn xzyz(self) -> Self {
        let xzyz = std::simd::simd_swizzle!(self.value, [0, 2, 1, 2]);

        Vector4 { value: xzyz }
    }

    #[inline]
    fn xzyw(self) -> Self {
        let xzyw = std::simd::simd_swizzle!(self.value, [0, 2, 1, 3]);

        Vector4 { value: xzyw }
    }

    #[inline]
    fn xzzx(self) -> Self {
        let xzzx = std::simd::simd_swizzle!(self.value, [0, 2, 2, 0]);

        Vector4 { value: xzzx }
    }

    #[inline]
    fn xzzy(self) -> Self {
        let xzzy = std::simd::simd_swizzle!(self.value, [0, 2, 2, 1]);

        Vector4 { value: xzzy }
    }

    #[inline]
    fn xzzz(self) -> Self {
        let xzzz = std::simd::simd_swizzle!(self.value, [0, 2, 2, 2]);

        Vector4 { value: xzzz }
    }

    #[inline]
    fn xzzw(self) -> Self {
        let xzzw = std::simd::simd_swizzle!(self.value, [0, 2, 2, 3]);

        Vector4 { value: xzzw }
    }

    #[inline]
    fn xzwx(self) -> Self {
        let xzwx = std::simd::simd_swizzle!(self.value, [0, 2, 3, 0]);

        Vector4 { value: xzwx }
    }

    #[inline]
    fn xzwy(self) -> Self {
        let xzwy = std::simd::simd_swizzle!(self.value, [0, 2, 3, 1]);

        Vector4 { value: xzwy }
    }

    #[inline]
    fn xzwz(self) -> Self {
        let xzwz = std::simd::simd_swizzle!(self.value, [0, 2, 3, 2]);

        Vector4 { value: xzwz }
    }

    #[inline]
    fn xzww(self) -> Self {
        let xzww = std::simd::simd_swizzle!(self.value, [0, 2, 3, 3]);

        Vector4 { value: xzww }
    }

    #[inline]
    fn xwxx(self) -> Self {
        let xwxx = std::simd::simd_swizzle!(self.value, [0, 3, 0, 0]);

        Vector4 { value: xwxx }
    }

    #[inline]
    fn xwxy(self) -> Self {
        let xwxy = std::simd::simd_swizzle!(self.value, [0, 3, 0, 1]);

        Vector4 { value: xwxy }
    }

    #[inline]
    fn xwxz(self) -> Self {
        let xwxz = std::simd::simd_swizzle!(self.value, [0, 3, 0, 2]);

        Vector4 { value: xwxz }
    }

    #[inline]
    fn xwxw(self) -> Self {
        let xwxw = std::simd::simd_swizzle!(self.value, [0, 3, 0, 3]);

        Vector4 { value: xwxw }
    }

    #[inline]
    fn xwyx(self) -> Self {
        let xwyx = std::simd::simd_swizzle!(self.value, [0, 3, 1, 0]);

        Vector4 { value: xwyx }
    }

    #[inline]
    fn xwyy(self) -> Self {
        let xwyy = std::simd::simd_swizzle!(self.value, [0, 3, 1, 1]);

        Vector4 { value: xwyy }
    }

    #[inline]
    fn xwyz(self) -> Self {
        let xwyz = std::simd::simd_swizzle!(self.value, [0, 3, 1, 2]);

        Vector4 { value: xwyz }
    }

    #[inline]
    fn xwyw(self) -> Self {
        let xwyw = std::simd::simd_swizzle!(self.value, [0, 3, 1, 3]);

        Vector4 { value: xwyw }
    }

    #[inline]
    fn xwzx(self) -> Self {
        let xwzx = std::simd::simd_swizzle!(self.value, [0, 3, 2, 0]);

        Vector4 { value: xwzx }
    }

    #[inline]
    fn xwzy(self) -> Self {
        let xwzy = std::simd::simd_swizzle!(self.value, [0, 3, 2, 1]);

        Vector4 { value: xwzy }
    }

    #[inline]
    fn xwzz(self) -> Self {
        let xwzz = std::simd::simd_swizzle!(self.value, [0, 3, 2, 2]);

        Vector4 { value: xwzz }
    }

    #[inline]
    fn xwzw(self) -> Self {
        let xwzw = std::simd::simd_swizzle!(self.value, [0, 3, 2, 3]);

        Vector4 { value: xwzw }
    }

    #[inline]
    fn xwwx(self) -> Self {
        let xwwx = std::simd::simd_swizzle!(self.value, [0, 3, 3, 0]);

        Vector4 { value: xwwx }
    }

    #[inline]
    fn xwwy(self) -> Self {
        let xwwy = std::simd::simd_swizzle!(self.value, [0, 3, 3, 1]);

        Vector4 { value: xwwy }
    }

    #[inline]
    fn xwwz(self) -> Self {
        let xwwz = std::simd::simd_swizzle!(self.value, [0, 3, 3, 2]);

        Vector4 { value: xwwz }
    }

    #[inline]
    fn xwww(self) -> Self {
        let xwww = std::simd::simd_swizzle!(self.value, [0, 3, 3, 3]);

        Vector4 { value: xwww }
    }

    #[inline]
    fn yxxx(self) -> Self {
        let yxxx = std::simd::simd_swizzle!(self.value, [1, 0, 0, 0]);

        Vector4 { value: yxxx }
    }

    #[inline]
    fn yxxy(self) -> Self {
        let yxxy = std::simd::simd_swizzle!(self.value, [1, 0, 0, 1]);

        Vector4 { value: yxxy }
    }

    #[inline]
    fn yxxz(self) -> Self {
        let yxxz = std::simd::simd_swizzle!(self.value, [1, 0, 0, 2]);

        Vector4 { value: yxxz }
    }

    #[inline]
    fn yxxw(self) -> Self {
        let yxxw = std::simd::simd_swizzle!(self.value, [1, 0, 0, 3]);

        Vector4 { value: yxxw }
    }

    #[inline]
    fn yxyx(self) -> Self {
        let yxyx = std::simd::simd_swizzle!(self.value, [1, 0, 1, 0]);

        Vector4 { value: yxyx }
    }

    #[inline]
    fn yxyy(self) -> Self {
        let yxyy = std::simd::simd_swizzle!(self.value, [1, 0, 1, 1]);

        Vector4 { value: yxyy }
    }

    #[inline]
    fn yxyz(self) -> Self {
        let yxyz = std::simd::simd_swizzle!(self.value, [1, 0, 1, 2]);

        Vector4 { value: yxyz }
    }

    #[inline]
    fn yxyw(self) -> Self {
        let yxyw = std::simd::simd_swizzle!(self.value, [1, 0, 1, 3]);

        Vector4 { value: yxyw }
    }

    #[inline]
    fn yxzx(self) -> Self {
        let yxzx = std::simd::simd_swizzle!(self.value, [1, 0, 2, 0]);

        Vector4 { value: yxzx }
    }

    #[inline]
    fn yxzy(self) -> Self {
        let yxzy = std::simd::simd_swizzle!(self.value, [1, 0, 2, 1]);

        Vector4 { value: yxzy }
    }

    #[inline]
    fn yxzz(self) -> Self {
        let yxzz = std::simd::simd_swizzle!(self.value, [1, 0, 2, 2]);

        Vector4 { value: yxzz }
    }

    #[inline]
    fn yxzw(self) -> Self {
        let yxzw = std::simd::simd_swizzle!(self.value, [1, 0, 2, 3]);

        Vector4 { value: yxzw }
    }

    #[inline]
    fn yxwx(self) -> Self {
        let yxwx = std::simd::simd_swizzle!(self.value, [1, 0, 3, 0]);

        Vector4 { value: yxwx }
    }

    #[inline]
    fn yxwy(self) -> Self {
        let yxwy = std::simd::simd_swizzle!(self.value, [1, 0, 3, 1]);

        Vector4 { value: yxwy }
    }

    #[inline]
    fn yxwz(self) -> Self {
        let yxwz = std::simd::simd_swizzle!(self.value, [1, 0, 3, 2]);

        Vector4 { value: yxwz }
    }

    #[inline]
    fn yxww(self) -> Self {
        let yxww = std::simd::simd_swizzle!(self.value, [1, 0, 3, 3]);

        Vector4 { value: yxww }
    }

    #[inline]
    fn yyxx(self) -> Self {
        let yyxx = std::simd::simd_swizzle!(self.value, [1, 1, 0, 0]);

        Vector4 { value: yyxx }
    }

    #[inline]
    fn yyxy(self) -> Self {
        let yyxy = std::simd::simd_swizzle!(self.value, [1, 1, 0, 1]);

        Vector4 { value: yyxy }
    }

    #[inline]
    fn yyxz(self) -> Self {
        let yyxz = std::simd::simd_swizzle!(self.value, [1, 1, 0, 2]);

        Vector4 { value: yyxz }
    }

    #[inline]
    fn yyxw(self) -> Self {
        let yyxw = std::simd::simd_swizzle!(self.value, [1, 1, 0, 3]);

        Vector4 { value: yyxw }
    }

    #[inline]
    fn yyyx(self) -> Self {
        let yyyx = std::simd::simd_swizzle!(self.value, [1, 1, 1, 0]);

        Vector4 { value: yyyx }
    }

    #[inline]
    fn yyyy(self) -> Self {
        Vector4::broadcast(self.y())
    }

    #[inline]
    fn yyyz(self) -> Self {
        let yyyz = std::simd::simd_swizzle!(self.value, [1, 1, 1, 2]);

        Vector4 { value: yyyz }
    }

    #[inline]
    fn yyyw(self) -> Self {
        let yyyw = std::simd::simd_swizzle!(self.value, [1, 1, 1, 3]);

        Vector4 { value: yyyw }
    }

    #[inline]
    fn yyzx(self) -> Self {
        let yyzx = std::simd::simd_swizzle!(self.value, [1, 1, 2, 0]);

        Vector4 { value: yyzx }
    }

    #[inline]
    fn yyzy(self) -> Self {
        let yyzy = std::simd::simd_swizzle!(self.value, [1, 1, 2, 1]);

        Vector4 { value: yyzy }
    }

    #[inline]
    fn yyzz(self) -> Self {
        let yyzz = std::simd::simd_swizzle!(self.value, [1, 1, 2, 2]);

        Vector4 { value: yyzz }
    }

    #[inline]
    fn yyzw(self) -> Self {
        let yyzw = std::simd::simd_swizzle!(self.value, [1, 1, 2, 3]);

        Vector4 { value: yyzw }
    }

    #[inline]
    fn yywx(self) -> Self {
        let yywx = std::simd::simd_swizzle!(self.value, [1, 1, 3, 0]);

        Vector4 { value: yywx }
    }

    #[inline]
    fn yywy(self) -> Self {
        let yywy = std::simd::simd_swizzle!(self.value, [1, 1, 3, 1]);

        Vector4 { value: yywy }
    }

    #[inline]
    fn yywz(self) -> Self {
        let yywz = std::simd::simd_swizzle!(self.value, [1, 1, 3, 2]);

        Vector4 { value: yywz }
    }

    #[inline]
    fn yyww(self) -> Self {
        let yyww = std::simd::simd_swizzle!(self.value, [1, 1, 3, 3]);

        Vector4 { value: yyww }
    }

    #[inline]
    fn yzxx(self) -> Self {
        let yzxx = std::simd::simd_swizzle!(self.value, [1, 2, 0, 0]);

        Vector4 { value: yzxx }
    }

    #[inline]
    fn yzxy(self) -> Self {
        let yzxy = std::simd::simd_swizzle!(self.value, [1, 2, 0, 1]);

        Vector4 { value: yzxy }
    }

    #[inline]
    fn yzxz(self) -> Self {
        let yzxz = std::simd::simd_swizzle!(self.value, [1, 2, 0, 2]);

        Vector4 { value: yzxz }
    }

    #[inline]
    fn yzxw(self) -> Self {
        let yzxw = std::simd::simd_swizzle!(self.value, [1, 2, 0, 3]);

        Vector4 { value: yzxw }
    }

    #[inline]
    fn yzyx(self) -> Self {
        let yzyx = std::simd::simd_swizzle!(self.value, [1, 2, 1, 0]);

        Vector4 { value: yzyx }
    }

    #[inline]
    fn yzyy(self) -> Self {
        let yzyy = std::simd::simd_swizzle!(self.value, [1, 2, 1, 1]);

        Vector4 { value: yzyy }
    }

    #[inline]
    fn yzyz(self) -> Self {
        let yzyz = std::simd::simd_swizzle!(self.value, [1, 2, 1, 2]);

        Vector4 { value: yzyz }
    }

    #[inline]
    fn yzyw(self) -> Self {
        let yzyw = std::simd::simd_swizzle!(self.value, [1, 2, 1, 3]);

        Vector4 { value: yzyw }
    }

    #[inline]
    fn yzzx(self) -> Self {
        let yzzx = std::simd::simd_swizzle!(self.value, [1, 2, 2, 0]);

        Vector4 { value: yzzx }
    }

    #[inline]
    fn yzzy(self) -> Self {
        let yzzy = std::simd::simd_swizzle!(self.value, [1, 2, 2, 1]);

        Vector4 { value: yzzy }
    }

    #[inline]
    fn yzzz(self) -> Self {
        let yzzz = std::simd::simd_swizzle!(self.value, [1, 2, 2, 2]);

        Vector4 { value: yzzz }
    }

    #[inline]
    fn yzzw(self) -> Self {
        let yzzw = std::simd::simd_swizzle!(self.value, [1, 2, 2, 3]);

        Vector4 { value: yzzw }
    }

    #[inline]
    fn yzwx(self) -> Self {
        let yzwx = std::simd::simd_swizzle!(self.value, [1, 2, 3, 0]);

        Vector4 { value: yzwx }
    }

    #[inline]
    fn yzwy(self) -> Self {
        let yzwy = std::simd::simd_swizzle!(self.value, [1, 2, 3, 1]);

        Vector4 { value: yzwy }
    }

    #[inline]
    fn yzwz(self) -> Self {
        let yzwz = std::simd::simd_swizzle!(self.value, [1, 2, 3, 2]);

        Vector4 { value: yzwz }
    }

    #[inline]
    fn yzww(self) -> Self {
        let yzww = std::simd::simd_swizzle!(self.value, [1, 2, 3, 3]);

        Vector4 { value: yzww }
    }

    #[inline]
    fn ywxx(self) -> Self {
        let ywxx = std::simd::simd_swizzle!(self.value, [1, 3, 0, 0]);

        Vector4 { value: ywxx }
    }

    #[inline]
    fn ywxy(self) -> Self {
        let ywxy = std::simd::simd_swizzle!(self.value, [1, 3, 0, 1]);

        Vector4 { value: ywxy }
    }

    #[inline]
    fn ywxz(self) -> Self {
        let ywxz = std::simd::simd_swizzle!(self.value, [1, 3, 0, 2]);

        Vector4 { value: ywxz }
    }

    #[inline]
    fn ywxw(self) -> Self {
        let ywxw = std::simd::simd_swizzle!(self.value, [1, 3, 0, 3]);

        Vector4 { value: ywxw }
    }

    #[inline]
    fn ywyx(self) -> Self {
        let ywyx = std::simd::simd_swizzle!(self.value, [1, 3, 1, 0]);

        Vector4 { value: ywyx }
    }

    #[inline]
    fn ywyy(self) -> Self {
        let ywyy = std::simd::simd_swizzle!(self.value, [1, 3, 1, 1]);

        Vector4 { value: ywyy }
    }

    #[inline]
    fn ywyz(self) -> Self {
        let ywyz = std::simd::simd_swizzle!(self.value, [1, 3, 1, 2]);

        Vector4 { value: ywyz }
    }

    #[inline]
    fn ywyw(self) -> Self {
        let ywyw = std::simd::simd_swizzle!(self.value, [1, 3, 1, 3]);

        Vector4 { value: ywyw }
    }

    #[inline]
    fn ywzx(self) -> Self {
        let ywzx = std::simd::simd_swizzle!(self.value, [1, 3, 2, 0]);

        Vector4 { value: ywzx }
    }

    #[inline]
    fn ywzy(self) -> Self {
        let ywzy = std::simd::simd_swizzle!(self.value, [1, 3, 2, 1]);

        Vector4 { value: ywzy }
    }

    #[inline]
    fn ywzz(self) -> Self {
        let ywzz = std::simd::simd_swizzle!(self.value, [1, 3, 2, 2]);

        Vector4 { value: ywzz }
    }

    #[inline]
    fn ywzw(self) -> Self {
        let ywzw = std::simd::simd_swizzle!(self.value, [1, 3, 2, 3]);

        Vector4 { value: ywzw }
    }

    #[inline]
    fn ywwx(self) -> Self {
        let ywwx = std::simd::simd_swizzle!(self.value, [1, 3, 3, 0]);

        Vector4 { value: ywwx }
    }

    #[inline]
    fn ywwy(self) -> Self {
        let ywwy = std::simd::simd_swizzle!(self.value, [1, 3, 3, 1]);

        Vector4 { value: ywwy }
    }

    #[inline]
    fn ywwz(self) -> Self {
        let ywwz = std::simd::simd_swizzle!(self.value, [1, 3, 3, 2]);

        Vector4 { value: ywwz }
    }

    #[inline]
    fn ywww(self) -> Self {
        let ywww = std::simd::simd_swizzle!(self.value, [1, 3, 3, 3]);

        Vector4 { value: ywww }
    }

    #[inline]
    fn zxxx(self) -> Self {
        let zxxx = std::simd::simd_swizzle!(self.value, [2, 0, 0, 0]);

        Vector4 { value: zxxx }
    }

    #[inline]
    fn zxxy(self) -> Self {
        let zxxy = std::simd::simd_swizzle!(self.value, [2, 0, 0, 1]);

        Vector4 { value: zxxy }
    }

    #[inline]
    fn zxxz(self) -> Self {
        let zxxz = std::simd::simd_swizzle!(self.value, [2, 0, 0, 2]);

        Vector4 { value: zxxz }
    }

    #[inline]
    fn zxxw(self) -> Self {
        let zxxw = std::simd::simd_swizzle!(self.value, [2, 0, 0, 3]);

        Vector4 { value: zxxw }
    }

    #[inline]
    fn zxyx(self) -> Self {
        let zxyx = std::simd::simd_swizzle!(self.value, [2, 0, 1, 0]);

        Vector4 { value: zxyx }
    }

    #[inline]
    fn zxyy(self) -> Self {
        let zxyy = std::simd::simd_swizzle!(self.value, [2, 0, 1, 1]);

        Vector4 { value: zxyy }
    }

    #[inline]
    fn zxyz(self) -> Self {
        let zxyz = std::simd::simd_swizzle!(self.value, [2, 0, 1, 2]);

        Vector4 { value: zxyz }
    }

    #[inline]
    fn zxyw(self) -> Self {
        let zxyw = std::simd::simd_swizzle!(self.value, [2, 0, 1, 3]);

        Vector4 { value: zxyw }
    }

    #[inline]
    fn zxzx(self) -> Self {
        let zxzx = std::simd::simd_swizzle!(self.value, [2, 0, 2, 0]);

        Vector4 { value: zxzx }
    }

    #[inline]
    fn zxzy(self) -> Self {
        let zxzy = std::simd::simd_swizzle!(self.value, [2, 0, 2, 1]);

        Vector4 { value: zxzy }
    }

    #[inline]
    fn zxzz(self) -> Self {
        let zxzz = std::simd::simd_swizzle!(self.value, [2, 0, 2, 2]);

        Vector4 { value: zxzz }
    }

    #[inline]
    fn zxzw(self) -> Self {
        let zxzw = std::simd::simd_swizzle!(self.value, [2, 0, 2, 3]);

        Vector4 { value: zxzw }
    }

    #[inline]
    fn zxwx(self) -> Self {
        let zxwx = std::simd::simd_swizzle!(self.value, [2, 0, 3, 0]);

        Vector4 { value: zxwx }
    }

    #[inline]
    fn zxwy(self) -> Self {
        let zxwy = std::simd::simd_swizzle!(self.value, [2, 0, 3, 1]);

        Vector4 { value: zxwy }
    }

    #[inline]
    fn zxwz(self) -> Self {
        let zxwz = std::simd::simd_swizzle!(self.value, [2, 0, 3, 2]);

        Vector4 { value: zxwz }
    }

    #[inline]
    fn zxww(self) -> Self {
        let zxww = std::simd::simd_swizzle!(self.value, [2, 0, 3, 3]);

        Vector4 { value: zxww }
    }

    #[inline]
    fn zyxx(self) -> Self {
        let zyxx = std::simd::simd_swizzle!(self.value, [2, 1, 0, 0]);

        Vector4 { value: zyxx }
    }

    #[inline]
    fn zyxy(self) -> Self {
        let zyxy = std::simd::simd_swizzle!(self.value, [2, 1, 0, 1]);

        Vector4 { value: zyxy }
    }

    #[inline]
    fn zyxz(self) -> Self {
        let zyxz = std::simd::simd_swizzle!(self.value, [2, 1, 0, 2]);

        Vector4 { value: zyxz }
    }

    #[inline]
    fn zyxw(self) -> Self {
        let zyxw = std::simd::simd_swizzle!(self.value, [2, 1, 0, 3]);

        Vector4 { value: zyxw }
    }

    #[inline]
    fn zyyx(self) -> Self {
        let zyyx = std::simd::simd_swizzle!(self.value, [2, 1, 1, 0]);

        Vector4 { value: zyyx }
    }

    #[inline]
    fn zyyy(self) -> Self {
        let zyyy = std::simd::simd_swizzle!(self.value, [2, 1, 1, 1]);

        Vector4 { value: zyyy }
    }

    #[inline]
    fn zyyz(self) -> Self {
        let zyyz = std::simd::simd_swizzle!(self.value, [2, 1, 1, 2]);

        Vector4 { value: zyyz }
    }

    #[inline]
    fn zyyw(self) -> Self {
        let zyyw = std::simd::simd_swizzle!(self.value, [2, 1, 1, 3]);

        Vector4 { value: zyyw }
    }

    #[inline]
    fn zyzx(self) -> Self {
        let zyzx = std::simd::simd_swizzle!(self.value, [2, 1, 2, 0]);

        Vector4 { value: zyzx }
    }

    #[inline]
    fn zyzy(self) -> Self {
        let zyzy = std::simd::simd_swizzle!(self.value, [2, 1, 2, 1]);

        Vector4 { value: zyzy }
    }

    #[inline]
    fn zyzz(self) -> Self {
        let zyzz = std::simd::simd_swizzle!(self.value, [2, 1, 2, 2]);

        Vector4 { value: zyzz }
    }

    #[inline]
    fn zyzw(self) -> Self {
        let zyzw = std::simd::simd_swizzle!(self.value, [2, 1, 2, 3]);

        Vector4 { value: zyzw }
    }

    #[inline]
    fn zywx(self) -> Self {
        let zywx = std::simd::simd_swizzle!(self.value, [2, 1, 3, 0]);

        Vector4 { value: zywx }
    }

    #[inline]
    fn zywy(self) -> Self {
        let zywy = std::simd::simd_swizzle!(self.value, [2, 1, 3, 1]);

        Vector4 { value: zywy }
    }

    #[inline]
    fn zywz(self) -> Self {
        let zywz = std::simd::simd_swizzle!(self.value, [2, 1, 3, 2]);

        Vector4 { value: zywz }
    }

    #[inline]
    fn zyww(self) -> Self {
        let zyww = std::simd::simd_swizzle!(self.value, [2, 1, 3, 3]);

        Vector4 { value: zyww }
    }

    #[inline]
    fn zzxx(self) -> Self {
        let zzxx = std::simd::simd_swizzle!(self.value, [2, 2, 0, 0]);

        Vector4 { value: zzxx }
    }

    #[inline]
    fn zzxy(self) -> Self {
        let zzxy = std::simd::simd_swizzle!(self.value, [2, 2, 0, 1]);

        Vector4 { value: zzxy }
    }

    #[inline]
    fn zzxz(self) -> Self {
        let zzxz = std::simd::simd_swizzle!(self.value, [2, 2, 0, 2]);

        Vector4 { value: zzxz }
    }

    #[inline]
    fn zzxw(self) -> Self {
        let zzxw = std::simd::simd_swizzle!(self.value, [2, 2, 0, 3]);

        Vector4 { value: zzxw }
    }

    #[inline]
    fn zzyx(self) -> Self {
        let zzyx = std::simd::simd_swizzle!(self.value, [2, 2, 1, 0]);

        Vector4 { value: zzyx }
    }

    #[inline]
    fn zzyy(self) -> Self {
        let zzyy = std::simd::simd_swizzle!(self.value, [2, 2, 1, 1]);

        Vector4 { value: zzyy }
    }

    #[inline]
    fn zzyz(self) -> Self {
        let zzyz = std::simd::simd_swizzle!(self.value, [2, 2, 1, 2]);

        Vector4 { value: zzyz }
    }

    #[inline]
    fn zzyw(self) -> Self {
        let zzyw = std::simd::simd_swizzle!(self.value, [2, 2, 1, 3]);

        Vector4 { value: zzyw }
    }

    #[inline]
    fn zzzx(self) -> Self {
        let zzzx = std::simd::simd_swizzle!(self.value, [2, 2, 2, 0]);

        Vector4 { value: zzzx }
    }

    #[inline]
    fn zzzy(self) -> Self {
        let zzzy = std::simd::simd_swizzle!(self.value, [2, 2, 2, 1]);

        Vector4 { value: zzzy }
    }

    #[inline]
    fn zzzz(self) -> Self {
        Vector4::broadcast(self.z())
    }

    #[inline]
    fn zzzw(self) -> Self {
        let zzzw = std::simd::simd_swizzle!(self.value, [2, 2, 2, 3]);

        Vector4 { value: zzzw }
    }

    #[inline]
    fn zzwx(self) -> Self {
        let zzwx = std::simd::simd_swizzle!(self.value, [2, 2, 3, 0]);

        Vector4 { value: zzwx }
    }

    #[inline]
    fn zzwy(self) -> Self {
        let zzwy = std::simd::simd_swizzle!(self.value, [2, 2, 3, 1]);

        Vector4 { value: zzwy }
    }

    #[inline]
    fn zzwz(self) -> Self {
        let zzwz = std::simd::simd_swizzle!(self.value, [2, 2, 3, 2]);

        Vector4 { value: zzwz }
    }

    #[inline]
    fn zzww(self) -> Self {
        let zzww = std::simd::simd_swizzle!(self.value, [2, 2, 3, 3]);

        Vector4 { value: zzww }
    }

    #[inline]
    fn zwxx(self) -> Self {
        let zwxx = std::simd::simd_swizzle!(self.value, [2, 3, 0, 0]);

        Vector4 { value: zwxx }
    }

    #[inline]
    fn zwxy(self) -> Self {
        let zwxy = std::simd::simd_swizzle!(self.value, [2, 3, 0, 1]);

        Vector4 { value: zwxy }
    }

    #[inline]
    fn zwxz(self) -> Self {
        let zwxz = std::simd::simd_swizzle!(self.value, [2, 3, 0, 2]);

        Vector4 { value: zwxz }
    }

    #[inline]
    fn zwxw(self) -> Self {
        let zwxw = std::simd::simd_swizzle!(self.value, [2, 3, 0, 3]);

        Vector4 { value: zwxw }
    }

    #[inline]
    fn zwyx(self) -> Self {
        let zwyx = std::simd::simd_swizzle!(self.value, [2, 3, 1, 0]);

        Vector4 { value: zwyx }
    }

    #[inline]
    fn zwyy(self) -> Self {
        let zwyy = std::simd::simd_swizzle!(self.value, [2, 3, 1, 1]);

        Vector4 { value: zwyy }
    }

    #[inline]
    fn zwyz(self) -> Self {
        let zwyz = std::simd::simd_swizzle!(self.value, [2, 3, 1, 2]);

        Vector4 { value: zwyz }
    }

    #[inline]
    fn zwyw(self) -> Self {
        let zwyw = std::simd::simd_swizzle!(self.value, [2, 3, 1, 3]);

        Vector4 { value: zwyw }
    }

    #[inline]
    fn zwzx(self) -> Self {
        let zwzx = std::simd::simd_swizzle!(self.value, [2, 3, 2, 0]);

        Vector4 { value: zwzx }
    }

    #[inline]
    fn zwzy(self) -> Self {
        let zwzy = std::simd::simd_swizzle!(self.value, [2, 3, 2, 1]);

        Vector4 { value: zwzy }
    }

    #[inline]
    fn zwzz(self) -> Self {
        let zwzz = std::simd::simd_swizzle!(self.value, [2, 3, 2, 2]);

        Vector4 { value: zwzz }
    }

    #[inline]
    fn zwzw(self) -> Self {
        let zwzw = std::simd::simd_swizzle!(self.value, [2, 3, 2, 3]);

        Vector4 { value: zwzw }
    }

    #[inline]
    fn zwwx(self) -> Self {
        let zwwx = std::simd::simd_swizzle!(self.value, [2, 3, 3, 0]);

        Vector4 { value: zwwx }
    }

    #[inline]
    fn zwwy(self) -> Self {
        let zwwy = std::simd::simd_swizzle!(self.value, [2, 3, 3, 1]);

        Vector4 { value: zwwy }
    }

    #[inline]
    fn zwwz(self) -> Self {
        let zwwz = std::simd::simd_swizzle!(self.value, [2, 3, 3, 2]);

        Vector4 { value: zwwz }
    }

    #[inline]
    fn zwww(self) -> Self {
        let zwww = std::simd::simd_swizzle!(self.value, [2, 3, 3, 3]);

        Vector4 { value: zwww }
    }

    #[inline]
    fn wxxx(self) -> Self {
        let wxxx = std::simd::simd_swizzle!(self.value, [3, 0, 0, 0]);

        Vector4 { value: wxxx }
    }

    #[inline]
    fn wxxy(self) -> Self {
        let wxxy = std::simd::simd_swizzle!(self.value, [3, 0, 0, 1]);

        Vector4 { value: wxxy }
    }

    #[inline]
    fn wxxz(self) -> Self {
        let wxxz = std::simd::simd_swizzle!(self.value, [3, 0, 0, 2]);

        Vector4 { value: wxxz }
    }

    #[inline]
    fn wxxw(self) -> Self {
        let wxxw = std::simd::simd_swizzle!(self.value, [3, 0, 0, 3]);

        Vector4 { value: wxxw }
    }

    #[inline]
    fn wxyx(self) -> Self {
        let wxyx = std::simd::simd_swizzle!(self.value, [3, 0, 1, 0]);

        Vector4 { value: wxyx }
    }

    #[inline]
    fn wxyy(self) -> Self {
        let wxyy = std::simd::simd_swizzle!(self.value, [3, 0, 1, 1]);

        Vector4 { value: wxyy }
    }

    #[inline]
    fn wxyz(self) -> Self {
        let wxyz = std::simd::simd_swizzle!(self.value, [3, 0, 1, 2]);

        Vector4 { value: wxyz }
    }

    #[inline]
    fn wxyw(self) -> Self {
        let wxyw = std::simd::simd_swizzle!(self.value, [3, 0, 1, 3]);

        Vector4 { value: wxyw }
    }

    #[inline]
    fn wxzx(self) -> Self {
        let wxzx = std::simd::simd_swizzle!(self.value, [3, 0, 2, 0]);

        Vector4 { value: wxzx }
    }

    #[inline]
    fn wxzy(self) -> Self {
        let wxzy = std::simd::simd_swizzle!(self.value, [3, 0, 2, 1]);

        Vector4 { value: wxzy }
    }

    #[inline]
    fn wxzz(self) -> Self {
        let wxzz = std::simd::simd_swizzle!(self.value, [3, 0, 2, 2]);

        Vector4 { value: wxzz }
    }

    #[inline]
    fn wxzw(self) -> Self {
        let wxzw = std::simd::simd_swizzle!(self.value, [3, 0, 2, 3]);

        Vector4 { value: wxzw }
    }

    #[inline]
    fn wxwx(self) -> Self {
        let wxwx = std::simd::simd_swizzle!(self.value, [3, 0, 3, 0]);

        Vector4 { value: wxwx }
    }

    #[inline]
    fn wxwy(self) -> Self {
        let wxwy = std::simd::simd_swizzle!(self.value, [3, 0, 3, 1]);

        Vector4 { value: wxwy }
    }

    #[inline]
    fn wxwz(self) -> Self {
        let wxwz = std::simd::simd_swizzle!(self.value, [3, 0, 3, 2]);

        Vector4 { value: wxwz }
    }

    #[inline]
    fn wxww(self) -> Self {
        let wxww = std::simd::simd_swizzle!(self.value, [3, 0, 3, 3]);

        Vector4 { value: wxww }
    }

    #[inline]
    fn wyxx(self) -> Self {
        let wyxx = std::simd::simd_swizzle!(self.value, [3, 1, 0, 0]);

        Vector4 { value: wyxx }
    }

    #[inline]
    fn wyxy(self) -> Self {
        let wyxy = std::simd::simd_swizzle!(self.value, [3, 1, 0, 1]);

        Vector4 { value: wyxy }
    }

    #[inline]
    fn wyxz(self) -> Self {
        let wyxz = std::simd::simd_swizzle!(self.value, [3, 1, 0, 2]);

        Vector4 { value: wyxz }
    }

    #[inline]
    fn wyxw(self) -> Self {
        let wyxw = std::simd::simd_swizzle!(self.value, [3, 1, 0, 3]);

        Vector4 { value: wyxw }
    }

    #[inline]
    fn wyyx(self) -> Self {
        let wyyx = std::simd::simd_swizzle!(self.value, [3, 1, 1, 0]);

        Vector4 { value: wyyx }
    }

    #[inline]
    fn wyyy(self) -> Self {
        let wyyy = std::simd::simd_swizzle!(self.value, [3, 1, 1, 1]);

        Vector4 { value: wyyy }
    }

    #[inline]
    fn wyyz(self) -> Self {
        let wyyz = std::simd::simd_swizzle!(self.value, [3, 1, 1, 2]);

        Vector4 { value: wyyz }
    }

    #[inline]
    fn wyyw(self) -> Self {
        let wyyw = std::simd::simd_swizzle!(self.value, [3, 1, 1, 3]);

        Vector4 { value: wyyw }
    }

    #[inline]
    fn wyzx(self) -> Self {
        let wyzx = std::simd::simd_swizzle!(self.value, [3, 1, 2, 0]);

        Vector4 { value: wyzx }
    }

    #[inline]
    fn wyzy(self) -> Self {
        let wyzy = std::simd::simd_swizzle!(self.value, [3, 1, 2, 1]);

        Vector4 { value: wyzy }
    }

    #[inline]
    fn wyzz(self) -> Self {
        let wyzz = std::simd::simd_swizzle!(self.value, [3, 1, 2, 2]);

        Vector4 { value: wyzz }
    }

    #[inline]
    fn wyzw(self) -> Self {
        let wyzw = std::simd::simd_swizzle!(self.value, [3, 1, 2, 3]);

        Vector4 { value: wyzw }
    }

    #[inline]
    fn wywx(self) -> Self {
        let wywx = std::simd::simd_swizzle!(self.value, [3, 1, 3, 0]);

        Vector4 { value: wywx }
    }

    #[inline]
    fn wywy(self) -> Self {
        let wywy = std::simd::simd_swizzle!(self.value, [3, 1, 3, 1]);

        Vector4 { value: wywy }
    }

    #[inline]
    fn wywz(self) -> Self {
        let wywz = std::simd::simd_swizzle!(self.value, [3, 1, 3, 2]);

        Vector4 { value: wywz }
    }

    #[inline]
    fn wyww(self) -> Self {
        let wyww = std::simd::simd_swizzle!(self.value, [3, 1, 3, 3]);

        Vector4 { value: wyww }
    }

    #[inline]
    fn wzxx(self) -> Self {
        let wzxx = std::simd::simd_swizzle!(self.value, [3, 2, 0, 0]);

        Vector4 { value: wzxx }
    }

    #[inline]
    fn wzxy(self) -> Self {
        let wzxy = std::simd::simd_swizzle!(self.value, [3, 2, 0, 1]);

        Vector4 { value: wzxy }
    }

    #[inline]
    fn wzxz(self) -> Self {
        let wzxz = std::simd::simd_swizzle!(self.value, [3, 2, 0, 2]);

        Vector4 { value: wzxz }
    }

    #[inline]
    fn wzxw(self) -> Self {
        let wzxw = std::simd::simd_swizzle!(self.value, [3, 2, 0, 3]);

        Vector4 { value: wzxw }
    }

    #[inline]
    fn wzyx(self) -> Self {
        let wzyx = std::simd::simd_swizzle!(self.value, [3, 2, 1, 0]);

        Vector4 { value: wzyx }
    }

    #[inline]
    fn wzyy(self) -> Self {
        let wzyy = std::simd::simd_swizzle!(self.value, [3, 2, 1, 1]);

        Vector4 { value: wzyy }
    }

    #[inline]
    fn wzyz(self) -> Self {
        let wzyz = std::simd::simd_swizzle!(self.value, [3, 2, 1, 2]);

        Vector4 { value: wzyz }
    }

    #[inline]
    fn wzyw(self) -> Self {
        let wzyw = std::simd::simd_swizzle!(self.value, [3, 2, 1, 3]);

        Vector4 { value: wzyw }
    }

    #[inline]
    fn wzzx(self) -> Self {
        let wzzx = std::simd::simd_swizzle!(self.value, [3, 2, 2, 0]);

        Vector4 { value: wzzx }
    }

    #[inline]
    fn wzzy(self) -> Self {
        let wzzy = std::simd::simd_swizzle!(self.value, [3, 2, 2, 1]);

        Vector4 { value: wzzy }
    }

    #[inline]
    fn wzzz(self) -> Self {
        let wzzz = std::simd::simd_swizzle!(self.value, [3, 2, 2, 2]);

        Vector4 { value: wzzz }
    }

    #[inline]
    fn wzzw(self) -> Self {
        let wzzw = std::simd::simd_swizzle!(self.value, [3, 2, 2, 3]);

        Vector4 { value: wzzw }
    }

    #[inline]
    fn wzwx(self) -> Self {
        let wzwx = std::simd::simd_swizzle!(self.value, [3, 2, 3, 0]);

        Vector4 { value: wzwx }
    }

    #[inline]
    fn wzwy(self) -> Self {
        let wzwy = std::simd::simd_swizzle!(self.value, [3, 2, 3, 1]);

        Vector4 { value: wzwy }
    }

    #[inline]
    fn wzwz(self) -> Self {
        let wzwz = std::simd::simd_swizzle!(self.value, [3, 2, 3, 2]);

        Vector4 { value: wzwz }
    }

    #[inline]
    fn wzww(self) -> Self {
        let wzww = std::simd::simd_swizzle!(self.value, [3, 2, 3, 3]);

        Vector4 { value: wzww }
    }

    #[inline]
    fn wwxx(self) -> Self {
        let wwxx = std::simd::simd_swizzle!(self.value, [3, 3, 0, 0]);

        Vector4 { value: wwxx }
    }

    #[inline]
    fn wwxy(self) -> Self {
        let wwxy = std::simd::simd_swizzle!(self.value, [3, 3, 0, 1]);

        Vector4 { value: wwxy }
    }

    #[inline]
    fn wwxz(self) -> Self {
        let wwxz = std::simd::simd_swizzle!(self.value, [3, 3, 0, 2]);

        Vector4 { value: wwxz }
    }

    #[inline]
    fn wwxw(self) -> Self {
        let wwxw = std::simd::simd_swizzle!(self.value, [3, 3, 0, 3]);

        Vector4 { value: wwxw }
    }

    #[inline]
    fn wwyx(self) -> Self {
        let wwyx = std::simd::simd_swizzle!(self.value, [3, 3, 1, 0]);

        Vector4 { value: wwyx }
    }

    #[inline]
    fn wwyy(self) -> Self {
        let wwyy = std::simd::simd_swizzle!(self.value, [3, 3, 1, 1]);

        Vector4 { value: wwyy }
    }

    #[inline]
    fn wwyz(self) -> Self {
        let wwyz = std::simd::simd_swizzle!(self.value, [3, 3, 1, 2]);

        Vector4 { value: wwyz }
    }

    #[inline]
    fn wwyw(self) -> Self {
        let wwyw = std::simd::simd_swizzle!(self.value, [3, 3, 1, 3]);

        Vector4 { value: wwyw }
    }

    #[inline]
    fn wwzx(self) -> Self {
        let wwzx = std::simd::simd_swizzle!(self.value, [3, 3, 2, 0]);

        Vector4 { value: wwzx }
    }

    #[inline]
    fn wwzy(self) -> Self {
        let wwzy = std::simd::simd_swizzle!(self.value, [3, 3, 2, 1]);

        Vector4 { value: wwzy }
    }

    #[inline]
    fn wwzz(self) -> Self {
        let wwzz = std::simd::simd_swizzle!(self.value, [3, 3, 2, 2]);

        Vector4 { value: wwzz }
    }

    #[inline]
    fn wwzw(self) -> Self {
        let wwzw = std::simd::simd_swizzle!(self.value, [3, 3, 2, 3]);

        Vector4 { value: wwzw }
    }

    #[inline]
    fn wwwx(self) -> Self {
        let wwwx = std::simd::simd_swizzle!(self.value, [3, 3, 3, 0]);

        Vector4 { value: wwwx }
    }

    #[inline]
    fn wwwy(self) -> Self {
        let wwwy = std::simd::simd_swizzle!(self.value, [3, 3, 3, 2]);

        Vector4 { value: wwwy }
    }

    #[inline]
    fn wwwz(self) -> Self {
        let wwwz = std::simd::simd_swizzle!(self.value, [3, 3, 3, 2]);

        Vector4 { value: wwwz }
    }

    #[inline]
    fn wwww(self) -> Self {
        Vector4::broadcast(self.w())
    }
}
