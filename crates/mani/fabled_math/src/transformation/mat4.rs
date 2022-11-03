use crate::{Affine3, Vector4};

use crate::matrix4x4_math::transpose_mat4;

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Matrix4x4 {
    pub column_x: Vector4,
    pub column_y: Vector4,
    pub column_z: Vector4,
    pub column_w: Vector4,
}

#[rustfmt::skip]
impl Default for Matrix4x4 {
    fn default() -> Self {
        Self {
            column_x: Vector4::set(1.0, 0.0, 0.0, 0.0),
            column_y: Vector4::set(0.0, 1.0, 0.0, 0.0),
            column_z: Vector4::set(0.0, 0.0, 1.0, 0.0),
            column_w: Vector4::set(0.0, 0.0, 0.0, 1.0),
        }
    }
}

impl Matrix4x4 {
    #[rustfmt::skip]
    pub const IDENTITY : Matrix4x4 = Matrix4x4{
        column_x: Vector4::set(1.0, 0.0, 0.0, 0.0),
        column_y: Vector4::set(0.0, 1.0, 0.0, 0.0),
        column_z: Vector4::set(0.0, 0.0, 1.0, 0.0),
        column_w: Vector4::set(0.0, 0.0, 0.0, 1.0),
    };

    pub const ZERO : Matrix4x4 = Matrix4x4{
        column_x: Vector4::ZERO,
        column_y: Vector4::ZERO,
        column_z: Vector4::ZERO,
        column_w: Vector4::ZERO,
    };
}

impl Matrix4x4 {
    #[inline]
    pub const fn set( column_x: Vector4, column_y: Vector4, column_z: Vector4, column_w: Vector4) -> Matrix4x4 {
        Matrix4x4 {
            column_x,
            column_y,
            column_z,
            column_w,
        }
    }


    #[inline]
    pub const fn broadcast(val: f32) -> Matrix4x4 {
        let splat_vector4: Vector4 = Vector4::broadcast(val);

        Matrix4x4 {
            column_x: splat_vector4,
            column_y: splat_vector4,
            column_z: splat_vector4,
            column_w: splat_vector4,
        }
    }

    #[inline]
    pub const fn from_primitive(array: [f32; 16]) -> Matrix4x4{
        let w_column = [array[3], array[7], array[11], array[15]];
        let x_column = [array[0], array[4], array[8], array[12]];
        let y_column = [array[1], array[5], array[9], array[13]];
        let z_column = [array[2], array[6], array[10], array[14]];

        Matrix4x4 {
            column_x: Vector4::from_primitive(x_column),
            column_y: Vector4::from_primitive(y_column),
            column_z: Vector4::from_primitive(z_column),
            column_w: Vector4::from_primitive(w_column)
        }

    }

    #[rustfmt::skip]
    #[inline]
    pub const fn to_primitive(self) -> [f32; 16] {
        let x_column = self.column_x;
        let y_column = self.column_y;
        let z_column = self.column_y;
        let w_column = self.column_w;

        [
            x_column.x(), y_column.x(), z_column.x(), w_column.x(),
            x_column.y(), y_column.y(), z_column.y(), w_column.y(),
            x_column.z(), y_column.z(), z_column.z(), w_column.z(),
            x_column.w(), y_column.w(), z_column.w(), w_column.w(),
        ]
    }

    #[inline]
    pub const fn to_diagonal(self) -> Vector4 {
        Vector4::set(
            self.column_x.x(),
            self.column_y.y(),
            self.column_z.z(),
            self.column_w.w(),
        )
    }
}

impl Display for Matrix4x4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Matrix4x4\n[\n\t{}\n\t{}\n\t{}\n\t{}\n]",
            self.column_x, self.column_y, self.column_z, self.column_w
        )
    }
}

// Component-Wise
impl Add<f32> for Matrix4x4 {
    type Output = Matrix4x4;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        let splat_vector4 = Vector4::broadcast(rhs);

