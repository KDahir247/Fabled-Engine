use crate::{Affine3, Matrix3x3, Vector4};

use crate::matrix4x4_math::transpose_mat4;

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use std::fmt::Display;

#[derive(Copy, Clone)]
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

    pub const ZERO: Matrix4x4 = Matrix4x4 {
        column_x: Vector4::ZERO,
        column_y: Vector4::ZERO,
        column_z: Vector4::ZERO,
        column_w: Vector4::ZERO,
    };
}

impl Matrix4x4 {
    #[inline(always)]
    pub const fn set(
        column_x: Vector4,
        column_y: Vector4,
        column_z: Vector4,
        column_w: Vector4,
    ) -> Matrix4x4 {
        Matrix4x4 {
            column_x,
            column_y,
            column_z,
            column_w,
        }
    }


    #[inline(always)]
    pub const fn broadcast(val: f32) -> Matrix4x4 {
        let splat_vector4: Vector4 = Vector4::broadcast(val);

        Matrix4x4 {
            column_x: splat_vector4,
            column_y: splat_vector4,
            column_z: splat_vector4,
            column_w: splat_vector4,
        }
    }

    #[inline(always)]
    pub const fn to_diagonal(self) -> Vector4 {
        Vector4::set(
                self.column_x.x(),
        self.column_y.y(),
        self.column_z.z(),
        self.column_w.w(),
        )
    }

    #[rustfmt::skip]
    #[inline]
    pub const fn to_primitive(self) -> [f32; 16] {
        let x_column = self.column_x;
        let y_column = self.column_y;
        let z_column = self.column_y;
        let w_column = self.column_w;

        [
            x_column.x(), x_column.y(), x_column.z(), x_column.w(),
            y_column.x(), y_column.y(), y_column.z(), y_column.w(),
            z_column.x(), z_column.y(), z_column.z(), z_column.w(),
            w_column.x(), w_column.y(), w_column.z(), w_column.w(),
            ]
    }

