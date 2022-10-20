use crate::Vector4;

use crate::matrix4x4_math::transpose;

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};

use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Matrix4x4 {
    pub column_x : Vector4,
    pub column_y : Vector4,
    pub column_z : Vector4,
    pub column_w : Vector4,


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

impl Matrix4x4{

    #[inline]
    pub const fn set_from_column(column_x : Vector4, column_y : Vector4, column_z : Vector4, column_w : Vector4) -> Matrix4x4{
        Matrix4x4 { column_x, column_y, column_z, column_w }
    }


    #[inline]
    pub const fn splat(val : f32) -> Matrix4x4{
        let splat_vector4 : Vector4 = Vector4::splat(val);

        Matrix4x4 {
            column_x: splat_vector4,
            column_y: splat_vector4,
            column_z: splat_vector4,
            column_w: splat_vector4
        }
    }

    #[inline]
    pub const fn to_primitive(self) -> [f32; 16]{
        let x_column = self.column_x;
        let y_column = self.column_y;
        let z_column = self.column_y;
        let w_column = self.column_w;

        [
            x_column.x(),
            y_column.x(),
            z_column.x(),
            w_column.x(),

            x_column.y(),
            y_column.y(),
            z_column.y(),
            w_column.y(),

            x_column.z(),
            y_column.z(),
            z_column.z(),
            w_column.z(),

            x_column.w(),
            y_column.w(),
            z_column.w(),
            w_column.w()
        ]
    }

    #[inline]
    pub const fn to_diagonal(self) -> Vector4{
        Vector4::set(self.column_x.x(), self.column_y.y(), self.column_z.z(), self.column_w.w())
    }
}

impl Matrix4x4{
    #[rustfmt::skip]
    pub const IDENTITY : Matrix4x4 = Matrix4x4{
        column_x: Vector4::set(1.0, 0.0, 0.0, 0.0),
        column_y: Vector4::set(0.0, 1.0, 0.0, 0.0),
        column_z: Vector4::set(0.0, 0.0, 1.0, 0.0),
        column_w: Vector4::set(0.0, 0.0, 0.0, 1.0),
    };
}

impl Display for Matrix4x4{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
        "Matrix4x4\n[\n\t{}\n\t{}\n\t{}\n\t{}\n]",
        self.column_x, self.column_y, self.column_z, self.column_w
        )
    }
}

// Component-Wise
impl Add<f32> for Matrix4x4{
    type Output = Matrix4x4;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        let splat_vector4 = Vector4::splat(rhs);

        Matrix4x4{
            column_x: self.column_x + splat_vector4,
            column_y: self.column_y + splat_vector4,
            column_z: self.column_z + splat_vector4,
            column_w: self.column_w + splat_vector4,
        }
    }
}

impl AddAssign<f32> for Matrix4x4{
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        let splat_vector4 = Vector4::splat(rhs);

        self.column_x += splat_vector4;
        self.column_y += splat_vector4;
        self.column_z += splat_vector4;
        self.column_w += splat_vector4;
    }
}

impl Sub<f32> for Matrix4x4{
    type Output = Matrix4x4;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        let splat_vector4 = Vector4::splat(rhs);

        Matrix4x4{
            column_x: self.column_x - splat_vector4,
            column_y: self.column_y - splat_vector4,
            column_z: self.column_z - splat_vector4,
            column_w: self.column_w - splat_vector4,
        }
    }
}

impl SubAssign<f32> for Matrix4x4{
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        let splat_vector4 = Vector4::splat(rhs);

        self.column_x -= splat_vector4;
        self.column_y -= splat_vector4;
        self.column_z -= splat_vector4;
        self.column_w -= splat_vector4;
    }
}

impl Mul<f32> for Matrix4x4{
    type Output = Matrix4x4;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        let splat_vector4 = Vector4::splat(rhs);

        Matrix4x4{
            column_x: self.column_x * splat_vector4,
            column_y: self.column_y * splat_vector4,
            column_z: self.column_z * splat_vector4,
            column_w: self.column_w * splat_vector4,
        }
    }
}

impl MulAssign<f32> for Matrix4x4{
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        let splat_vector4 = Vector4::splat(rhs);

        self.column_x *= splat_vector4;
        self.column_y *= splat_vector4;
        self.column_z *= splat_vector4;
        self.column_w *= splat_vector4;

    }
}

// Matrix-Wise
impl Add for Matrix4x4{
    type Output = Matrix4x4;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Matrix4x4{
            column_x: self.column_x + rhs.column_x,
            column_y: self.column_y + rhs.column_y,
            column_z: self.column_z + rhs.column_z,
            column_w: self.column_w + rhs.column_w,
        }
    }
}

impl AddAssign for Matrix4x4{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.column_x += rhs.column_x;
        self.column_y += rhs.column_y;
        self.column_z += rhs.column_z;
        self.column_w += rhs.column_w;
    }
}

impl Sub for Matrix4x4{
    type Output = Matrix4x4;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Matrix4x4{
            column_x: self.column_x + rhs.column_x,
            column_y: self.column_y + rhs.column_y,
            column_z: self.column_z + rhs.column_z,
            column_w: self.column_w + rhs.column_w,
        }
    }
}

impl SubAssign for Matrix4x4{
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
        let transposed_matrix : Matrix4x4 = transpose(rhs);

        Matrix4x4{
            column_x: self.column_x * transposed_matrix.column_x,
            column_y: self.column_y * transposed_matrix.column_y,
            column_z: self.column_z * transposed_matrix.column_z,
            column_w: self.column_w * transposed_matrix.column_w,
        }
    }
}

impl MulAssign for Matrix4x4 {
    fn mul_assign(&mut self, rhs: Self) {
        let transposed_matrix: Matrix4x4 = transpose(rhs);

        self.column_x *= transposed_matrix.column_x;
        self.column_y *= transposed_matrix.column_y;
        self.column_z *= transposed_matrix.column_z;
        self.column_w *= transposed_matrix.column_w;

    }
}

pub mod matrix4x4_math{

    use crate::{Matrix4x4, Vector4};

    #[inline]
    pub fn transpose(matrix_4x4 : Matrix4x4) -> Matrix4x4{
        let x_column = matrix_4x4.column_x;
        let y_column = matrix_4x4.column_y;
        let z_column = matrix_4x4.column_z;
        let w_column = matrix_4x4.column_w;

        Matrix4x4 {
            column_x: Vector4::set(x_column.x(), y_column.x(), z_column.x(), w_column.x()),
            column_y: Vector4::set(x_column.y(), y_column.y(), z_column.y(), w_column.y()),
            column_z: Vector4::set(x_column.z(), y_column.z(), z_column.z(), w_column.z()),
            column_w: Vector4::set(x_column.w(), y_column.w(), z_column.w(), w_column.w())
        }
    }
}