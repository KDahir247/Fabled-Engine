use crate::{Matrix3x3, Quaternion, Vector2, Vector4};

use crate::math_trait::Swizzles3;

use crate::matrix3x3_math::transpose_mat3;
use crate::vector_math::{component_sum, cross};

use std::fmt::{Display, Formatter};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub value: std::simd::f32x4,
}

impl Default for Vector3 {
    fn default() -> Self {
        Vector3::ZERO
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

    pub const NEG_ONE: Vector3 = Vector3 {
        value: std::simd::f32x4::from_array([-1.0, -1.0, -1.0, 0.0]),
    };

    pub const ZERO: Vector3 = Vector3 {
        value: std::simd::f32x4::from_array([0.0, 0.0, 0.0, 0.0]),
    };


    #[inline(always)]
    pub const fn set(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 {
            value: std::simd::f32x4::from_array([x, y, z, 0.0]),
        }
    }

    #[inline(always)]
    pub const fn broadcast(val: f32) -> Vector3 {
        Vector3 {
            value: std::simd::f32x4::from_array([val, val, val, 0.0]),
        }
    }

    #[inline(always)]
    pub const fn x(self) -> f32 {
        let array_vec3: [f32; 4] = self.value.to_array();

        array_vec3[0]
    }

    #[inline(always)]
    pub const fn y(self) -> f32 {
        let array_vec3: [f32; 4] = self.value.to_array();

        array_vec3[1]
    }

    #[inline(always)]
    pub const fn z(self) -> f32 {
        let array_vec3: [f32; 4] = self.value.to_array();

        array_vec3[2]
    }

    #[inline]
    pub fn trunc_vec2(self) -> Vector2 {
        let a = self.value.to_array();

        Vector2 {
            value: [a[0], a[1]],
        }
    }

    #[inline]
    pub const fn to_primitive(self) -> [f32; 3] {
        [self.x(), self.y(), self.z()]
    }

    #[inline]
    pub const fn from_primitive(array: [f32; 3]) -> Vector3 {
        Vector3::set(array[0], array[1], array[2])
    }
}

impl Display for Vector3 {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            _f,
            "Vector3 (x : {}, y : {}, z : {})",
            self.x(),
            self.y(),
            self.z()
        )
    }
}

impl Mul<Quaternion> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: Quaternion) -> Self::Output {
        let quaternion_real: f32 = rhs.to_real();

        let quaternion_real_vector: Vector3 = Vector3::broadcast(quaternion_real);

        let t = cross(rhs.value, self.value);

        let lhs: Vector3 = Vector3 { value: t + t };

        let intermediate: Vector3 = Vector3 {
            value: cross(rhs.value, lhs.value),
        };

        (quaternion_real_vector * lhs) + (intermediate + self)
    }
}

impl MulAssign<Quaternion> for Vector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Quaternion) {
        self.value = (*self * rhs).value;
    }
}

impl Mul<Vector3> for Matrix3x3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        let transpose_matrix: Matrix3x3 = transpose_mat3(self);

        let transpose_x: Vector3 = transpose_matrix.column_x * rhs;
        let transpose_y: Vector3 = transpose_matrix.column_y * rhs;
        let transpose_z: Vector3 = transpose_matrix.column_z * rhs;

        Vector3::set(
            component_sum(transpose_x.value),
            component_sum(transpose_y.value),
            component_sum(transpose_z.value),
        )
    }
}

impl MulAssign<Matrix3x3> for Vector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Matrix3x3) {
        self.value = (rhs * *self).value;
    }
}

// Component-Wise
impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            value: self.value * rhs.value,
        }
    }
}

impl MulAssign<Vector3> for Vector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Vector3) {
        self.value *= rhs.value;
    }
}

impl Div<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn div(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            value: self.value / rhs.value,
        }
    }
}

impl DivAssign<Vector3> for Vector3 {
    #[inline]
    fn div_assign(&mut self, rhs: Vector3) {
        self.value /= rhs.value;
    }
}

impl Rem<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn rem(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            value: self.value % rhs.value,
        }
    }
}

