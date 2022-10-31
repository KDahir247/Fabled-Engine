use std::{
    fmt::Display,
    ops::{Mul, MulAssign},
};

use crate::{Matrix3x3, Vector3};

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Affine3 {
    pub translation: Vector3,
    pub matrix3: Matrix3x3,
}

impl Affine3 {
    pub const IDENTITY: Affine3 = Affine3 {
        translation: Vector3::ZERO,
        matrix3: Matrix3x3::IDENTITY,
    };

    pub const ZERO: Affine3 = Affine3 {
        translation: Vector3::ZERO,
        matrix3: Matrix3x3::ZERO,
    };
}


impl Affine3 {
    #[inline(always)]
    pub const fn set(
        column_x: Vector3,
        column_y: Vector3,
        column_z: Vector3,
        column_w: Vector3,
    ) -> Affine3 {
        Affine3 {
            translation: column_w,
            matrix3: Matrix3x3::set(column_x, column_y, column_z),
        }
    }

    #[inline(always)]
    pub const fn new(translation: Vector3, matrix3: Matrix3x3) -> Affine3 {
        Affine3 {
            translation,
            matrix3,
        }
    }

    #[inline]
    pub const fn broadcast(val: f32) -> Affine3 {
        Affine3 {
            translation: Vector3::broadcast(val),
            matrix3: Matrix3x3::broadcast(val),
        }
    }

    #[inline]
    pub const fn splat(val: Vector3) -> Affine3 {
        Affine3 {
            translation: val,
            matrix3: Matrix3x3 {
                column_x: val,
                column_y: val,
                column_z: val,
            },
        }
    }

    #[inline]
    pub const fn from_primitive(array: [f32; 12]) -> Affine3 {
        let rotation_matrix: [f32; 9] = [
            array[0], array[1], array[2], array[3], array[4], array[5], array[6], array[7],
            array[8],
        ];

        let translation: [f32; 3] = [array[9], array[10], array[11]];

        Affine3 {
            translation: Vector3::from_primitive(translation),
            matrix3: Matrix3x3::from_primitive(rotation_matrix),
        }
    }

    #[rustfmt::skip]
    #[inline]
    pub const fn to_primitive(self) -> [f32; 12] {
        let translation: [f32; 3] = self.translation.to_primitive();
        let rotation_matrix: [f32; 9] = self.matrix3.to_primitive();

        [
            rotation_matrix[0], rotation_matrix[1], rotation_matrix[2], translation[0],
            rotation_matrix[3], rotation_matrix[4], rotation_matrix[5], translation[1],
            rotation_matrix[6], rotation_matrix[7], rotation_matrix[8], translation[2],
        ]
    }
}

impl Display for Affine3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "Affine3\n[\n\tMatrix4x4[\n\t\t{}\n\t\t{}\n\t\t{}\n\t\t]\n\tTranslation[\n\t\t{}\n\t]\n]",
               self.matrix3.column_x, self.matrix3.column_y, self.matrix3.column_z, self.translation
        )
    }
}

impl Mul for Affine3 {
    type Output = Affine3;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let matrix3: Matrix3x3 = self.matrix3 * rhs.matrix3;
        let translation: Vector3 = self.matrix3 * rhs.translation + self.translation;

        Affine3 {
            translation,
            matrix3,
        }
    }
}

impl MulAssign for Affine3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.translation = self.matrix3 * rhs.translation + self.translation;
        self.matrix3 = self.matrix3 * rhs.matrix3;
    }
}

pub mod affine3_math {

    use crate::{Affine3, Matrix3x3, Quaternion, Vector3};

    use crate::matrix3x3_math::{
        determinant_mat3, from_quat_mat3, inverse_mat3, rotate_x_mat3, rotate_y_mat3, rotate_z_mat3,
    };
    use crate::quaternion_math::{
        from_angle_axis_quat, from_euler_quat, from_rotation_matrix_quat,
    };
    use crate::vector_math::{length, rcp};

    use crate::EulerOrder;

    #[inline]
    pub fn from_quaternion_affine3(quaternion: Quaternion) -> Affine3 {
        let rotation_matrix: Matrix3x3 = from_quat_mat3(quaternion);

        Affine3 {
            translation: Vector3::ZERO,
            matrix3: rotation_matrix,
        }
    }

    #[inline]
    pub fn from_angle_axis_affine3(normalized_axis: Vector3, angle_radian: f32) -> Affine3 {
        let quaternion: Quaternion = from_angle_axis_quat(normalized_axis, angle_radian);

        let rotation_matrix: Matrix3x3 = from_quat_mat3(quaternion);

        Affine3 {
            translation: Vector3::ZERO,
            matrix3: rotation_matrix,
        }
    }

    #[inline]
    pub fn rotate_x_affine3(angle_radian: f32) -> Affine3 {
        let rotation_matrix: Matrix3x3 = rotate_x_mat3(angle_radian);

        Affine3 {
            translation: Vector3::ZERO,
            matrix3: rotation_matrix,
        }
    }

    #[inline]
    pub fn rotate_y_affine3(angle_radian: f32) -> Affine3 {
        let rotation_matrix: Matrix3x3 = rotate_y_mat3(angle_radian);

        Affine3 {
            translation: Vector3::ZERO,
            matrix3: rotation_matrix,
        }
    }

