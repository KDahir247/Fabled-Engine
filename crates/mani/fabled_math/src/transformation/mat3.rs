use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use crate::matrix3x3_math::transpose;
use crate::Vector3;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Matrix3x3 {
    pub value: std::simd::f32x16,
}

#[rustfmt::skip]
impl Default for Matrix3x3 {
    fn default() -> Self {
        Self {
            value: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0
            ].into(),
        }
    }
}

impl Matrix3x3 {
    #[rustfmt::skip]
    #[inline]
    pub fn set_from_columns(
        x_column: Vector3,
        y_column: Vector3,
        z_column: Vector3,
    ) -> Matrix3x3 {
        Matrix3x3 {
            value: [
                x_column[0], x_column[1], x_column[2], 0.0,
                y_column[0], y_column[1], y_column[2], 0.0,
                z_column[0], z_column[1], z_column[2], 0.0,
                0.0, 0.0, 0.0, 0.0
            ].into(),
        }
    }

    #[rustfmt::skip]
    #[inline]
    pub fn set_from_rows(
        x_rows: Vector3,
        y_rows: Vector3,
        z_rows: Vector3
    ) -> Matrix3x3 {
        Matrix3x3{
            value: [
                x_rows[0], y_rows[0], z_rows[0], 0.0,
                x_rows[1], y_rows[1], z_rows[1], 0.0,
                x_rows[2], y_rows[2], z_rows[2], 0.0,
                0.0, 0.0, 0.0, 0.0
            ].into(),
        }
    }


    #[inline]
    pub fn splat(val: f32) -> Matrix3x3 {
        Matrix3x3 {
            value: std::simd::f32x16::splat(val),
        }
    }


    #[inline]
    pub const fn to_primitive(self) -> [f32; 16] {
        self.value.to_array()
    }
}

impl Matrix3x3 {
    #[rustfmt::skip]
    pub const IDENTITY: Matrix3x3 = Matrix3x3 {
        value: std::simd::f32x16::from_array([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0])
    };
}

#[rustfmt::skip]
impl From<[f32; 16]> for Matrix3x3 {
    fn from(matrix: [f32; 16]) -> Self {
        
        Self {
            value: [
                matrix[0], matrix[1], matrix[2], 0.0, 
                matrix[4], matrix[5], matrix[6], 0.0, 
                matrix[8], matrix[9], matrix[10], 0.0, 
                0.0, 0.0, 0.0, 0.0
            ].into(),
        }
    }
}

#[rustfmt::skip]
impl From<[f32; 9]> for Matrix3x3 {
    fn from(matrix: [f32; 9]) -> Self {
   
        Self {
            value: [
                matrix[0], matrix[1], matrix[2], 0.0, 
                matrix[3], matrix[4], matrix[5], 0.0, 
                matrix[6], matrix[7], matrix[8], 0.0,
                0.0, 0.0, 0.0, 0.0
            ].into(),
        }
    }
}

impl Display for Matrix3x3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let array_representation: [f32; 16] = *self.value.as_array();

        write!(
            f,
            "Matrix3x3\n[\n\t{}, {}, {}\n\t{}, {}, {},\n\t{}, {}, {}\n]",
            array_representation[0],
            array_representation[1],
            array_representation[2],
            array_representation[4],
            array_representation[5],
            array_representation[6],
            array_representation[8],
            array_representation[9],
            array_representation[10]
        )
    }
}

// Component-Wise
impl Add<f32> for Matrix3x3 {
    type Output = Matrix3x3;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        let splat_matrix_3x3 = Matrix3x3::splat(rhs);

        Matrix3x3 {
            value: self.value + splat_matrix_3x3.value,
        }
    }
}

impl AddAssign<f32> for Matrix3x3 {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        let splat_matrix_3x3 = Matrix3x3::splat(rhs);

        self.value += splat_matrix_3x3.value;
    }
}

impl Sub<f32> for Matrix3x3 {
    type Output = Matrix3x3;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        let splat_matrix_3x3 = Matrix3x3::splat(rhs);

        Matrix3x3 {
            value: self.value - splat_matrix_3x3.value,
        }
    }
}

impl SubAssign<f32> for Matrix3x3 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        let splat_matrix_3x3 = Matrix3x3::splat(rhs);

        self.value -= splat_matrix_3x3.value;
    }
}

impl Mul<f32> for Matrix3x3 {
    type Output = Matrix3x3;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        let matrix3x3_of_rhs = Matrix3x3::splat(rhs);

        Matrix3x3 {
            value: self.value * matrix3x3_of_rhs.value,
        }
    }
}

impl MulAssign<f32> for Matrix3x3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        let matrix3x3_of_rhs = Matrix3x3::splat(rhs);

        self.value *= matrix3x3_of_rhs.value;
    }
}


// Matrix-Wise
impl Add for Matrix3x3 {
    type Output = Matrix3x3;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Matrix3x3 {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign for Matrix3x3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value
    }
}

impl Sub for Matrix3x3 {
    type Output = Matrix3x3;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Matrix3x3 {
            value: self.value - rhs.value,
        }
    }
}

impl SubAssign for Matrix3x3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value
    }
}

impl Mul for Matrix3x3 {
    type Output = Matrix3x3;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let transposed_rhs: Matrix3x3 = transpose(rhs);

        Matrix3x3 {
            value: self.value * transposed_rhs.value,
        }
    }
}

impl MulAssign for Matrix3x3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= transpose(rhs).value;
    }
}

pub mod matrix3x3_math {
    use crate::{reverse, Matrix3x3, Vector2, Vector3};