impl RemAssign<Vector3> for Vector3 {
    #[inline]
    fn rem_assign(&mut self, rhs: Vector3) {
        self.value %= rhs.value;
    }
}

impl Add<f32> for Vector3 {
    type Output = Vector3;


    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector3 {
            value: self.value + splat_f32x4,
        }
    }
}

impl AddAssign<f32> for Vector3 {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value += splat_f32x4;
    }
}

impl Sub<f32> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector3 {
            value: self.value - splat_f32x4,
        }
    }
}

impl SubAssign<f32> for Vector3 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value -= splat_f32x4;
    }
}

// Vector-Wise
impl Mul<f32> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector3 {
            value: self.value * splat_f32x4,
        }
    }
}

impl MulAssign<f32> for Vector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value *= splat_f32x4;
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector3 {
            value: self.value / splat_f32x4,
        }
    }
}

impl DivAssign<f32> for Vector3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value /= splat_f32x4;
    }
}

impl Rem<f32> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn rem(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Vector3 {
            value: self.value % splat_f32x4,
        }
    }
}

impl RemAssign<f32> for Vector3 {
    #[inline]
    fn rem_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value %= splat_f32x4;
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    #[inline]
    fn neg(self) -> Self::Output {
        Vector3 {
            value: self.value.neg(),
        }
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign<Vector3> for Vector3 {
    #[inline]
    fn add_assign(&mut self, rhs: Vector3) {
        self.value += rhs.value;
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    #[inline]
    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            value: self.value - rhs.value,
        }
    }
}

impl SubAssign<Vector3> for Vector3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vector3) {
        self.value -= rhs.value;
    }
}

impl Swizzles3 for Vector3 {
    type Swizzle2 = Vector2;
    type Swizzle4 = Vector4;

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
    fn xxx(self) -> Self {
        let xxx = std::simd::simd_swizzle!(self.value, [0, 0, 0, 0]);

        Vector3 { value: xxx }
    }

    #[inline]
    fn xxy(self) -> Self {
        let xxy_swizzle = std::simd::simd_swizzle!(self.value, [0, 0, 1, 3]);

        Vector3 { value: xxy_swizzle }
    }

    #[inline]
    fn xxz(self) -> Self {
        let xxz = std::simd::simd_swizzle!(self.value, [0, 0, 2, 3]);

        Vector3 { value: xxz }
    }

    #[inline]
    fn xyx(self) -> Self {
        let xyx = std::simd::simd_swizzle!(self.value, [0, 1, 0, 3]);

        Vector3 { value: xyx }
    }

    #[inline]
    fn xyy(self) -> Self {
        let xyy = std::simd::simd_swizzle!(self.value, [0, 1, 1, 3]);

        Vector3 { value: xyy }
    }

    #[inline]
    fn xzx(self) -> Self {
        let xzx = std::simd::simd_swizzle!(self.value, [0, 2, 0, 3]);

        Vector3 { value: xzx }
    }

    #[inline]
    fn xzy(self) -> Self {
        let xzy = std::simd::simd_swizzle!(self.value, [0, 2, 1, 3]);

        Vector3 { value: xzy }
    }

    #[inline]
    fn xzz(self) -> Self {
        let xzz = std::simd::simd_swizzle!(self.value, [0, 2, 2, 3]);

        Vector3 { value: xzz }
    }

    #[inline]
    fn yxx(self) -> Self {
        let yxx = std::simd::simd_swizzle!(self.value, [1, 0, 0, 3]);

        Vector3 { value: yxx }
    }

    #[inline]
    fn yxy(self) -> Self {
        let yxy = std::simd::simd_swizzle!(self.value, [1, 0, 1, 3]);

        Vector3 { value: yxy }
    }

    #[inline]
    fn yxz(self) -> Self {
        let yxz = std::simd::simd_swizzle!(self.value, [1, 0, 2, 3]);

        Vector3 { value: yxz }
    }

    #[inline]
    fn yyx(self) -> Self {
        let yyx = std::simd::simd_swizzle!(self.value, [1, 1, 0, 3]);

        Vector3 { value: yyx }
    }