        Matrix4x4 {
            column_x: self.column_x + splat_vector4,
            column_y: self.column_y + splat_vector4,
            column_z: self.column_z + splat_vector4,
            column_w: self.column_w + splat_vector4,
        }
    }
}

impl AddAssign<f32> for Matrix4x4 {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        let splat_vector4 = Vector4::broadcast(rhs);

        self.column_x += splat_vector4;
        self.column_y += splat_vector4;
        self.column_z += splat_vector4;
        self.column_w += splat_vector4;
    }
}

impl Sub<f32> for Matrix4x4 {
    type Output = Matrix4x4;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        let splat_vector4 = Vector4::broadcast(rhs);

        Matrix4x4 {
            column_x: self.column_x - splat_vector4,
            column_y: self.column_y - splat_vector4,
            column_z: self.column_z - splat_vector4,
            column_w: self.column_w - splat_vector4,
        }
    }
}

impl SubAssign<f32> for Matrix4x4 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        let splat_vector4 = Vector4::broadcast(rhs);

        self.column_x -= splat_vector4;
        self.column_y -= splat_vector4;
        self.column_z -= splat_vector4;
        self.column_w -= splat_vector4;
    }
}

impl Mul<f32> for Matrix4x4 {
    type Output = Matrix4x4;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        let splat_vector4 = Vector4::broadcast(rhs);

        Matrix4x4 {
            column_x: self.column_x * splat_vector4,
            column_y: self.column_y * splat_vector4,
            column_z: self.column_z * splat_vector4,
            column_w: self.column_w * splat_vector4,
        }
    }
}

impl MulAssign<f32> for Matrix4x4 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        let splat_vector4 = Vector4::broadcast(rhs);

        self.column_x *= splat_vector4;
        self.column_y *= splat_vector4;
        self.column_z *= splat_vector4;
        self.column_w *= splat_vector4;
    }
}

// Matrix-Wise
impl Add for Matrix4x4 {
    type Output = Matrix4x4;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Matrix4x4 {
            column_x: self.column_x + rhs.column_x,
            column_y: self.column_y + rhs.column_y,
            column_z: self.column_z + rhs.column_z,
            column_w: self.column_w + rhs.column_w,
        }
    }
}

impl AddAssign for Matrix4x4 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.column_x += rhs.column_x;
        self.column_y += rhs.column_y;
        self.column_z += rhs.column_z;
        self.column_w += rhs.column_w;
    }
}

impl Sub for Matrix4x4 {
    type Output = Matrix4x4;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Matrix4x4 {
            column_x: self.column_x + rhs.column_x,
            column_y: self.column_y + rhs.column_y,
            column_z: self.column_z + rhs.column_z,
            column_w: self.column_w + rhs.column_w,
        }
    }
}

impl SubAssign for Matrix4x4 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.column_x -= rhs.column_x;
        self.column_y -= rhs.column_y;
        self.column_z -= rhs.column_z;
        self.column_w -= rhs.column_w;
    }
}

impl Mul for Matrix4x4 {
    type Output = Matrix4x4;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let transposed_matrix: Matrix4x4 = transpose_mat4(rhs);

        Matrix4x4 {
            column_x: self.column_x * transposed_matrix.column_x,
            column_y: self.column_y * transposed_matrix.column_y,
            column_z: self.column_z * transposed_matrix.column_z,
            column_w: self.column_w * transposed_matrix.column_w,
        }
    }
}

impl MulAssign for Matrix4x4 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        let transposed_matrix: Matrix4x4 = transpose_mat4(rhs);

        self.column_x *= transposed_matrix.column_x;
        self.column_y *= transposed_matrix.column_y;
        self.column_z *= transposed_matrix.column_z;
        self.column_w *= transposed_matrix.column_w;
    }
}


impl Mul<Affine3> for Matrix4x4 {
    type Output = Matrix4x4;