    #[inline]
    pub fn transpose(matrix_3x3: Matrix3x3) -> Matrix3x3 {
        const TRANSPOSE_INDICES: [usize; 16] =
            [0, 4, 8, 12, 1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15];

        let transposed_value = std::simd::simd_swizzle!(matrix_3x3.value, TRANSPOSE_INDICES);

        Matrix3x3 {
            value: transposed_value,
        }
    }

    #[inline]
    pub fn from_scale(scalar_vector: Vector2) -> Matrix3x3 {
        let extended_scalar = scalar_vector.extend_vec3();

        Matrix3x3::set_from_columns(
            extended_scalar * Vector3::RIGHT,
            extended_scalar * Vector3::UP,
            Vector3::FORWARD,
        )
    }

    #[inline]
    pub fn from_translation(translation_vector: Vector3) -> Matrix3x3 {
        Matrix3x3::set_from_columns(Vector3::RIGHT, Vector3::UP, translation_vector)
    }

    #[inline]
    pub fn rotate_x(angle_radians: f32) -> Matrix3x3 {
        let (sin_angle, cos_angle) = angle_radians.sin_cos();

        let cos_sin_vector: Vector3 = Vector3::set(0.0, cos_angle, sin_angle);

        let neg_sin_cos_vector: Vector3 = Vector3 {
            value: reverse(cos_sin_vector.value) * Vector3::DOWN.value,
        };

        Matrix3x3::set_from_columns(Vector3::RIGHT, cos_sin_vector, neg_sin_cos_vector)
    }

    #[inline]
    pub fn rotate_y(angle_radians: f32) -> Matrix3x3 {
        let (sin_angle, cos_angle) = angle_radians.sin_cos();

        let cos_sin_vector: Vector3 = Vector3::set(cos_angle, 0.0, -sin_angle);

        let neg_sin_cos_vector: Vector3 = Vector3 {
            value: reverse(cos_sin_vector.value) * Vector3::LEFT.value,
        };

        Matrix3x3::set_from_columns(cos_sin_vector, Vector3::UP, neg_sin_cos_vector)
    }

    #[inline]
    pub fn rotate_z(angle_radians: f32) -> Matrix3x3 {
        let (sin_angle, cos_angle) = angle_radians.sin_cos();

        let cos_sin_vector: Vector3 = Vector3::set(cos_angle, sin_angle, 0.0);

        let neg_sin_cos_vector: Vector3 = Vector3 {
            value: reverse(cos_sin_vector.value) * Vector3::LEFT.value,
        };

        Matrix3x3::set_from_columns(cos_sin_vector, neg_sin_cos_vector, Vector3::FORWARD)
    }

    #[inline]
    pub fn from_angle_axis(axis_normalized: Vector3, angle_radians: f32) -> Matrix3x3 {
        let (sin_angle, cos_angle) = angle_radians.sin_cos();

        let axis_sqr = axis_normalized * axis_normalized;

        let xz = axis_normalized[0] * axis_normalized[2];

        let yx = axis_normalized[1] * axis_normalized[0];

        let zy = axis_normalized[2] * axis_normalized[1];

        let one_min_cos = 1.0 - cos_angle;

        let axis_mul_sin = axis_normalized * sin_angle;

        Matrix3x3::set_from_columns(
            Vector3::set(
                cos_angle + axis_sqr[0] * one_min_cos,
                yx * one_min_cos + axis_mul_sin[2],
                xz * one_min_cos - axis_mul_sin[1],
            ),
            Vector3::set(
                yx * one_min_cos - axis_mul_sin[2],
                cos_angle + axis_sqr[1] * one_min_cos,
                zy * one_min_cos + axis_mul_sin[0],
            ),
            Vector3::set(
                xz * one_min_cos + axis_mul_sin[1],
                zy * one_min_cos - axis_mul_sin[0],
                cos_angle + axis_sqr[2] * one_min_cos,
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

        let extended_translation = translation.extend_vec3() + Vector3::FORWARD;

        Matrix3x3::set_from_columns(
            Vector3::set(angle_cos * scalar_vec.value[0], angle_sin, 0.0),
            Vector3::set(-angle_sin, angle_cos * scalar_vec.value[1], 0.0),
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

    pub fn determinant(_matrix: Matrix3x3) -> f32 {
        todo!()
    }

    pub fn inverse(_matrix: Matrix3x3) -> Matrix3x3 {
        todo!()
    }
}


pub struct SOAMatrix3x3<const N: usize> {
    pub value: [Matrix3x3; N],
}

#[cfg(test)]
mod matrix_3x3_test {
    use crate::matrix3x3_math::{from_angle_axis, transpose};
    use crate::{Matrix3x3, Vector3};

    #[test]
    fn matrix3x3_transpose_test() {
        let matrix = Matrix3x3::set_from_rows(
            Vector3::set(0.548_919, -0.8138058, 0.190_809),
            Vector3::set(0.8007619, 0.4465074, -0.3992637),
            Vector3::set(0.2397255, 0.371_956, 0.8967611),
        );
        println!("original {}", matrix);

        let transposed = transpose(matrix);

        println!("transposed {}", transposed);

        let reverted = transpose(transposed);

        assert_eq!(matrix, reverted);
    }

    #[test]
    fn matrix3x3_angle_axis_test() {
        let m = from_angle_axis(
            Vector3::set(0.566_138_5, 0.226_455_4, 0.792_593_9),
            56.11f32.to_radians(),
        );


        println!("{}", m);


        let m2 = Matrix3x3::set_from_columns(
            Vector3::set(2.0, 11.0, 2.0),
            Vector3::set(3.0, 8.0, 5.0),
            Vector3::set(-4.0, 7.0, 3.0),
        );


        let v1 = Vector3::set(3.0, 7.0, 5.0);

        let res = m2 * v1;

        println!("{}", res);
    }
}