    #[inline]
    fn yyy(self) -> Self {
        let yyy = std::simd::simd_swizzle!(self.value, [1, 1, 1, 3]);

        Vector3 { value: yyy }
    }

    #[inline]
    fn yyz(self) -> Self {
        let yyz = std::simd::simd_swizzle!(self.value, [1, 1, 2, 3]);

        Vector3 { value: yyz }
    }

    #[inline]
    fn yzx(self) -> Self {
        let yzx = std::simd::simd_swizzle!(self.value, [1, 2, 0, 3]);

        Vector3 { value: yzx }
    }

    #[inline]
    fn yzy(self) -> Self {
        let yzy = std::simd::simd_swizzle!(self.value, [1, 2, 1, 3]);

        Vector3 { value: yzy }
    }

    #[inline]
    fn yzz(self) -> Self {
        let yzz = std::simd::simd_swizzle!(self.value, [1, 2, 2, 3]);

        Vector3 { value: yzz }
    }

    #[inline]
    fn zxx(self) -> Self {
        let zxx = std::simd::simd_swizzle!(self.value, [2, 0, 0, 3]);

        Vector3 { value: zxx }
    }

    #[inline]
    fn zxy(self) -> Self {
        let zxy = std::simd::simd_swizzle!(self.value, [2, 0, 1, 3]);

        Vector3 { value: zxy }
    }

    #[inline]
    fn zxz(self) -> Self {
        let zxz = std::simd::simd_swizzle!(self.value, [2, 0, 2, 3]);

        Vector3 { value: zxz }
    }

    #[inline]
    fn zyx(self) -> Self {
        let zyx = std::simd::simd_swizzle!(self.value, [2, 1, 0, 3]);

        Vector3 { value: zyx }
    }

    #[inline]
    fn zyy(self) -> Self {
        let zyy = std::simd::simd_swizzle!(self.value, [2, 1, 1, 3]);

        Vector3 { value: zyy }
    }

    #[inline]
    fn zyz(self) -> Self {
        let zyz = std::simd::simd_swizzle!(self.value, [2, 1, 2, 3]);

        Vector3 { value: zyz }
    }

    #[inline]
    fn zzx(self) -> Self {
        let zzx = std::simd::simd_swizzle!(self.value, [2, 2, 0, 3]);

        Vector3 { value: zzx }
    }

    #[inline]
    fn zzy(self) -> Self {
        let zzy = std::simd::simd_swizzle!(self.value, [2, 2, 1, 3]);

        Vector3 { value: zzy }
    }

    #[inline]
    fn zzz(self) -> Self {
        let zzz = std::simd::simd_swizzle!(self.value, [2, 2, 2, 3]);

        Vector3 { value: zzz }
    }

    #[inline]
    fn xxxx(self) -> Self::Swizzle4 {
        let xxxx = std::simd::simd_swizzle!(self.value, [0, 0, 0, 0]);

        Vector4 { value: xxxx }
    }

    #[inline]
    fn xxxy(self) -> Self::Swizzle4 {
        let xxxy = std::simd::simd_swizzle!(self.value, [0, 0, 0, 1]);

        Vector4 { value: xxxy }
    }

    #[inline]
    fn xxxz(self) -> Self::Swizzle4 {
        let xxxz = std::simd::simd_swizzle!(self.value, [0, 0, 0, 2]);

        Vector4 { value: xxxz }
    }

    #[inline]
    fn xxyx(self) -> Self::Swizzle4 {
        let xxyx = std::simd::simd_swizzle!(self.value, [0, 0, 1, 0]);

        Vector4 { value: xxyx }
    }

    #[inline]
    fn xxyy(self) -> Self::Swizzle4 {
        let xxyy = std::simd::simd_swizzle!(self.value, [0, 0, 1, 1]);

        Vector4 { value: xxyy }
    }

    #[inline]
    fn xxyz(self) -> Self::Swizzle4 {
        let xxyz = std::simd::simd_swizzle!(self.value, [0, 0, 1, 2]);

        Vector4 { value: xxyz }
    }

