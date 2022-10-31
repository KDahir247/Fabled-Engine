use crate::Vector3;

use crate::matrix3x3_math::transpose;

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Matrix3x3 {
    pub column_x: Vector3,
    pub column_y: Vector3,
    pub column_z: Vector3,
}

#[rustfmt::skip]
impl Default for Matrix3x3 {
    fn default() -> Self {
        Self {
            column_x: Vector3::set(1.0, 0.0, 0.0),
            column_y: Vector3::set(0.0, 1.0, 0.0),
            column_z: Vector3::set(0.0, 0.0, 1.0)
        }
    }
}


impl Matrix3x3 {
    pub const IDENTITY: Matrix3x3 = Matrix3x3 {
        column_x: Vector3::set(1.0, 0.0, 0.0),
        column_y: Vector3::set(0.0, 1.0, 0.0),
        column_z: Vector3::set(0.0, 0.0, 1.0)
    };


    pub const ZERO : Matrix3x3 = Matrix3x3{
        column_x: Vector3::ZERO,
        column_y: Vector3::ZERO,
        column_z: Vector3::ZERO,
    };
}


impl Matrix3x3 {
    #[inline]
    pub const fn set_from_columns(
        column_x: Vector3,
        column_y: Vector3,
        column_z: Vector3,
    ) -> Matrix3x3 {
        Matrix3x3 { column_x,column_y,column_z}
    }

    #[inline]
    pub const fn splat(val: f32) -> Matrix3x3 {
        let splat_vector3 : Vector3 = Vector3::splat(val);

        Matrix3x3 {
            column_x: splat_vector3,
            column_y: splat_vector3,
            column_z: splat_vector3,
        }
    }

    #[inline]
    pub const fn to_primitive(self) -> [f32; 9] {
        let x_column = self.column_x;
        let y_column = self.column_y;
        let z_column = self.column_z;

        [
            x_column.x(),
            y_column.x(),
            z_column.x(),

            x_column.y(),
            y_column.y(),
            z_column.y(),

            x_column.z(),
            y_column.z(),
            z_column.z(),
        ]
    }

    #[inline]
    pub const fn to_diagonal(self) -> Vector3 {
        Vector3::set(self.column_x.x(), self.column_y.y(), self.column_z.z())
    }
}


impl Display for Matrix3x3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "Matrix3x3\n[\n\t{}\n\t{}\n\t{}\n]",
            self.column_x, self.column_y, self.column_z,
        )
    }
}

// Component-Wise
impl Add<f32> for Matrix3x3 {
    type Output = Matrix3x3;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        let splat_vector3 = Vector3::splat(rhs);

        Matrix3x3 {
            column_x: self.column_x + splat_vector3,
            column_y: self.column_y + splat_vector3,
            column_z: self.column_z + splat_vector3,
        }
    }
}

impl AddAssign<f32> for Matrix3x3 {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        let splat_vector3 = Vector3::splat(rhs);

        self.column_x += splat_vector3;
        self.column_y += splat_vector3;
        self.column_z += splat_vector3;
    }
}

impl Sub<f32> for Matrix3x3 {
    type Output = Matrix3x3;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        let splat_vector3 = Vector3::splat(rhs);

        Matrix3x3 {
            column_x: self.column_x - splat_vector3,
            column_y: self.column_y - splat_vector3,
            column_z: self.column_z - splat_vector3,
        }
    }
}

impl SubAssign<f32> for Matrix3x3 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        let splat_vector3 = Vector3::splat(rhs);

        self.column_x -= splat_vector3;
        self.column_y -= splat_vector3;
        self.column_z -= splat_vector3;
    }
}

impl Mul<f32> for Matrix3x3 {
    type Output = Matrix3x3;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        let splat_vector3 = Vector3::splat(rhs);

        Matrix3x3 {
            column_x: self.column_x * splat_vector3,
            column_y: self.column_y * splat_vector3,
            column_z: self.column_z * splat_vector3,
        }
    }
}

impl MulAssign<f32> for Matrix3x3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        let splat_vector3 = Vector3::splat(rhs);

        self.column_x *= splat_vector3;
        self.column_y *= splat_vector3;
        self.column_z *= splat_vector3;
    }
}