    #[inline]
    pub fn rotate_z_affine3(angle_radian: f32) -> Affine3 {
        let rotation_matrix: Matrix3x3 = rotate_z_mat3(angle_radian);

        Affine3 {
            translation: Vector3::ZERO,
            matrix3: rotation_matrix,
        }
    }

    #[inline]
    pub fn from_euler_affine3(euler_radians: Vector3, euler_order: EulerOrder) -> Affine3 {
        let quaternion: Quaternion = from_euler_quat(euler_radians, euler_order);

        let rotation_matrix: Matrix3x3 = from_quat_mat3(quaternion);

        Affine3 {
            translation: Vector3::ZERO,
            matrix3: rotation_matrix,
        }
    }

    #[inline]
    pub fn from_translate_euler_affine3(
        translation: Vector3,
        euler_radians: Vector3,
        euler_order: EulerOrder,
    ) -> Affine3 {
        let quaternion: Quaternion = from_euler_quat(euler_radians, euler_order);

        let rotation_matrix: Matrix3x3 = from_quat_mat3(quaternion);

        Affine3 {
            translation,
            matrix3: rotation_matrix,
        }
    }

    #[inline]
    pub fn from_translate_quat_affine3(translation: Vector3, quaternion: Quaternion) -> Affine3 {
        let rotation_matrix: Matrix3x3 = from_quat_mat3(quaternion);

        Affine3 {
            translation,
            matrix3: rotation_matrix,
        }
    }

    #[inline]
    pub fn from_translation_affine3(translation: Vector3) -> Affine3 {
        Affine3 {
            translation,
            matrix3: Matrix3x3::IDENTITY,
        }
    }

    #[inline]
    pub fn from_scale_affine3(scale: Vector3) -> Affine3 {
        let x_scaled_column: Vector3 = Vector3::set(scale.x(), 0.0, 0.0);
        let y_scaled_column: Vector3 = Vector3::set(0.0, scale.y(), 0.0);
        let z_scaled_column: Vector3 = Vector3::set(0.0, 0.0, scale.z());

        let diagonal_matrix: Matrix3x3 =
            Matrix3x3::set(x_scaled_column, y_scaled_column, z_scaled_column);

        Affine3 {
            translation: Vector3::ZERO,
            matrix3: diagonal_matrix,
        }
    }

    #[inline]
    pub fn inverse_affine3(afffine3: Affine3) -> Affine3 {
        let inverse_matrix3x3: Matrix3x3 = inverse_mat3(afffine3.matrix3);

        let inverse_translation: Vector3 = -(inverse_matrix3x3 * afffine3.translation);

        Affine3 {
            translation: inverse_translation,
            matrix3: inverse_matrix3x3,
        }
    }

    #[inline]
    pub fn decompose_translation_scale_affine3(afffine3: Affine3) -> (Vector3, Vector3) {
        let rotation_matrix: Matrix3x3 = afffine3.matrix3;

        let determinant: f32 = determinant_mat3(afffine3.matrix3);

        let scale: Vector3 = Vector3::set(
            length(rotation_matrix.column_x.value) * determinant.signum(),
            length(rotation_matrix.column_y.value),
            length(rotation_matrix.column_z.value),
        );

        let translation: Vector3 = afffine3.translation;

        (translation, scale)
    }

    #[inline]
    pub fn decompose_translation_rotation_affine3(afffine3: Affine3) -> (Vector3, Quaternion) {
        let (translation, scale) = decompose_translation_scale_affine3(afffine3);

        let inverse_scale: Vector3 = Vector3 {
            value: rcp(scale.value),
        };

        let column_x: Vector3 = afffine3.matrix3.column_x * inverse_scale.x();
        let column_y: Vector3 = afffine3.matrix3.column_y * inverse_scale.y();
        let column_z: Vector3 = afffine3.matrix3.column_z * inverse_scale.z();

        let quaternion: Quaternion =
            from_rotation_matrix_quat(Matrix3x3::set(column_x, column_y, column_z));

        (translation, quaternion)
    }

    #[inline]
    pub fn decompose_rotation_scale_affine3(afffine3: Affine3) -> (Quaternion, Vector3) {
        let determinant: f32 = determinant_mat3(afffine3.matrix3);

        let scale: Vector3 = Vector3::set(
            length(afffine3.matrix3.column_x.value) * determinant.signum(),
            length(afffine3.matrix3.column_y.value),
            length(afffine3.matrix3.column_z.value),
        );

        let inverse_scale: Vector3 = Vector3 {
            value: rcp(scale.value),
        };

        let column_x: Vector3 = afffine3.matrix3.column_x * inverse_scale.x();
        let column_y: Vector3 = afffine3.matrix3.column_y * inverse_scale.y();
        let column_z: Vector3 = afffine3.matrix3.column_z * inverse_scale.z();

        let quaternion: Quaternion =
            from_rotation_matrix_quat(Matrix3x3::set(column_x, column_y, column_z));


        (quaternion, scale)
    }

    #[inline]
    pub fn decompose_trs_affine3(afffine3: Affine3) -> (Vector3, Quaternion, Vector3) {
        let (rotation, scale) = decompose_rotation_scale_affine3(afffine3);
        let translation: Vector3 = afffine3.translation;

        (translation, rotation, scale)
    }
}