    #[inline]
    pub const fn from_primitive(array: [f32; 16]) -> Matrix4x4 {

        let w_column = [array[12], array[13], array[14], array[15]];
        let x_column = [array[0], array[1], array[2], array[3]];
        let y_column = [array[4], array[5], array[6], array[7]];
        let z_column = [array[8], array[9], array[10], array[11]];

        Matrix4x4 {
            column_x: Vector4::from_primitive(x_column),
            column_y: Vector4::from_primitive(y_column),
            column_z: Vector4::from_primitive(z_column),
            column_w: Vector4::from_primitive(w_column),
        }
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

impl From<Matrix3x3> for Matrix4x4 {
    #[inline]
    fn from(value: Matrix3x3) -> Self {
        Matrix4x4 {
            column_x: Vector4::set(
                value.column_x.x(),
                value.column_x.y(),
                value.column_x.z(),
                0.0,
            ),
            column_y: Vector4::set(
                value.column_y.x(),
                value.column_y.y(),
                value.column_y.z(),
                0.0,
            ),
            column_z: Vector4::set(
                value.column_z.x(),
                value.column_z.y(),
                value.column_z.z(),
                0.0,
            ),
            column_w: Vector4::W,
        }
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
            column_x: self.column_x - rhs.column_x,
            column_y: self.column_y - rhs.column_y,
            column_z: self.column_z - rhs.column_z,
            column_w: self.column_w - rhs.column_w,
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
        const VEC3_MASK: std::simd::f32x4 = std::simd::f32x4::from_array([1.0, 1.0, 1.0, 0.0]);

        let column_x_extended = Vector4 {
            value: rhs.matrix3.column_x.value * VEC3_MASK,
        };
        let column_y_extended = Vector4 {
            value: rhs.matrix3.column_y.value * VEC3_MASK,
        };
        let column_z_extended = Vector4 {
            value: rhs.matrix3.column_z.value * VEC3_MASK,
        };
        let column_w_extended = Vector4 {
            value: rhs.translation.value,
        };

        Matrix4x4 {
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

    use crate::{EulerOrder, Matrix4x4, Quaternion, Vector3, Vector4};

    use crate::quaternion_math::{from_euler_quat, from_rotation_matrix_quat};
    use crate::vector_math::{dot, length, rcp};

    use crate::math_trait::{QuaternionSwizzles, Swizzles4};

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
    pub const fn from_scale_mat4(scalar_vector: Vector3) -> Matrix4x4 {
        Matrix4x4::set(
            Vector4::set(scalar_vector.x(), 0.0, 0.0, 0.0),
            Vector4::set(0.0, scalar_vector.y(), 0.0, 0.0),
            Vector4::set(0.0, 0.0, scalar_vector.z(), 0.0),
            Vector4::set(0.0, 0.0, 0.0, 1.0),
        )
    }

    #[inline]
    pub const fn from_translation_mat4(translation_vector: Vector3) -> Matrix4x4 {
        Matrix4x4::set(
            Vector4::set(1.0, 0.0, 0.0, 0.0),
            Vector4::set(0.0, 1.0, 0.0, 0.0),
            Vector4::set(0.0, 0.0, 1.0, 0.0),
            Vector4 {
                value: translation_vector.value,
            },
        )
    }

    #[inline]
    pub fn rotate_x_mat4(angle_radian: f32) -> Matrix4x4 {
        const NEG_SIN_VECTOR4: Vector4 = Vector4::set(1.0, -1.0, 1.0, 1.0);

        let (sin_angle, cos_angle) = angle_radian.sin_cos();

        let cos_sin_vector: Vector4 = Vector4::set(0.0, cos_angle, sin_angle, 0.0);

        let neg_sin_cos_vector: Vector4 = cos_sin_vector.xzyw() * NEG_SIN_VECTOR4;

        Matrix4x4::set(
            Vector4::RIGHT,
            cos_sin_vector,
            neg_sin_cos_vector,
            Vector4::W,
        )
    }

    #[inline]
    pub fn rotate_y_mat4(angle_radian: f32) -> Matrix4x4 {
        const NEG_SIN_VECTOR4: Vector4 = Vector4::set(-1.0, 1.0, 1.0, 1.0);

        let (sin_angle, cos_angle) = angle_radian.sin_cos();

        let cos_sin_vector: Vector4 = Vector4::set(cos_angle, 0.0, -sin_angle, 0.0);

        let neg_sin_cos_vector: Vector4 = cos_sin_vector.zyxw() * NEG_SIN_VECTOR4;

        Matrix4x4::set(cos_sin_vector, Vector4::UP, neg_sin_cos_vector, Vector4::W)
    }

    #[inline]
    pub fn rotate_z_mat4(angle_radian: f32) -> Matrix4x4 {
        const NEG_SIN_VECTOR4: Vector4 = Vector4::set(-1.0, 1.0, 1.0, 1.0);

        let (sin_angle, cos_angle) = angle_radian.sin_cos();

        let cos_sin_vector: Vector4 = Vector4::set(cos_angle, sin_angle, 0.0, 0.0);

        let neg_sin_cos_vector: Vector4 = cos_sin_vector.yxzw() * NEG_SIN_VECTOR4;

        Matrix4x4::set(
            cos_sin_vector,
            neg_sin_cos_vector,
            Vector4::FORWARD,
            Vector4::W,
        )
    }

    #[inline]
    pub fn from_angle_axis_mat4(axis_normalized: Vector3, angle_radian: f32) -> Matrix4x4 {
        let (sin_angle, cos_angle) = angle_radian.sin_cos();

        let axis_sqr: Vector3 = axis_normalized * axis_normalized;

        let zy: f32 = axis_normalized.z() * axis_normalized.y();
        let xz: f32 = axis_normalized.x() * axis_normalized.z();
        let yx: f32 = axis_normalized.y() * axis_normalized.x();
        let axis_mul_sin: Vector3 = axis_normalized * sin_angle;

        let one_min_cos: f32 = 1.0 - cos_angle;

        let yx_theta: f32 = yx * one_min_cos;
        let xz_theta: f32 = xz * one_min_cos;
        let zy_theta: f32 = zy * one_min_cos;
        let axis_theta: Vector3 = axis_sqr * one_min_cos + cos_angle;

        Matrix4x4::set(
            Vector4::set(
                axis_theta.x(),
                yx_theta + axis_mul_sin.z(),
                xz_theta - axis_mul_sin.y(),
                0.0,
            ),
            Vector4::set(
                yx_theta - axis_mul_sin.z(),
                axis_theta.y(),
                zy_theta + axis_mul_sin.x(),
                0.0,
            ),
            Vector4::set(
                xz_theta + axis_mul_sin.y(),
                zy_theta - axis_mul_sin.x(),
                axis_theta.z(),
                0.0,
            ),
            Vector4::W,
        )
    }

    #[inline]
    pub fn from_quat_mat4(quaternion: Quaternion) -> Matrix4x4 {
        let quaternion2: Quaternion = quaternion + quaternion;

        let a: std::simd::f32x4 = quaternion.jiij().value * quaternion2.jjkk().value;

        let b: std::simd::f32x4 = quaternion.kwww().value * quaternion2.kkji().value;

        let ii2: f32 = quaternion.i() * quaternion2.i();

        let interleave = a.interleave(b);

        let first: Vector4 = Vector4 {
            value: interleave.0,
        };
        let second: Vector4 = Vector4 {
            value: interleave.1,
        };

        let column_0: Vector4 = Vector4::set(
            -first.y() - first.x() + 1.0,
            first.w() + first.z(),
            second.x() - second.y(),
            0.0,
        );
        let column_1: Vector4 = Vector4::set(
            first.z() - first.w(),
            -ii2 - first.y() + 1.0,
            second.w() + second.z(),
            0.0,
        );
        let column_2: Vector4 = Vector4::set(
            second.y() + second.x(),
            second.z() - second.w(),
            -ii2 - first.x() + 1.0,
            0.0,
        );

        Matrix4x4::set(column_0, column_1, column_2, Vector4::W)
    }

    #[inline]
    pub fn from_euler_mat4(euler_radian: Vector3, euler_order: EulerOrder) -> Matrix4x4 {
        let quaternion = from_euler_quat(euler_radian, euler_order);

        from_quat_mat4(quaternion)
    }

    #[inline]
    pub fn compose_trs_mat4(translation : Vector3, rotation : Quaternion, scale : Vector3) -> Matrix4x4{
        let translation_vector = Vector4{ value: translation.value } + Vector4::W;
        let scale_vector = Vector4::set(scale.x(), scale.y(), scale.z(), 0.0);
        let rotation_matrix = from_quat_mat4(rotation);

        Matrix4x4{
            column_x: rotation_matrix.column_x * scale_vector,
            column_y: rotation_matrix.column_y * scale_vector,
            column_z: rotation_matrix.column_z * scale_vector,
            column_w: translation_vector,
        }
    }

    #[inline]
    pub fn decompose_trs_mat4(matrix: Matrix4x4) -> (Vector3, Quaternion, Vector3) {
        let determinant = determinant_mat4(matrix);

        let scale = Vector3::set(
            length(matrix.column_x.value) * determinant.signum(),
            length(matrix.column_y.value),
            length(matrix.column_z.value)
        );

        let inverse_scale = Vector3{
            value: rcp(scale.value),
        };

        let column_x = matrix.column_x * inverse_scale.x();
        let column_y = matrix.column_y * inverse_scale.y();
        let column_z = matrix.column_z * inverse_scale.z();


        let quaternion = from_rotation_matrix_quat(Matrix4x4::set(column_x, column_y, column_z, Vector4::ZERO).into());

        let translation = matrix.column_w.trunc_vec3();

        (translation, quaternion, scale)
    }

    pub fn determinant_mat4(matrix: Matrix4x4) -> f32 {
        const NEG_Y_W : Vector4 = Vector4::set(1.0, -1.0, 1.0, -1.0);

        let column_y_yxxx = matrix.column_y.yxxx();
        let column_y_zzyy = matrix.column_y.zzyy();
        let column_y_wwwz = matrix.column_y.wwwz();

        let column_z_zyxx = matrix.column_z.zyxx();
        let column_z_zyyx = matrix.column_z.zyyx();
        let column_z_wwzw = matrix.column_z.wwzw();

        let column_w_xxzy = matrix.column_w.xxzy();
        let column_w_zyyx = matrix.column_w.zyyx();
        let column_w_wwzw = matrix.column_w.wwzw();

        let z_zyyx_mul_w_wwzw = column_z_zyyx * column_w_wwzw;
        let z_wwzw_mul_w_zyyx = column_z_wwzw * column_w_zyyx;
        let z_zyxx_mul_w_xxzy = column_z_zyxx * column_w_xxzy;

        let zw_high_low = z_zyxx_mul_w_xxzy.zwzw();

        let sub0 = z_zyyx_mul_w_wwzw - z_wwzw_mul_w_zyyx;
        let sub1 = zw_high_low - z_zyxx_mul_w_xxzy;

        let sub0_xxyz = sub0.xxyz();
        let sub01_xyyw = Vector4::set(sub0.y(), sub0.w(), sub0.w(), sub1.x());
        let sub01_xzww = Vector4::set(sub0.z(), sub1.x(), sub1.y(), sub1.y());

        let rhs = column_y_yxxx * sub0_xxyz;
        let lhs = column_y_zzyy * sub01_xyyw;
        let y_wwwz_mul_01_xzww = column_y_wwwz * sub01_xzww;

        let difference = rhs - lhs;
        let result = difference + y_wwwz_mul_01_xzww;

        let detcof = result * NEG_Y_W;

        dot(matrix.column_x.value, detcof.value)
    }

    pub fn inverse_mat4(matrix: Matrix4x4) -> Matrix4x4 {
        const NEG_X_Z: Vector4 = Vector4::set(-1.0, 1.0, -1.0, 1.0);
        const NEG_Y_W: Vector4 = Vector4::set(1.0, -1.0, 1.0, -1.0);

        let matrix_high_w_xxxz = Vector4::set(
            matrix.column_w.w(),
            matrix.column_w.w(),
            matrix.column_w.w(),
            matrix.column_z.w(),
        );
        let matrix_high_z_xxxz = Vector4::set(
            matrix.column_w.z(),
            matrix.column_w.z(),
            matrix.column_w.z(),
            matrix.column_z.z(),
        );
        let matrix_high_y_xxxz = Vector4::set(
            matrix.column_w.y(),
            matrix.column_w.y(),
            matrix.column_w.y(),
            matrix.column_z.y(),
        );
        let matrix_high_x_xxxz = Vector4::set(
            matrix.column_w.x(),
            matrix.column_w.x(),
            matrix.column_w.x(),
            matrix.column_z.x(),
        );

        let matrix_mid_w = Vector4::set(
            matrix.column_z.w(),
            matrix.column_z.w(),
            matrix.column_y.w(),
            matrix.column_y.w(),
        );
        let matrix_mid_z = Vector4::set(
            matrix.column_z.z(),
            matrix.column_z.z(),
            matrix.column_y.z(),
            matrix.column_y.z(),
        );
        let matrix_mid_y = Vector4::set(
            matrix.column_z.y(),
            matrix.column_z.y(),
            matrix.column_y.y(),
            matrix.column_y.y(),
        );
        let matrix_mid_x = Vector4::set(
            matrix.column_z.x(),
            matrix.column_z.x(),
            matrix.column_y.x(),
            matrix.column_y.x(),
        );

        let matrix_low_w_xzzz = Vector4::set(
            matrix.column_y.w(),
            matrix.column_x.w(),
            matrix.column_x.w(),
            matrix.column_x.w(),
        );
        let matrix_low_z_xzzz = Vector4::set(
            matrix.column_y.z(),
            matrix.column_x.z(),
            matrix.column_x.z(),
            matrix.column_x.z(),
        );
        let matrix_low_y_xzzz = Vector4::set(
            matrix.column_y.y(),
            matrix.column_x.y(),
            matrix.column_x.y(),
            matrix.column_x.y(),
        );
        let matrix_low_x_xzzz = Vector4::set(
            matrix.column_y.x(),
            matrix.column_x.x(),
            matrix.column_x.x(),
            matrix.column_x.x(),
        );

        let z_mul_w = {
            let rhs = matrix_mid_z * matrix_high_w_xxxz;
            let lhs = matrix_high_z_xxxz * matrix_mid_w;

            rhs - lhs
        };

        let y_mul_w = {
            let rhs = matrix_mid_y * matrix_high_w_xxxz;
            let lhs = matrix_high_y_xxxz * matrix_mid_w;

            rhs - lhs
        };

        let y_mul_z = {
            let rhs = matrix_mid_y * matrix_high_z_xxxz;
            let lhs = matrix_high_y_xxxz * matrix_mid_z;

            rhs - lhs
        };

        let x_mul_w = {
            let rhs = matrix_mid_x * matrix_high_w_xxxz;
            let lhs = matrix_high_x_xxxz * matrix_mid_w;

            rhs - lhs
        };

        let x_mul_z = {
            let rhs = matrix_mid_x * matrix_high_z_xxxz;
            let lhs = matrix_high_x_xxxz * matrix_mid_z;

            rhs - lhs
        };

        let x_mul_y = {
            let rhs = matrix_mid_x * matrix_high_y_xxxz;
            let lhs = matrix_high_x_xxxz * matrix_mid_y;

            rhs - lhs
        };

        let yzw_inverse = {
            let y_zw_sub_z_yw = (matrix_low_y_xzzz * z_mul_w) - (matrix_low_z_xzzz * y_mul_w);
            let yzw = y_zw_sub_z_yw + (matrix_low_w_xzzz * y_mul_z);

            NEG_Y_W * yzw
        };

        let xzw_inverse = {
            let x_zw_sub_z_xw = (matrix_low_x_xzzz * z_mul_w) - (matrix_low_z_xzzz * x_mul_w);
            let xzw = x_zw_sub_z_xw + (matrix_low_w_xzzz * x_mul_z);

            NEG_X_Z * xzw
        };

        let xyw_inverse = {
            let x_yw_sub_y_xw = (matrix_low_x_xzzz * y_mul_w) - (matrix_low_y_xzzz * x_mul_w);
            let xyw = x_yw_sub_y_xw + (matrix_low_w_xzzz * x_mul_y);

            NEG_Y_W * xyw
        };

        let xyz_inverse = {
            let x_yz_sub_y_xz = (matrix_low_x_xzzz * y_mul_z) - (matrix_low_y_xzzz * x_mul_z);
            let xyz = x_yz_sub_y_xz + (matrix_low_z_xzzz * x_mul_y);

            NEG_X_Z * xyz
        };

        let row = Vector4::set(
            yzw_inverse.x(),
            xzw_inverse.x(),
            xyw_inverse.x(),
            xyz_inverse.x(),
        );

        let dot = dot(matrix.column_x.value, row.value);

        let rcp_dot = Vector4::broadcast(dot.recip());

        Matrix4x4 {
            column_x: yzw_inverse * rcp_dot,
            column_y: xzw_inverse * rcp_dot,
            column_z: xyw_inverse * rcp_dot,
            column_w: xyz_inverse * rcp_dot,
        }
    }

    #[inline]
    pub fn multiply_point3_mat4(matrix : Matrix4x4, point3 : Vector3) -> Vector3{
        let x_axis = matrix.column_x.trunc_vec3() * point3.x();
        let y_axis = matrix.column_y.trunc_vec3() * point3.y();
        let z_axis = matrix.column_z.trunc_vec3() * point3.z();

        let w_axis = matrix.column_w.trunc_vec3() + Vector3::RIGHT;

        (x_axis + y_axis) + (z_axis + w_axis)
    }

    #[inline]
    pub fn multiply_vector3_mat4(matrix : Matrix4x4, vector3 : Vector3) -> Vector3{
        let x_axis = matrix.column_x.trunc_vec3() * vector3.x();
        let y_axis = matrix.column_y.trunc_vec3() * vector3.y();
        let z_axis = matrix.column_z.trunc_vec3() * vector3.z();

        x_axis + y_axis + z_axis
    }


    #[inline]
    pub fn project_point3_mat4(matrix: Matrix4x4, point3 : Vector3) -> Vector3{
        let x_axis = matrix.column_x * point3.x();
        let y_axis = matrix.column_y * point3.y();
        let z_axis = matrix.column_z * point3.z();
        let w_axis = matrix.column_w;

        let result = (x_axis + y_axis) + (z_axis + w_axis);

        (result * Vector4::broadcast(result.w().recip())).trunc_vec3()
    }

}