use fabled_math::Transform;
use glam::Vec4Swizzles;

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

        let target_rotation = x_rotation * y_rotation;

        let mut desired_rotation = current_rotation * target_rotation;

        desired_rotation = desired_rotation.normalize();

        self.transform.rotation = [
            desired_rotation.x,
            desired_rotation.y,
            desired_rotation.z,
            desired_rotation.w,
        ];
    }

    // local coordinates (model's forward and not the world forward).
    pub fn update_translation(&mut self, translation: [f32; 3]) {
        let mut transformation_matrix = self.transform.get_transformation_matrix();
        let m4_transformation_representation = glam::Mat4::from_cols_array(&transformation_matrix);

        let forward = m4_transformation_representation.z_axis.normalize().xyz();
        let right = m4_transformation_representation.x_axis.normalize().xyz();

        self.forward = forward.to_array();
        self.right = right.to_array();

        let (_rotation_matrix, target_translation) = transformation_matrix.split_at_mut(12);

        assert!(target_translation.len().eq(&4));

        target_translation[0] += self.forward[0] * translation[2];
        target_translation[1] += self.forward[1] * translation[2];
        target_translation[2] += self.forward[2] * translation[2];

        target_translation[0] += self.right[0] * translation[0];
        target_translation[1] += self.right[1] * translation[0];
        target_translation[2] += self.right[2] * translation[0];

        let up = forward.cross(right).to_array();

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
    use crate::camera::Orientation;

    #[test]
    fn update_translation() {
        let mut orientation = Orientation::default();

        orientation.update_translation([1.0, 5.0, 2.0]);
        println!("{:?}", orientation.transform.position);
    }
}
