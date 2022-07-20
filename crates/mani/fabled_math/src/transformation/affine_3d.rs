use crate::{rotate_x, rotate_y, to_rotation_matrix, Matrix3x3, Quaternion, Vector3};

#[derive(Default)]
pub struct Affine3 {
    pub translation: Vector3,
    pub matrix3: Matrix3x3,
}


impl Affine3 {
    // euler is in radians
    #[rustfmt::skip]
    pub fn new(translation: Vector3, euler_radians: Vector3) -> Self {


        let x = rotate_x(euler_radians[0]);
        let y = rotate_y(euler_radians[1]);
        let z = rotate_y(euler_radians[2]);

        let quaternion = x*y* z;


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
    }
}
