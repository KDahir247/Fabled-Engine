use crate::quaternion_math::{rotate_x, rotate_y, rotate_z, to_rotation_matrix};
use crate::{Matrix3x3, Vector3};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Affine3 {
    pub translation: Vector3,
    pub matrix3: Matrix3x3,
}


impl Affine3 {
    #[rustfmt::skip]
    pub fn new(translation: Vector3, euler_radians: Vector3) -> Self {

        let x = rotate_x(euler_radians.x());
        let y = rotate_y(euler_radians.y());
        let z = rotate_z(euler_radians.z());

        let quaternion = x * y * z;

        let rotation_matrix = to_rotation_matrix(quaternion);

        Self{
            translation,
            matrix3:rotation_matrix
        }
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
