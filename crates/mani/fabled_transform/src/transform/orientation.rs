use crate::transform::transform::Transform;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Orientation {
    pub transform: Transform,
    pub forward: [f32; 3],
    pub right: [f32; 3],
}

impl Default for Orientation {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            forward: [0.0, 0.0, 1.0],
            right: [1.0, 0.0, 0.0],
        }
    }
}

impl Orientation {
    /// rotation is representing angle pitch, yaw, roll in radians
    pub fn update_rotation(&mut self, rotation: [f32; 3]) {
        let current_rotation = glam::Quat::from_array(self.transform.rotation);

        let x_rotation = glam::Quat::from_rotation_x(rotation[0]);
        let y_rotation = glam::Quat::from_rotation_y(rotation[1]);
        let z_rotation = glam::Quat::from_rotation_z(rotation[2]);

        let target_rotation = x_rotation * y_rotation * z_rotation;

        let mut desired_rotation = current_rotation * target_rotation;

        desired_rotation = desired_rotation.normalize();

        self.transform.rotation = [
            desired_rotation.x,
            desired_rotation.y,
            desired_rotation.z,
            desired_rotation.w,
        ];

        // Recalculate forward and right direction.
        self.forward = (desired_rotation * glam::Vec3::Z).to_array();
        self.right = (desired_rotation * glam::Vec3::X).to_array();
    }

    pub fn update_translation(&mut self, translation: [f32; 3]) {
        let mut transformation_matrix = self.transform.get_transformation_matrix();

        let (_rotation_matrix, target_translation) = transformation_matrix.split_at_mut(12);

        assert!(target_translation.len().eq(&4));

        target_translation[0] += self.forward[0] * translation[2];
        target_translation[1] += self.forward[1] * translation[2];
        target_translation[2] += self.forward[2] * translation[2];

        target_translation[0] += self.right[0] * translation[0];
        target_translation[1] += self.right[1] * translation[0];
        target_translation[2] += self.right[2] * translation[0];

        let z = self.forward[0] * self.right[1] - self.forward[1] * self.right[0];
        let x = self.forward[1] * self.right[2] - self.forward[2] * self.right[1];
        let y = self.forward[2] * self.right[0] - self.forward[0] * self.right[2];

        let up = [x, y, z];

        target_translation[0] += up[0] * translation[1];
        target_translation[1] += up[1] * translation[1];
        target_translation[2] += up[2] * translation[1];

        self.transform.position = [
            target_translation[0],
            target_translation[1],
            target_translation[2],
        ];
    }
}

#[cfg(test)]
mod orientation_test {
    use crate::Orientation;

    #[test]
    fn update_translation() {
        let mut orientation = Orientation::default();

        orientation.update_translation([1.0, 5.0, 2.0]);

        let position = orientation.transform.position;

        assert!(position[0].eq(&1.0));
        assert!(position[1].eq(&5.0));
        assert!(position[2].eq(&2.0));
    }

    #[test]
    fn update_rotation() {
        // Angle can't be greater than + 180 for quaternion if so it will map to the
        // negative equivalent making the test fail.
        let rotation_target = [
            15.03f32.to_radians(),
            60.123f32.to_radians(),
            179f32.to_radians(),
        ];

        let error_threshold = 0.0001;
        let mut orientation = Orientation::default();

        orientation.update_rotation(rotation_target);

        let rotation = orientation.transform.rotation;

        // we will convert it to euler rotation for test.
        let (x_rad, y_rad, z_rad) = glam::Quat::from_array(rotation).to_euler(glam::EulerRot::XYZ);

        assert!((x_rad - rotation_target[0]).abs() < error_threshold);
        assert!((y_rad - rotation_target[1]).abs() < error_threshold);
        assert!((z_rad - rotation_target[2]).abs() < error_threshold);
    }


    #[test]
    pub fn recalculate_orientation() {
        // Angle can't be greater than + 180 for quaternion if so it will map to the
        // negative equivalent making the test fail.
        let rotation_target = [
            15.03f32.to_radians(),
            60.123f32.to_radians(),
            179f32.to_radians(),
        ];

        let mut orientation = Orientation::default();

        orientation.update_rotation(rotation_target);


        // Value have been extracted from popular developed game engines
        // Forward (0.9, -0.1, 0.5)
        // Right (-0.5, -0.2, 0.8)
        let proven_forward = [0.9, -0.1, 0.5];
        let proven_right = [-0.5, -0.2, 0.8];

        assert!(((orientation.forward[0] * 10.0).round() / 10.0).eq(&proven_forward[0]));
        assert!(((orientation.forward[1] * 10.0).round() / 10.0).eq(&proven_forward[1]));
        assert!(((orientation.forward[2] * 10.0).round() / 10.0).eq(&proven_forward[2]));


        assert!(((orientation.right[0] * 10.0).round() / 10.0).eq(&proven_right[0]));
        assert!(((orientation.right[1] * 10.0).round() / 10.0).eq(&proven_right[1]));
        assert!(((orientation.right[2] * 10.0).round() / 10.0).eq(&proven_right[2]));
    }
}