    #[inline]
    fn xxzx(self) -> Self::Swizzle4 {
        let xxzx = std::simd::simd_swizzle!(self.value, [0, 0, 2, 0]);

        Vector4 { value: xxzx }
    }

    #[inline]
    fn xxzy(self) -> Self::Swizzle4 {
        let xxzy = std::simd::simd_swizzle!(self.value, [0, 0, 2, 1]);

        Vector4 { value: xxzy }
    }

    #[inline]
    fn xxzz(self) -> Self::Swizzle4 {
        let xxzz = std::simd::simd_swizzle!(self.value, [0, 0, 2, 2]);

        Vector4 { value: xxzz }
    }

    #[inline]
    fn xyxx(self) -> Self::Swizzle4 {
        let xyxx = std::simd::simd_swizzle!(self.value, [0, 1, 0, 0]);

        Vector4 { value: xyxx }
    }

    #[inline]
    fn xyxy(self) -> Self::Swizzle4 {
        let xyxy = std::simd::simd_swizzle!(self.value, [0, 1, 0, 1]);

        Vector4 { value: xyxy }
    }

    #[inline]
    fn xyxz(self) -> Self::Swizzle4 {
        let xyxz = std::simd::simd_swizzle!(self.value, [0, 1, 0, 2]);

        Vector4 { value: xyxz }
    }

    #[inline]
    fn xyyx(self) -> Self::Swizzle4 {
        let xyyx = std::simd::simd_swizzle!(self.value, [0, 1, 1, 0]);

        Vector4 { value: xyyx }
    }

    #[inline]
    fn xyyy(self) -> Self::Swizzle4 {
        let xyyy = std::simd::simd_swizzle!(self.value, [0, 1, 1, 1]);

        Vector4 { value: xyyy }
    }

    #[inline]
    fn xyyz(self) -> Self::Swizzle4 {
        let xyyz = std::simd::simd_swizzle!(self.value, [0, 1, 1, 2]);

        Vector4 { value: xyyz }
    }

    #[inline]
    fn xyzx(self) -> Self::Swizzle4 {
        let xyzx = std::simd::simd_swizzle!(self.value, [0, 1, 2, 0]);

        Vector4 { value: xyzx }
    }

    #[inline]
    fn xyzy(self) -> Self::Swizzle4 {
        let xyzy = std::simd::simd_swizzle!(self.value, [0, 1, 2, 1]);

        Vector4 { value: xyzy }
    }

    #[inline]
    fn xyzz(self) -> Self::Swizzle4 {
        let xyzz = std::simd::simd_swizzle!(self.value, [0, 1, 2, 2]);

        Vector4 { value: xyzz }
    }

    #[inline]
    fn xzxx(self) -> Self::Swizzle4 {
        let xzxx = std::simd::simd_swizzle!(self.value, [0, 2, 0, 0]);

        Vector4 { value: xzxx }
    }

    #[inline]
    fn xzxy(self) -> Self::Swizzle4 {
        let xzxy = std::simd::simd_swizzle!(self.value, [0, 2, 0, 1]);

        Vector4 { value: xzxy }
    }

    #[inline]
    fn xzxz(self) -> Self::Swizzle4 {
        let xzxz = std::simd::simd_swizzle!(self.value, [0, 2, 0, 2]);

        Vector4 { value: xzxz }
    }

    #[inline]
    fn xzyx(self) -> Self::Swizzle4 {
        let xzyx = std::simd::simd_swizzle!(self.value, [0, 2, 1, 0]);

        Vector4 { value: xzyx }
    }

    #[inline]
    fn xzyy(self) -> Self::Swizzle4 {
        let xzyy = std::simd::simd_swizzle!(self.value, [0, 2, 1, 1]);

        Vector4 { value: xzyy }
    }

    #[inline]
    fn xzyz(self) -> Self::Swizzle4 {
        let xzyz = std::simd::simd_swizzle!(self.value, [0, 2, 1, 2]);

        Vector4 { value: xzyz }
    }

    #[inline]
    fn xzzx(self) -> Self::Swizzle4 {
        let xzzx = std::simd::simd_swizzle!(self.value, [0, 2, 2, 0]);

        Vector4 { value: xzzx }
    }