// Matrix-Wise
impl Add for Matrix3x3 {
    type Output = Matrix3x3;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Matrix3x3 {
            column_x: self.column_x + rhs.column_x,
            column_y: self.column_y + rhs.column_y,
            column_z: self.column_z + rhs.column_z,
        }
    }
}

impl AddAssign for Matrix3x3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.column_x += rhs.column_x;
        self.column_y += rhs.column_y;
        self.column_z += rhs.column_z;
    }
}

impl Sub for Matrix3x3 {
    type Output = Matrix3x3;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Matrix3x3 {
            column_x: self.column_x - rhs.column_x,
            column_y: self.column_y - rhs.column_y,
            column_z: self.column_z - rhs.column_z,
        }
    }
}

impl SubAssign for Matrix3x3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.column_x -= rhs.column_x;
        self.column_y -= rhs.column_y;
        self.column_z -= rhs.column_z;
    }
}

impl Mul for Matrix3x3 {
    type Output = Matrix3x3;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let transposed_matrix: Matrix3x3 = transpose(rhs);

        Matrix3x3 {
            column_x: self.column_x * transposed_matrix.column_x,
            column_y: self.column_y * transposed_matrix.column_y,
            column_z: self.column_z * transposed_matrix.column_z,
        }
    }
}

impl MulAssign for Matrix3x3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        let transposed_matrix = transpose(rhs);
        self.column_x *= transposed_matrix.column_x;
        self.column_y *= transposed_matrix.column_y;
        self.column_z *= transposed_matrix.column_z;
    }
}

pub mod matrix3x3_math {
    use crate::{Matrix3x3, Vector2, Vector3, EulerOrder};

    use crate::vector_math::{dot, cross};

    use crate::math_trait::Swizzles3;

    #[inline]
    pub fn transpose(matrix_3x3: Matrix3x3) -> Matrix3x3 {
        let x_column = matrix_3x3.column_x;
        let y_column = matrix_3x3.column_y;
        let z_column = matrix_3x3.column_z;

        Matrix3x3 {
            column_x: Vector3::set(x_column.x(), y_column.x(), z_column.x()),
            column_y: Vector3::set(x_column.y(), y_column.y(), z_column.y()),
            column_z: Vector3::set(x_column.z(), y_column.z(), z_column.z()),
        }
    }

    #[inline]
    pub fn from_scale(scalar_vector: Vector2) -> Matrix3x3 {
        Matrix3x3::set_from_columns(
            Vector3::set(scalar_vector.x(), 0.0, 0.0),
            Vector3::set(0.0, scalar_vector.y(), 0.0),
            Vector3::FORWARD,
        )
    }

    #[inline]
    pub fn from_translation(translation_vector: Vector3) -> Matrix3x3 {
        Matrix3x3::set_from_columns(Vector3::RIGHT, Vector3::UP, translation_vector)
    }

    #[inline]
    pub fn rotate_x(angle_radians: f32) -> Matrix3x3 {
        const NEG_SIN_VECTOR3: Vector3 = Vector3::set(1.0, -1.0, 1.0);

        let (sin_angle, cos_angle) = angle_radians.sin_cos();

        let cos_sin_vector: Vector3 = Vector3::set(0.0, cos_angle, sin_angle);

        let neg_sin_cos_vector: Vector3 = cos_sin_vector.xzy() * NEG_SIN_VECTOR3;

        Matrix3x3::set_from_columns(Vector3::RIGHT, cos_sin_vector, neg_sin_cos_vector)
    }

    #[inline]
    pub fn rotate_y(angle_radians: f32) -> Matrix3x3 {
        const NEG_SIN_VECTOR3: Vector3 = Vector3::set(-1.0, 1.0, 1.0);

        let (sin_angle, cos_angle) = angle_radians.sin_cos();

        let cos_sin_vector: Vector3 = Vector3::set(cos_angle, 0.0, -sin_angle);

        let neg_sin_cos_vector: Vector3 = cos_sin_vector.zyx() * NEG_SIN_VECTOR3;

        Matrix3x3::set_from_columns(cos_sin_vector, Vector3::UP, neg_sin_cos_vector)
    }

