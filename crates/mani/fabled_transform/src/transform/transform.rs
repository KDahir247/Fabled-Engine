#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: [0.0; 3], // Origin
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0; 3],
        }
    }
}

impl Transform {
    pub fn new(position: [f32; 3], rotation: [f32; 4], scale: [f32; 3]) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    #[rustfmt::skip]
    pub fn get_rotation_matrix(&self) -> [f32; 9] {

        let x = self.rotation[0];
        let y = self.rotation[1];
        let z = self.rotation[2];
        let w = self.rotation[3];

        let x2 = x * x;
        let y2 = y * y;
        let z2 = z * z;
        let xy = x * y;
        let xz = x * z;
        let yz = y * z;
        let wx = w * x;
        let wy = w * y;
        let wz = w * z;

        [
            1.0 - 2.0 * (y2 + z2), 2.0 * (xy + wz), 2.0 * (xz - wy),//col 0
            2.0 * (xy - wz), 1.0 - 2.0 * (x2 + z2), 2.0 * (yz + wx),//col 1
            2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (x2 + y2) //col 2
        ]
    }

    pub fn get_transformation_matrix(&self) -> [f32; 16] {
        let rotation = glam::Quat::from_array(self.rotation).normalize();
        let scale = glam::Vec3::from(self.scale);
        let translation = glam::Vec3::from(self.position);

        let transformation_matrix =
            glam::Mat4::from_scale_rotation_translation(scale, rotation, translation);

        transformation_matrix.to_cols_array()
    }
}

#[cfg(test)]
mod transform_test {
    use crate::transform::transform::Transform;

    #[test]
    fn transformation_matrix() {
        let threshold = 0.0000001;

        let quaternion = glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            128.0f32.to_radians(),
            18.5f32.to_radians(),
            90.4f32.to_radians(),
        );

        let position = [3.153, 100.1, 1.5];

        let transform = Transform::new(
            position,
            [quaternion.x, quaternion.y, quaternion.z, quaternion.w],
            [1.0, 1.0, 1.0],
        );

        let transformation_matrix = transform.get_transformation_matrix();

        let m4_transformation_representation = glam::Mat4::from_cols_array(&transformation_matrix);

        let decomposed_translation = m4_transformation_representation.w_axis.truncate();

        assert!(position.eq(&decomposed_translation.to_array()));

        // let c1 = m4_transformation_representation.x_axis.truncate();
        // let c2 = m4_transformation_representation.y_axis.truncate();
        // let c3 = m4_transformation_representation.z_axis.truncate();
        //
        // let angle_x = c3.y.atan2(c3.z).to_degrees();
        // let angle_y = c3.x.atan2((c3.y * c3.y + c3.z * c3.z).sqrt()).to_degrees();
        // let angle_z = c1.y.atan2(c1.x).to_degrees();

        let (_scale, rotation, _position) =
            m4_transformation_representation.to_scale_rotation_translation();

        assert!((rotation.x - quaternion.x).abs() < threshold);
        assert!((rotation.y - quaternion.y).abs() < threshold);
        assert!((rotation.z - quaternion.z).abs() < threshold);
        assert!((rotation.w - quaternion.w).abs() < threshold);
    }

    #[test]
    fn rotation_matrix() {
        let threshold = 0.0000001;

        let quaternion = glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            45.0f32.to_radians(),
            15.5f32.to_radians(),
            190.4f32.to_radians(),
        );

        let transform = Transform::new(
            [3.0, 2.0, 1.5],
            [quaternion.x, quaternion.y, quaternion.z, quaternion.w],
            [1.0, 1.0, 1.0],
        );

        let rotation_matrix = transform.get_rotation_matrix();
        let transformation_matrix = transform.get_transformation_matrix();

        // transformation matrix on the 3x3 on the top left should be the rotation
        // matrix if there is no scaling.
        assert!(rotation_matrix[0].eq(&transformation_matrix[0]));
        assert!(rotation_matrix[1].eq(&transformation_matrix[1]));
        assert!(rotation_matrix[2].eq(&transformation_matrix[2]));
        // last row in the transformation matrix is zero.
        assert!(rotation_matrix[4].eq(&transformation_matrix[5]));
        assert!(rotation_matrix[5].eq(&transformation_matrix[6]));
        // last row in the transformation matrix is zero.
        assert!(rotation_matrix[6].eq(&transformation_matrix[8]));
        assert!(rotation_matrix[7].eq(&transformation_matrix[9]));
        assert!(rotation_matrix[8].eq(&transformation_matrix[10]));

        let rotation_matrix = glam::Mat3::from_cols_array(&rotation_matrix);
        let quaternion = glam::Quat::from_mat3(&rotation_matrix);

        assert!((quaternion.x - transform.rotation[0]).abs() < threshold);
        assert!((quaternion.y - transform.rotation[1]).abs() < threshold);
        assert!((quaternion.z - transform.rotation[2]).abs() < threshold);
        assert!((quaternion.w - transform.rotation[3]).abs() < threshold);
    }
}