    #[inline]
    fn mul(self, rhs: Affine3) -> Self::Output {
        const VEC3_MASK : std::simd::f32x4 = std::simd::f32x4::from_array([1.0, 1.0, 1.0, 0.0]);

        let column_x_extended = Vector4{ value: rhs.matrix3.column_x.value * VEC3_MASK };
        let column_y_extended = Vector4{ value: rhs.matrix3.column_y.value * VEC3_MASK };
        let column_z_extended = Vector4{ value: rhs.matrix3.column_z.value * VEC3_MASK };
        let column_w_extended = Vector4{ value: rhs.translation.value };

        Matrix4x4{
            column_x: column_x_extended,
            column_y: column_y_extended,
            column_z: column_z_extended,
            column_w: column_w_extended,
        }
    }
}

impl MulAssign<Affine3> for Matrix4x4 {
    #[inline]
    fn mul_assign(&mut self, rhs: Affine3) {
        *self = *self * rhs;
    }
}


pub mod matrix4x4_math {

    use crate::{Matrix4x4, Vector4, Vector3, Quaternion, EulerOrder};

    use crate::quaternion_math::from_euler_quat;
    use crate::math_trait::{Swizzles4, QuaternionSwizzles};

    #[inline]
    pub const fn transpose_mat4(matrix_4x4: Matrix4x4) -> Matrix4x4 {
        let x_column = matrix_4x4.column_x;
        let y_column = matrix_4x4.column_y;
        let z_column = matrix_4x4.column_z;
        let w_column = matrix_4x4.column_w;

        Matrix4x4 {
            column_x: Vector4::set(x_column.x(), y_column.x(), z_column.x(), w_column.x()),
            column_y: Vector4::set(x_column.y(), y_column.y(), z_column.y(), w_column.y()),
            column_z: Vector4::set(x_column.z(), y_column.z(), z_column.z(), w_column.z()),
            column_w: Vector4::set(x_column.w(), y_column.w(), z_column.w(), w_column.w()),
        }
    }

    #[inline]
    pub const fn from_scale_mat4(scalar_vector : Vector3) -> Matrix4x4{
        Matrix4x4::set(
            Vector4::set(scalar_vector.x(), 0.0, 0.0, 0.0),
            Vector4::set(0.0, scalar_vector.y(), 0.0, 0.0),
            Vector4::set(0.0, 0.0, scalar_vector.z(), 0.0),
            Vector4::set(0.0, 0.0, 0.0, 1.0)
        )
    }

    #[inline]
    pub const fn from_translation_mat4(translation_vector: Vector3) -> Matrix4x4{
        Matrix4x4::set(
            Vector4::set(1.0, 0.0, 0.0, 0.0),
            Vector4::set(0.0, 1.0, 0.0, 0.0),
            Vector4::set(0.0, 0.0, 1.0, 0.0),
            Vector4 { value: translation_vector.value }
        )
    }

    #[inline]
    pub fn rotate_x_mat4(angle_radian: f32) -> Matrix4x4{
        const NEG_SIN_VECTOR4: Vector4 = Vector4::set(1.0, -1.0, 1.0, 1.0);

        let (sin_angle, cos_angle) = angle_radian.sin_cos();

        let cos_sin_vector: Vector4 = Vector4::set(0.0, cos_angle, sin_angle,0.0);

        let neg_sin_cos_vector: Vector4 = cos_sin_vector.xzyw() * NEG_SIN_VECTOR4;

        Matrix4x4::set(Vector4::RIGHT, cos_sin_vector, neg_sin_cos_vector, Vector4::W)
    }

    #[inline]
    pub fn rotate_y_mat4(angle_radian: f32) -> Matrix4x4{
        const NEG_SIN_VECTOR4: Vector4 = Vector4::set(-1.0, 1.0, 1.0, 1.0);

        let (sin_angle, cos_angle) = angle_radian.sin_cos();

        let cos_sin_vector: Vector4 = Vector4::set(cos_angle, 0.0, -sin_angle, 0.0);

        let neg_sin_cos_vector: Vector4 = cos_sin_vector.zyxw() * NEG_SIN_VECTOR4;

        Matrix4x4::set(cos_sin_vector, Vector4::UP, neg_sin_cos_vector, Vector4::W)
    }

