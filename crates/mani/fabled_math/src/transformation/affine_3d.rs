use crate::{Matrix3x3, Vector3, Quaternion};

use crate::quaternion_math::{rotate_x, rotate_y, rotate_z, to_rotation_matrix};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Affine3 {
    pub translation: Vector3,
    pub matrix3: Matrix3x3,
}


impl Affine3{

    pub const IDENTITY : Affine3 = Affine3{ translation: Vector3::ZERO, matrix3: Matrix3x3::IDENTITY };

    pub const ZERO : Affine3 = Affine3{ translation: Vector3::ZERO, matrix3: Matrix3x3::ZERO };



}

impl Affine3 {
    pub fn new(translation: Vector3, euler_radians: Vector3) -> Affine3 {

        let x = rotate_x(euler_radians.x());
        let y = rotate_y(euler_radians.y());
        let z = rotate_z(euler_radians.z());

        let quaternion = x * y * z;

        let rotation_matrix = to_rotation_matrix(quaternion);

        Affine3{
            translation,
            matrix3:rotation_matrix
        }
    }


    pub fn from_scale(scale : Vector3) -> Affine3{

        let x_scaled_column = Vector3::set(scale.x(), 0.0, 0.0);
        let y_scaled_column = Vector3::set(0.0, scale.y(), 0.0);
        let z_scaled_column = Vector3::set(0.0, 0.0, scale.z());

        let diagonal_matrix = Matrix3x3::set_from_columns(x_scaled_column, y_scaled_column, z_scaled_column);

        Affine3 { translation: Vector3::ZERO, matrix3: diagonal_matrix }
    }

    pub fn from_quaternion(quaternion : Quaternion) -> Affine3{
        let rotation_matrix = to_rotation_matrix(quaternion);

        Affine3 { translation: Vector3::ZERO, matrix3: rotation_matrix }
    }

}

#[cfg(test)]
mod affine3d_test {
    use crate::{Affine3, Vector3};

    #[test]
    fn affine_creation_test() {
        let affine3d = Affine3::new(
            Vector3::ZERO,
            Vector3::set(
                13.5f32.to_radians(),
                45.0f32.to_radians(),
                90.0f32.to_radians(),
            ),
        );
        println!("{}", affine3d.matrix3);
        println!("{}", affine3d.translation);
    }
}