    #[inline]
    fn xzzy(self) -> Self::Swizzle4 {
        let xzzy = std::simd::simd_swizzle!(self.value, [0, 2, 2, 1]);

        Vector4 { value: xzzy }
    }

    #[inline]
    fn xzzz(self) -> Self::Swizzle4 {
        let xzzz = std::simd::simd_swizzle!(self.value, [0, 2, 2, 2]);

        Vector4 { value: xzzz }
    }

    #[inline]
    fn yxxx(self) -> Self::Swizzle4 {
        let yxxx = std::simd::simd_swizzle!(self.value, [1, 0, 0, 0]);

        Vector4 { value: yxxx }
    }

    #[inline]
    fn yxxy(self) -> Self::Swizzle4 {
        let yxxy = std::simd::simd_swizzle!(self.value, [1, 0, 0, 1]);

        Vector4 { value: yxxy }
    }

    #[inline]
    fn yxxz(self) -> Self::Swizzle4 {
        let yxxz = std::simd::simd_swizzle!(self.value, [1, 0, 0, 2]);

        Vector4 { value: yxxz }
    }

    #[inline]
    fn yxyx(self) -> Self::Swizzle4 {
        let yxyx = std::simd::simd_swizzle!(self.value, [1, 0, 1, 0]);

        Vector4 { value: yxyx }
    }

    #[inline]
    fn yxyy(self) -> Self::Swizzle4 {
        let yxyy = std::simd::simd_swizzle!(self.value, [1, 0, 1, 1]);

        Vector4 { value: yxyy }
    }

    #[inline]
    fn yxyz(self) -> Self::Swizzle4 {
        let yxyz = std::simd::simd_swizzle!(self.value, [1, 0, 1, 2]);

        Vector4 { value: yxyz }
    }

    #[inline]
    fn yxzx(self) -> Self::Swizzle4 {
        let yxzx = std::simd::simd_swizzle!(self.value, [1, 0, 2, 0]);

        Vector4 { value: yxzx }
    }

    #[inline]
    fn yxzy(self) -> Self::Swizzle4 {
        let yxzy = std::simd::simd_swizzle!(self.value, [1, 0, 2, 1]);

        Vector4 { value: yxzy }
    }

    #[inline]
    fn yxzz(self) -> Self::Swizzle4 {
        let yxzz = std::simd::simd_swizzle!(self.value, [1, 0, 2, 2]);

        Vector4 { value: yxzz }
    }

    #[inline]
    fn yyxx(self) -> Self::Swizzle4 {
        let yyxx = std::simd::simd_swizzle!(self.value, [1, 1, 0, 0]);

        Vector4 { value: yyxx }
    }

    #[inline]
    fn yyxy(self) -> Self::Swizzle4 {
        let yyxy = std::simd::simd_swizzle!(self.value, [1, 1, 0, 1]);

        Vector4 { value: yyxy }
    }

    #[inline]
    fn yyxz(self) -> Self::Swizzle4 {
        let yyxz = std::simd::simd_swizzle!(self.value, [1, 1, 0, 2]);

        Vector4 { value: yyxz }
    }

    #[inline]
    fn yyyx(self) -> Self::Swizzle4 {
        let yyyx = std::simd::simd_swizzle!(self.value, [1, 1, 1, 0]);

        Vector4 { value: yyyx }
    }

    #[inline]
    fn yyyy(self) -> Self::Swizzle4 {
        Vector4::broadcast(self.y())
    }

    #[inline]
    fn yyyz(self) -> Self::Swizzle4 {
        let yyyz = std::simd::simd_swizzle!(self.value, [1, 1, 1, 2]);

        Vector4 { value: yyyz }
    }

    #[inline]
    fn yyzx(self) -> Self::Swizzle4 {
        let yyzx = std::simd::simd_swizzle!(self.value, [1, 1, 2, 0]);

        Vector4 { value: yyzx }
    }

    #[inline]
    fn yyzy(self) -> Self::Swizzle4 {
        let yyzy = std::simd::simd_swizzle!(self.value, [1, 1, 2, 1]);

        Vector4 { value: yyzy }
    }