    #[inline]
    pub fn rotate_z_mat4(angle_radian: f32) -> Matrix4x4{
        const NEG_SIN_VECTOR4: Vector4 = Vector4::set(-1.0, 1.0, 1.0, 1.0);

        let (sin_angle, cos_angle) = angle_radian.sin_cos();

        let cos_sin_vector: Vector4 = Vector4::set(cos_angle, sin_angle, 0.0, 0.0);

        let neg_sin_cos_vector:Vector4 = cos_sin_vector.yxzw() * NEG_SIN_VECTOR4;

        Matrix4x4::set(cos_sin_vector, neg_sin_cos_vector, Vector4::FORWARD, Vector4::W)
    }

    #[inline]
    pub fn from_angle_axis_mat4(axis_normalized: Vector3, angle_radian: f32) -> Matrix4x4{
        let (sin_angle, cos_angle) = angle_radian.sin_cos();

        let axis_sqr : Vector3 = axis_normalized * axis_normalized;

        let zy: f32 = axis_normalized.z() * axis_normalized.y();
        let xz: f32 = axis_normalized.x() * axis_normalized.z();
        let yx: f32 = axis_normalized.y() * axis_normalized.x();
        let axis_mul_sin : Vector3 = axis_normalized * sin_angle;

        let one_min_cos: f32 = 1.0 - cos_angle;

        let yx_theta: f32 = yx * one_min_cos;
        let xz_theta: f32 = xz * one_min_cos;
        let zy_theta: f32 = zy * one_min_cos;

        let axis_theta: Vector3 = axis_sqr * one_min_cos;

        Matrix4x4::set(
            Vector4::set(
                axis_theta.x() + cos_angle,
                yx_theta + axis_mul_sin.z(),
                xz_theta - axis_mul_sin.y(),
                0.0
            ),
            Vector4::set(
                yx_theta - axis_mul_sin.z(),
                axis_theta.y() + cos_angle,
                zy_theta + axis_mul_sin.x(),
                0.0
            ),
            Vector4::set(
                xz_theta + axis_mul_sin.y(),
                zy_theta - axis_mul_sin.x(),
                axis_theta.z() + cos_angle,
                0.0
            ),
            Vector4::W
        )
    }


    #[inline]
    pub fn from_quat_mat4(quaternion: Quaternion) -> Matrix4x4{

        let quaternion2: Quaternion = quaternion + quaternion;

        let a: std::simd::f32x4 = quaternion.jiij().value * quaternion2.jjkk().value;

        let b: std::simd::f32x4 = quaternion.kwww().value * quaternion2.kkji().value;

        let ii2: f32 = quaternion.i() * quaternion2.i();

        let interleave = a.interleave(b);

        let first: Vector4 = Vector4 { value: interleave.0 };
        let second: Vector4 = Vector4 { value: interleave.1 };

        let column_0: Vector4 = Vector4::set(-first.y() - first.x() + 1.0, first.w() + first.z(), second.x() - second.y(), 0.0);
        let column_1: Vector4 = Vector4::set(first.z() - first.w(), -ii2 - first.y() + 1.0, second.w() + second.z(), 0.0);
        let column_2: Vector4 = Vector4::set(second.y() + second.x(), second.z() - second.w(), -ii2 - first.x() + 1.0, 0.0);

        Matrix4x4::set(column_0, column_1, column_2, Vector4::W)
    }

    #[inline]
    pub fn from_euler_mat4(euler_radian: Vector3, euler_order: EulerOrder) -> Matrix4x4{
        let quaternion = from_euler_quat(euler_radian, euler_order);

        from_quat_mat4(quaternion)
    }

    // add decomposition and composition.


    #[inline]
    pub fn determinant_mat4(matrix : Matrix4x4) -> f32{
        todo!()
    }



    #[inline]
    pub fn inverse_mat4(matrix : Matrix4x4) -> Matrix4x4{
        todo!()
    }
}
