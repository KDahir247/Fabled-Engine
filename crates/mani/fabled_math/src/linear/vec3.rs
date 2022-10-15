use crate::math::{component_sum, mul_add};
use crate::math_trait::Vec3Swizzles;
use crate::matrix3x3_math::transpose;
use crate::{cross, Matrix3x3, Quaternion, Vector2, Vector4};
use std::fmt::{Display, Formatter};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

#[derive(Copy, Clone, PartialEq, Debug)]
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

    pub const NEG_ONE: Vector3 = Vector3 {
        value: std::simd::f32x4::from_array([-1.0, -1.0, -1.0, 0.0]),
    };

    pub const ZERO: Vector3 = Vector3 {
        value: std::simd::f32x4::from_array([0.0, 0.0, 0.0, 0.0]),
    };

    #[inline]
    pub fn trunc_vec2(self) -> Vector2 {
        let a = self.value.to_array();

        Vector2 {
            value: [a[0], a[1]],
        }
    }

    #[inline(always)]
    pub const fn set(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 {
            value: std::simd::f32x4::from_array([x, y, z, 0.0]),
        }
    }

    #[inline]
    pub const fn splat(val: f32) -> Vector3 {
        Vector3 {
            value: std::simd::f32x4::from_array([val; 4]),
        }
    }

    #[inline]
    pub const fn to_primitive(self) -> [f32; 3] {
        let simd_rep = self.value.to_array();
        let z = simd_rep[2];

        [simd_rep[0], simd_rep[1], z]
    }

    #[inline]
    pub const fn from_array(array: [f32; 3]) -> Vector3 {
        let z = array[2];

        Vector3::set(array[0], array[1], z)
    }

    #[inline]
    pub const fn x(self) -> f32 {
        let array_vec3: [f32; 4] = self.value.to_array();

        array_vec3[0]
    }

    #[inline]
    pub const fn y(self) -> f32 {
        let array_vec3: [f32; 4] = self.value.to_array();

        array_vec3[1]
    }

    #[inline]
    pub const fn z(self) -> f32 {
        let array_vec3: [f32; 4] = self.value.to_array();

        array_vec3[2]
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
        let mut result = self;
        result *= rhs;
        result
    }
}

impl MulAssign<Quaternion> for Vector3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Quaternion) {
        let quaternion_scalar = rhs.value[3];

        const TWO_VEC: std::simd::f32x4 = std::simd::f32x4::from_array([2.0; 4]);
        let quaternion_scalar_vector = std::simd::f32x4::from_array([quaternion_scalar; 4]);

        let t = TWO_VEC * cross(rhs.value, self.value);
        let intermediate = cross(rhs.value, t);

        self.value += mul_add(quaternion_scalar_vector, t, intermediate);
    }
}

impl Mul<Vector3> for Matrix3x3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        let row_col_matrix = transpose(self);

        let row_mul_col_vec_x = row_col_matrix.column_x * rhs;
        let row_mul_col_vec_y = row_col_matrix.column_y * rhs;
        let row_mul_col_vec_z = row_col_matrix.column_z * rhs;

        let x = component_sum(row_mul_col_vec_x.value);
        let y = component_sum(row_mul_col_vec_y.value);
        let z = component_sum(row_mul_col_vec_z.value);

        Vector3::set(x, y, z)
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
        Vector3 { value: -self.value }
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

impl Vec3Swizzles for Vector3 {
    type Vec2 = Vector2;
    type Vec4 = Vector4;

    #[inline]
    fn xx(self) -> Self::Vec2 {
        Vector2::splat(self.x())
    }

    #[inline]
    fn xy(self) -> Self::Vec2 {
        let arr_vector3 = self.value.to_array();
        let y = arr_vector3[1];
        let x = arr_vector3[0];

        Vector2::set(x, y)
    }

    #[inline]
    fn xz(self) -> Self::Vec2 {
        let arr_vector3 = self.value.to_array();
        let z = arr_vector3[2];
        let x = arr_vector3[0];

        Vector2::set(x, z)
    }

    #[inline]
    fn yx(self) -> Self::Vec2 {
        let arr_vector3 = self.value.to_array();
        let y = arr_vector3[1];
        let x = arr_vector3[0];

        Vector2::set(y, x)
    }

    #[inline]
    fn yy(self) -> Self::Vec2 {
        Vector2::splat(self.y())
    }

    #[inline]
    fn yz(self) -> Self::Vec2 {
        let arr_vector3 = self.value.to_array();
        let z = arr_vector3[2];
        let y = arr_vector3[1];

        Vector2::set(y, z)
    }