    #[inline]
    fn yyzz(self) -> Self::Swizzle4 {
        let yyzz = std::simd::simd_swizzle!(self.value, [1, 1, 2, 2]);

        Vector4 { value: yyzz }
    }

    #[inline]
    fn yzxx(self) -> Self::Swizzle4 {
        let yzxx = std::simd::simd_swizzle!(self.value, [1, 2, 0, 0]);

        Vector4 { value: yzxx }
    }

    #[inline]
    fn yzxy(self) -> Self::Swizzle4 {
        let yzxy = std::simd::simd_swizzle!(self.value, [1, 2, 0, 1]);

        Vector4 { value: yzxy }
    }

    #[inline]
    fn yzxz(self) -> Self::Swizzle4 {
        let yzxz = std::simd::simd_swizzle!(self.value, [1, 2, 0, 2]);

        Vector4 { value: yzxz }
    }

    #[inline]
    fn yzyx(self) -> Self::Swizzle4 {
        let yzyx = std::simd::simd_swizzle!(self.value, [1, 2, 1, 0]);

        Vector4 { value: yzyx }
    }

    #[inline]
    fn yzyy(self) -> Self::Swizzle4 {
        let yzyy = std::simd::simd_swizzle!(self.value, [1, 2, 1, 1]);

        Vector4 { value: yzyy }
    }

    #[inline]
    fn yzyz(self) -> Self::Swizzle4 {
        let yzyz = std::simd::simd_swizzle!(self.value, [1, 2, 1, 2]);

        Vector4 { value: yzyz }
    }

    #[inline]
    fn yzzx(self) -> Self::Swizzle4 {
        let yzzx = std::simd::simd_swizzle!(self.value, [1, 2, 2, 0]);

        Vector4 { value: yzzx }
    }

    #[inline]
    fn yzzy(self) -> Self::Swizzle4 {
        let yzzy = std::simd::simd_swizzle!(self.value, [1, 2, 2, 1]);

        Vector4 { value: yzzy }
    }

    #[inline]
    fn yzzz(self) -> Self::Swizzle4 {
        let yzzz = std::simd::simd_swizzle!(self.value, [1, 2, 2, 2]);

        Vector4 { value: yzzz }
    }

    #[inline]
    fn zxxx(self) -> Self::Swizzle4 {
        let zxxx = std::simd::simd_swizzle!(self.value, [2, 0, 0, 0]);

        Vector4 { value: zxxx }
    }

    #[inline]
    fn zxxy(self) -> Self::Swizzle4 {
        let zxxy = std::simd::simd_swizzle!(self.value, [2, 0, 0, 1]);

        Vector4 { value: zxxy }
    }

    #[inline]
    fn zxxz(self) -> Self::Swizzle4 {
        let zxxz = std::simd::simd_swizzle!(self.value, [2, 0, 0, 2]);

        Vector4 { value: zxxz }
    }

    #[inline]
    fn zxyx(self) -> Self::Swizzle4 {
        let zxyx = std::simd::simd_swizzle!(self.value, [2, 0, 1, 0]);

        Vector4 { value: zxyx }
    }

    #[inline]
    fn zxyy(self) -> Self::Swizzle4 {
        let zxyy = std::simd::simd_swizzle!(self.value, [2, 0, 1, 1]);

        Vector4 { value: zxyy }
    }

    #[inline]
    fn zxyz(self) -> Self::Swizzle4 {
        let zxyz = std::simd::simd_swizzle!(self.value, [2, 0, 1, 2]);

        Vector4 { value: zxyz }
    }

    #[inline]
    fn zxzx(self) -> Self::Swizzle4 {
        let zxzx = std::simd::simd_swizzle!(self.value, [2, 0, 2, 0]);

        Vector4 { value: zxzx }
    }

    #[inline]
    fn zxzy(self) -> Self::Swizzle4 {
        let zxzy = std::simd::simd_swizzle!(self.value, [2, 0, 2, 1]);

        Vector4 { value: zxzy }
    }