    #[inline]
    pub fn rotate_z(angle_radians: f32) -> Matrix3x3 {
        const NEG_SIN_VECTOR3: Vector3 = Vector3::set(-1.0, 1.0, 1.0);

        let (sin_angle, cos_angle) = angle_radians.sin_cos();

        let cos_sin_vector: Vector3 = Vector3::set(cos_angle, sin_angle, 0.0);

        let neg_sin_cos_vector: Vector3 = cos_sin_vector.yxz() * NEG_SIN_VECTOR3;

        Matrix3x3::set_from_columns(cos_sin_vector, neg_sin_cos_vector, Vector3::FORWARD)
    }

    #[inline]
    pub fn from_angle_axis(axis_normalized: Vector3, angle_radians: f32) -> Matrix3x3 {
        let (sin_angle, cos_angle) = angle_radians.sin_cos();

        let axis_sqr = axis_normalized * axis_normalized;

        let zy = axis_normalized.z() * axis_normalized.y();
        let xz = axis_normalized.x() * axis_normalized.z();
        let yx = axis_normalized.y() * axis_normalized.x();
        let axis_mul_sin = axis_normalized * sin_angle;

        let one_min_cos = 1.0 - cos_angle;

        let yx_theta = yx * one_min_cos;
        let xz_theta = xz * one_min_cos;
        let zy_theta = zy * one_min_cos;

        let axisx_theta = axis_sqr.x() * one_min_cos;
        let axisy_theta = axis_sqr.y() * one_min_cos;
        let axisz_theta = axis_sqr.z() * one_min_cos;

        Matrix3x3::set_from_columns(
            Vector3::set(
                axisx_theta + cos_angle,
                yx_theta + axis_mul_sin.z(),
                xz_theta - axis_mul_sin.y(),
            ),
            Vector3::set(
                yx_theta - axis_mul_sin.z(),
                axisy_theta + cos_angle,
                zy_theta + axis_mul_sin.x(),
            ),
            Vector3::set(
                xz_theta + axis_mul_sin.y(),
                zy_theta - axis_mul_sin.x(),
                axisz_theta + cos_angle,
            ),
        )
    }


    #[inline]
    pub fn from_scale_angle_translation(
        scalar_vec: Vector2,
        angle_radians: f32,
        translation: Vector2,
    ) -> Matrix3x3 {
        let (angle_sin, angle_cos) = angle_radians.sin_cos();

        let extended_translation = Vector3::set(translation.x(), translation.y(), 1.0);

        Matrix3x3::set_from_columns(
            Vector3::set(angle_cos * scalar_vec.x(), angle_sin, 0.0),
            Vector3::set(-angle_sin, angle_cos * scalar_vec.y(), 0.0),
            extended_translation,
        )
    }

    #[inline]
    pub fn from_angle(angle_radians: f32) -> Matrix3x3 {
        let (sin_angle, cos_angle) = angle_radians.sin_cos();

        Matrix3x3::set_from_columns(
            Vector3::set(cos_angle, sin_angle, 0.0),
            Vector3::set(-sin_angle, cos_angle, 0.0),
            Vector3::ZERO,
        )
    }

    pub fn from_euler(euler_radians : Vector3, euler_order : EulerOrder) -> Matrix3x3{
        let quaternion = crate::quaternion_math::from_euler(euler_radians, euler_order);

        let rotation_matrix = crate::quaternion_math::to_rotation_matrix(quaternion);

        rotation_matrix
    }

    pub fn determinant(matrix: Matrix3x3) -> f32 {
        dot(matrix.column_z.value, cross(matrix.column_x.value, matrix.column_y.value))
    }

    pub fn inverse(matrix: Matrix3x3) -> Matrix3x3 {
        let x = Vector3 { value: cross(matrix.column_y.value, matrix.column_z.value) };
        let y = Vector3 { value: cross(matrix.column_z.value, matrix.column_x.value) };
        let z = Vector3 { value: cross(matrix.column_x.value, matrix.column_y.value) };

        let inverse_determinant = determinant(matrix).recip();

        let cofactor_matrix = Matrix3x3::set_from_columns(x, y, z);
        let adjugate_matrix = transpose(cofactor_matrix);

        adjugate_matrix * inverse_determinant
    }
}