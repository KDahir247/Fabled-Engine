use std::fmt::Display;

use crate::Vector4;

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

