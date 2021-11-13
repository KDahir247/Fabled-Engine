use crate::container::AxisDirection;
use crate::{Orientation, Transform};

// NOT tested yet.
pub fn reflect_orientation(orientation: Orientation, direction: AxisDirection) -> Orientation {
    let direction_i32 = direction as i32;

    let mut c0_r0 = -(direction_i32 & 0b00000001).signum() as f32;
    let mut c1_r1 = -(direction_i32 & 0b00000010).signum() as f32;

    // todo temporary solution this will change eventually to not use if statement.
    if c0_r0.eq(&0.0) {
        c0_r0 = 1.0;
    }

    if c1_r1.eq(&0.0) {
        c1_r1 = 1.0;
    }

    let mut transformation_matrix = orientation.transform.get_transformation_matrix();

    let transformation_c0_r0 = transformation_matrix[0] * c0_r0;

    let transformation_c1_r1 = transformation_matrix[5] * c1_r1;

    transformation_matrix[0] = transformation_c0_r0;
    transformation_matrix[5] = transformation_c1_r1;

    let reflected_right = [
        orientation.right[0] * c0_r0,
        orientation.right[1],
        orientation.right[2],
    ];

    let glm_transformation_matrix = glam::Mat4::from_cols_array(&transformation_matrix);
    let (scale, rotation, translation) = glm_transformation_matrix.to_scale_rotation_translation();


    let reflected_transform = Transform {
        position: translation.to_array(),
        rotation: [rotation.x, rotation.y, rotation.z, rotation.w],
        scale: scale.to_array(),
    };

    Orientation {
        transform: reflected_transform,
        forward: orientation.forward,
        right: reflected_right,
    }
}