    #[inline]
    fn zxzz(self) -> Self::Swizzle4 {
        let zxzz = std::simd::simd_swizzle!(self.value, [2, 0, 2, 2]);

        Vector4 { value: zxzz }
    }

    #[inline]
    fn zyxx(self) -> Self::Swizzle4 {
        let zyxx = std::simd::simd_swizzle!(self.value, [2, 1, 0, 0]);

        Vector4 { value: zyxx }
    }

    #[inline]
    fn zyxy(self) -> Self::Swizzle4 {
        let zyxy = std::simd::simd_swizzle!(self.value, [2, 1, 0, 1]);

        Vector4 { value: zyxy }
    }

    #[inline]
    fn zyxz(self) -> Self::Swizzle4 {
        let zyxz = std::simd::simd_swizzle!(self.value, [2, 1, 0, 2]);

        Vector4 { value: zyxz }
    }

    #[inline]
    fn zyyx(self) -> Self::Swizzle4 {
        let zyyx = std::simd::simd_swizzle!(self.value, [2, 1, 1, 0]);

        Vector4 { value: zyyx }
    }

    #[inline]
    fn zyyy(self) -> Self::Swizzle4 {
        let zyyy = std::simd::simd_swizzle!(self.value, [2, 1, 1, 1]);

        Vector4 { value: zyyy }
    }

    #[inline]
    fn zyyz(self) -> Self::Swizzle4 {
        let zyyz = std::simd::simd_swizzle!(self.value, [2, 1, 1, 2]);

        Vector4 { value: zyyz }
    }

    #[inline]
    fn zyzx(self) -> Self::Swizzle4 {
        let zyzx = std::simd::simd_swizzle!(self.value, [2, 1, 2, 0]);

        Vector4 { value: zyzx }
    }

    #[inline]
    fn zyzy(self) -> Self::Swizzle4 {
        let zyzy = std::simd::simd_swizzle!(self.value, [2, 1, 2, 1]);

        Vector4 { value: zyzy }
    }

    #[inline]
    fn zyzz(self) -> Self::Swizzle4 {
        let zyzz = std::simd::simd_swizzle!(self.value, [2, 1, 2, 2]);

        Vector4 { value: zyzz }
    }

    #[inline]
    fn zzxx(self) -> Self::Swizzle4 {
        let zzxx = std::simd::simd_swizzle!(self.value, [2, 2, 0, 0]);

        Vector4 { value: zzxx }
    }

    #[inline]
    fn zzxy(self) -> Self::Swizzle4 {
        let zzxy = std::simd::simd_swizzle!(self.value, [2, 2, 0, 1]);

        Vector4 { value: zzxy }
    }

    #[inline]
    fn zzxz(self) -> Self::Swizzle4 {
        let zzxz = std::simd::simd_swizzle!(self.value, [2, 2, 0, 2]);

        Vector4 { value: zzxz }
    }

    #[inline]
    fn zzyx(self) -> Self::Swizzle4 {
        let zzyx = std::simd::simd_swizzle!(self.value, [2, 2, 1, 0]);

        Vector4 { value: zzyx }
    }

    #[inline]
    fn zzyy(self) -> Self::Swizzle4 {
        let zzyy = std::simd::simd_swizzle!(self.value, [2, 2, 1, 1]);

        Vector4 { value: zzyy }
    }

    #[inline]
    fn zzyz(self) -> Self::Swizzle4 {
        let zzyz = std::simd::simd_swizzle!(self.value, [2, 2, 1, 2]);

        Vector4 { value: zzyz }
    }

    #[inline]
    fn zzzx(self) -> Self::Swizzle4 {
        let zzzx = std::simd::simd_swizzle!(self.value, [2, 2, 2, 0]);

        Vector4 { value: zzzx }
    }

    #[inline]
    fn zzzy(self) -> Self::Swizzle4 {
        let zzzy = std::simd::simd_swizzle!(self.value, [2, 2, 2, 1]);

        Vector4 { value: zzzy }
    }

    #[inline]
    fn zzzz(self) -> Self::Swizzle4 {
        Vector4::broadcast(self.z())
    }
}