    #[inline]
    fn zx(self) -> Self::Vec2 {
        let arr_vector3 = self.value.to_array();
        let z = arr_vector3[2];
        let x = arr_vector3[0];

        Vector2::set(z, x)
    }

    #[inline]
    fn zy(self) -> Self::Vec2 {
        let arr_vector3 = self.value.to_array();
        let z = arr_vector3[2];
        let y = arr_vector3[1];

        Vector2::set(z, y)
    }

    #[inline]
    fn zz(self) -> Self::Vec2 {
        Vector2::splat(self.z())
    }

    #[inline]
    fn xxx(self) -> Self {
        Vector3::splat(self.x())
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
    fn xxxx(self) -> Self::Vec4 {
        let xxxx = std::simd::simd_swizzle!(self.value, [0, 0, 0, 0]);

        Vector4 { value: xxxx }
    }

    #[inline]
    fn xxxy(self) -> Self::Vec4 {
        let xxxy = std::simd::simd_swizzle!(self.value, [0, 0, 0, 1]);

        Vector4 { value: xxxy }
    }

    #[inline]
    fn xxxz(self) -> Self::Vec4 {
        let xxxz = std::simd::simd_swizzle!(self.value, [0, 0, 0, 2]);

        Vector4 { value: xxxz }
    }

    #[inline]
    fn xxyx(self) -> Self::Vec4 {
        let xxyx = std::simd::simd_swizzle!(self.value, [0, 0, 1, 0]);

        Vector4 { value: xxyx }
    }

    #[inline]
    fn xxyy(self) -> Self::Vec4 {
        let xxyy = std::simd::simd_swizzle!(self.value, [0, 0, 1, 1]);

        Vector4 { value: xxyy }
    }

    #[inline]
    fn xxyz(self) -> Self::Vec4 {
        let xxyz = std::simd::simd_swizzle!(self.value, [0, 0, 1, 2]);

        Vector4 { value: xxyz }
    }

    #[inline]
    fn xxzx(self) -> Self::Vec4 {
        let xxzx = std::simd::simd_swizzle!(self.value, [0, 0, 2, 0]);

        Vector4 { value: xxzx }
    }

    #[inline]
    fn xxzy(self) -> Self::Vec4 {
        let xxzy = std::simd::simd_swizzle!(self.value, [0, 0, 2, 1]);

        Vector4 { value: xxzy }
    }

    #[inline]
    fn xxzz(self) -> Self::Vec4 {
        let xxzz = std::simd::simd_swizzle!(self.value, [0, 0, 2, 2]);

        Vector4 { value: xxzz }
    }

    #[inline]
    fn xyxx(self) -> Self::Vec4 {
        let xyxx = std::simd::simd_swizzle!(self.value, [0, 1, 0, 0]);

        Vector4 { value: xyxx }
    }

    #[inline]
    fn xyxy(self) -> Self::Vec4 {
        let xyxy = std::simd::simd_swizzle!(self.value, [0, 1, 0, 1]);

        Vector4 { value: xyxy }
    }

    #[inline]
    fn xyxz(self) -> Self::Vec4 {
        let xyxz = std::simd::simd_swizzle!(self.value, [0, 1, 0, 2]);

        Vector4 { value: xyxz }
    }

    #[inline]
    fn xyyx(self) -> Self::Vec4 {
        let xyyx = std::simd::simd_swizzle!(self.value, [0, 1, 1, 0]);

        Vector4 { value: xyyx }
    }

    #[inline]
    fn xyyy(self) -> Self::Vec4 {
        let xyyy = std::simd::simd_swizzle!(self.value, [0, 1, 1, 1]);

        Vector4 { value: xyyy }
    }

    #[inline]
    fn xyyz(self) -> Self::Vec4 {
        let xyyz = std::simd::simd_swizzle!(self.value, [0, 1, 1, 2]);

        Vector4 { value: xyyz }
    }

    #[inline]
    fn xyzx(self) -> Self::Vec4 {
        let xyzx = std::simd::simd_swizzle!(self.value, [0, 1, 2, 0]);

        Vector4 { value: xyzx }
    }

    #[inline]
    fn xyzy(self) -> Self::Vec4 {
        let xyzy = std::simd::simd_swizzle!(self.value, [0, 1, 2, 1]);

        Vector4 { value: xyzy }
    }

    #[inline]
    fn xyzz(self) -> Self::Vec4 {
        let xyzz = std::simd::simd_swizzle!(self.value, [0, 1, 2, 2]);

        Vector4 { value: xyzz }
    }

    #[inline]
    fn xzxx(self) -> Self::Vec4 {
        let xzxx = std::simd::simd_swizzle!(self.value, [0, 2, 0, 0]);

        Vector4 { value: xzxx }
    }

    #[inline]
    fn xzxy(self) -> Self::Vec4 {
        let xzxy = std::simd::simd_swizzle!(self.value, [0, 2, 0, 1]);

        Vector4 { value: xzxy }
    }

    #[inline]
    fn xzxz(self) -> Self::Vec4 {
        let xzxz = std::simd::simd_swizzle!(self.value, [0, 2, 0, 2]);

        Vector4 { value: xzxz }
    }

    #[inline]
    fn xzyx(self) -> Self::Vec4 {
        let xzyx = std::simd::simd_swizzle!(self.value, [0, 2, 1, 0]);

        Vector4 { value: xzyx }
    }

    #[inline]
    fn xzyy(self) -> Self::Vec4 {
        let xzyy = std::simd::simd_swizzle!(self.value, [0, 2, 1, 1]);

        Vector4 { value: xzyy }
    }

    #[inline]
    fn xzyz(self) -> Self::Vec4 {
        let xzyz = std::simd::simd_swizzle!(self.value, [0, 2, 1, 2]);

        Vector4 { value: xzyz }
    }

    #[inline]
    fn xzzx(self) -> Self::Vec4 {
        let xzzx = std::simd::simd_swizzle!(self.value, [0, 2, 2, 0]);

        Vector4 { value: xzzx }
    }

    #[inline]
    fn xzzy(self) -> Self::Vec4 {
        let xzzy = std::simd::simd_swizzle!(self.value, [0, 2, 2, 1]);

        Vector4 { value: xzzy }
    }

    #[inline]
    fn xzzz(self) -> Self::Vec4 {
        let xzzz = std::simd::simd_swizzle!(self.value, [0, 2, 2, 2]);

        Vector4 { value: xzzz }
    }

    #[inline]
    fn yxxx(self) -> Self::Vec4 {
        let yxxx = std::simd::simd_swizzle!(self.value, [1, 0, 0, 0]);

        Vector4 { value: yxxx }
    }

    #[inline]
    fn yxxy(self) -> Self::Vec4 {
        let yxxy = std::simd::simd_swizzle!(self.value, [1, 0, 0, 1]);

        Vector4 { value: yxxy }
    }

    #[inline]
    fn yxxz(self) -> Self::Vec4 {
        let yxxz = std::simd::simd_swizzle!(self.value, [1, 0, 0, 2]);

        Vector4 { value: yxxz }
    }

    #[inline]
    fn yxyx(self) -> Self::Vec4 {
        let yxyx = std::simd::simd_swizzle!(self.value, [1, 0, 1, 0]);

        Vector4 { value: yxyx }
    }

    #[inline]
    fn yxyy(self) -> Self::Vec4 {
        let yxyy = std::simd::simd_swizzle!(self.value, [1, 0, 1, 1]);

        Vector4 { value: yxyy }
    }

    #[inline]
    fn yxyz(self) -> Self::Vec4 {
        let yxyz = std::simd::simd_swizzle!(self.value, [1, 0, 1, 2]);

        Vector4 { value: yxyz }
    }

    #[inline]
    fn yxzx(self) -> Self::Vec4 {
        let yxzx = std::simd::simd_swizzle!(self.value, [1, 0, 2, 0]);

        Vector4 { value: yxzx }
    }

    #[inline]
    fn yxzy(self) -> Self::Vec4 {
        let yxzy = std::simd::simd_swizzle!(self.value, [1, 0, 2, 1]);

        Vector4 { value: yxzy }
    }

    #[inline]
    fn yxzz(self) -> Self::Vec4 {
        let yxzz = std::simd::simd_swizzle!(self.value, [1, 0, 2, 2]);

        Vector4 { value: yxzz }
    }

    #[inline]
    fn yyxx(self) -> Self::Vec4 {
        let yyxx = std::simd::simd_swizzle!(self.value, [1, 1, 0, 0]);

        Vector4 { value: yyxx }
    }

    #[inline]
    fn yyxy(self) -> Self::Vec4 {
        let yyxy = std::simd::simd_swizzle!(self.value, [1, 1, 0, 1]);

        Vector4 { value: yyxy }
    }

    #[inline]
    fn yyxz(self) -> Self::Vec4 {
        let yyxz = std::simd::simd_swizzle!(self.value, [1, 1, 0, 2]);

        Vector4 { value: yyxz }
    }

    #[inline]
    fn yyyx(self) -> Self::Vec4 {
        let yyyx = std::simd::simd_swizzle!(self.value, [1, 1, 1, 0]);

        Vector4 { value: yyyx }
    }

    #[inline]
    fn yyyy(self) -> Self::Vec4 {
        Vector4::splat(self.y())
    }

    #[inline]
    fn yyyz(self) -> Self::Vec4 {
        let yyyz = std::simd::simd_swizzle!(self.value, [1, 1, 1, 2]);

        Vector4 { value: yyyz }
    }

    #[inline]
    fn yyzx(self) -> Self::Vec4 {
        let yyzx = std::simd::simd_swizzle!(self.value, [1, 1, 2, 0]);

        Vector4 { value: yyzx }
    }

    #[inline]
    fn yyzy(self) -> Self::Vec4 {
        let yyzy = std::simd::simd_swizzle!(self.value, [1, 1, 2, 1]);

        Vector4 { value: yyzy }
    }

    #[inline]
    fn yyzz(self) -> Self::Vec4 {
        let yyzz = std::simd::simd_swizzle!(self.value, [1, 1, 2, 2]);

        Vector4 { value: yyzz }
    }

    #[inline]
    fn yzxx(self) -> Self::Vec4 {
        let yzxx = std::simd::simd_swizzle!(self.value, [1, 2, 0, 0]);

        Vector4 { value: yzxx }
    }

    #[inline]
    fn yzxy(self) -> Self::Vec4 {
        let yzxy = std::simd::simd_swizzle!(self.value, [1, 2, 0, 1]);

        Vector4 { value: yzxy }
    }

    #[inline]
    fn yzxz(self) -> Self::Vec4 {
        let yzxz = std::simd::simd_swizzle!(self.value, [1, 2, 0, 2]);

        Vector4 { value: yzxz }
    }

    #[inline]
    fn yzyx(self) -> Self::Vec4 {
        let yzyx = std::simd::simd_swizzle!(self.value, [1, 2, 1, 0]);

        Vector4 { value: yzyx }
    }

    #[inline]
    fn yzyy(self) -> Self::Vec4 {
        let yzyy = std::simd::simd_swizzle!(self.value, [1, 2, 1, 1]);

        Vector4 { value: yzyy }
    }

    #[inline]
    fn yzyz(self) -> Self::Vec4 {
        let yzyz = std::simd::simd_swizzle!(self.value, [1, 2, 1, 2]);

        Vector4 { value: yzyz }
    }

    #[inline]
    fn yzzx(self) -> Self::Vec4 {
        let yzzx = std::simd::simd_swizzle!(self.value, [1, 2, 2, 0]);

        Vector4 { value: yzzx }
    }

    #[inline]
    fn yzzy(self) -> Self::Vec4 {
        let yzzy = std::simd::simd_swizzle!(self.value, [1, 2, 2, 1]);

        Vector4 { value: yzzy }
    }

    #[inline]
    fn yzzz(self) -> Self::Vec4 {
        let yzzz = std::simd::simd_swizzle!(self.value, [1, 2, 2, 2]);

        Vector4 { value: yzzz }
    }

    #[inline]
    fn zxxx(self) -> Self::Vec4 {
        let zxxx = std::simd::simd_swizzle!(self.value, [2, 0, 0, 0]);

        Vector4 { value: zxxx }
    }

    #[inline]
    fn zxxy(self) -> Self::Vec4 {
        let zxxy = std::simd::simd_swizzle!(self.value, [2, 0, 0, 1]);

        Vector4 { value: zxxy }
    }

    #[inline]
    fn zxxz(self) -> Self::Vec4 {
        let zxxz = std::simd::simd_swizzle!(self.value, [2, 0, 0, 2]);

        Vector4 { value: zxxz }
    }

    #[inline]
    fn zxyx(self) -> Self::Vec4 {
        let zxyx = std::simd::simd_swizzle!(self.value, [2, 0, 1, 0]);

        Vector4 { value: zxyx }
    }

    #[inline]
    fn zxyy(self) -> Self::Vec4 {
        let zxyy = std::simd::simd_swizzle!(self.value, [2, 0, 1, 1]);

        Vector4 { value: zxyy }
    }

    #[inline]
    fn zxyz(self) -> Self::Vec4 {
        let zxyz = std::simd::simd_swizzle!(self.value, [2, 0, 1, 2]);

        Vector4 { value: zxyz }
    }

    #[inline]
    fn zxzx(self) -> Self::Vec4 {
        let zxzx = std::simd::simd_swizzle!(self.value, [2, 0, 2, 0]);

        Vector4 { value: zxzx }
    }

    #[inline]
    fn zxzy(self) -> Self::Vec4 {
        let zxzy = std::simd::simd_swizzle!(self.value, [2, 0, 2, 1]);

        Vector4 { value: zxzy }
    }

    #[inline]
    fn zxzz(self) -> Self::Vec4 {
        let zxzz = std::simd::simd_swizzle!(self.value, [2, 0, 2, 2]);

        Vector4 { value: zxzz }
    }

    #[inline]
    fn zyxx(self) -> Self::Vec4 {
        let zyxx = std::simd::simd_swizzle!(self.value, [2, 1, 0, 0]);

        Vector4 { value: zyxx }
    }

    #[inline]
    fn zyxy(self) -> Self::Vec4 {
        let zyxy = std::simd::simd_swizzle!(self.value, [2, 1, 0, 1]);

        Vector4 { value: zyxy }
    }

    #[inline]
    fn zyxz(self) -> Self::Vec4 {
        let zyxz = std::simd::simd_swizzle!(self.value, [2, 1, 0, 2]);

        Vector4 { value: zyxz }
    }

    #[inline]
    fn zyyx(self) -> Self::Vec4 {
        let zyyx = std::simd::simd_swizzle!(self.value, [2, 1, 1, 0]);

        Vector4 { value: zyyx }
    }

    #[inline]
    fn zyyy(self) -> Self::Vec4 {
        let zyyy = std::simd::simd_swizzle!(self.value, [2, 1, 1, 1]);

        Vector4 { value: zyyy }
    }

    #[inline]
    fn zyyz(self) -> Self::Vec4 {
        let zyyz = std::simd::simd_swizzle!(self.value, [2, 1, 1, 2]);

        Vector4 { value: zyyz }
    }

    #[inline]
    fn zyzx(self) -> Self::Vec4 {
        let zyzx = std::simd::simd_swizzle!(self.value, [2, 1, 2, 0]);

        Vector4 { value: zyzx }
    }

    #[inline]
    fn zyzy(self) -> Self::Vec4 {
        let zyzy = std::simd::simd_swizzle!(self.value, [2, 1, 2, 1]);

        Vector4 { value: zyzy }
    }

    #[inline]
    fn zyzz(self) -> Self::Vec4 {
        let zyzz = std::simd::simd_swizzle!(self.value, [2, 1, 2, 2]);

        Vector4 { value: zyzz }
    }

    #[inline]
    fn zzxx(self) -> Self::Vec4 {
        let zzxx = std::simd::simd_swizzle!(self.value, [2, 2, 0, 0]);

        Vector4 { value: zzxx }
    }

    #[inline]
    fn zzxy(self) -> Self::Vec4 {
        let zzxy = std::simd::simd_swizzle!(self.value, [2, 2, 0, 1]);

        Vector4 { value: zzxy }
    }

    #[inline]
    fn zzxz(self) -> Self::Vec4 {
        let zzxz = std::simd::simd_swizzle!(self.value, [2, 2, 0, 2]);

        Vector4 { value: zzxz }
    }

    #[inline]
    fn zzyx(self) -> Self::Vec4 {
        let zzyx = std::simd::simd_swizzle!(self.value, [2, 2, 1, 0]);

        Vector4 { value: zzyx }
    }

    #[inline]
    fn zzyy(self) -> Self::Vec4 {
        let zzyy = std::simd::simd_swizzle!(self.value, [2, 2, 1, 1]);

        Vector4 { value: zzyy }
    }

    #[inline]
    fn zzyz(self) -> Self::Vec4 {
        let zzyz = std::simd::simd_swizzle!(self.value, [2, 2, 1, 2]);

        Vector4 { value: zzyz }
    }

    #[inline]
    fn zzzx(self) -> Self::Vec4 {
        let zzzx = std::simd::simd_swizzle!(self.value, [2, 2, 2, 0]);

        Vector4 { value: zzzx }
    }

    #[inline]
    fn zzzy(self) -> Self::Vec4 {
        let zzzy = std::simd::simd_swizzle!(self.value, [2, 2, 2, 1]);

        Vector4 { value: zzzy }
    }

    #[inline]
    fn zzzz(self) -> Self::Vec4 {
        Vector4::splat(self.z())
    }
}
