use crate::util::acos;
use crate::{Rotation, Scale, Translation};

pub fn forward(rotation: Rotation) -> [f32; 3] {
    let forward_translation = Translation {
        value: [0.0, 0.0, 1.0, 1.0],
    };

    vec_mul_qut(rotation, forward_translation)
}

pub fn right(rotation: Rotation) -> [f32; 3] {
    let right_translation = Translation {
        value: [1.0, 0.0, 0.0, 1.0],
    };

    vec_mul_qut(rotation, right_translation)
}

pub fn up(rotation: Rotation) -> [f32; 3] {
    let up_translation = Translation {
        value: [0.0, 1.0, 0.0, 1.0],
    };

    vec_mul_qut(rotation, up_translation)
}


pub fn vec_mul_qut(rotation: Rotation, translation: Translation) -> [f32; 3] {
    let rotation_matrix = get_rotation_matrix(rotation);

    let direction_vector = translation.value;

    let x = direction_vector[0];
    let y = direction_vector[1];
    let z = direction_vector[2];

    [
        rotation_matrix[0] * x + rotation_matrix[3] * y + rotation_matrix[6] * z,
        rotation_matrix[1] * x + rotation_matrix[4] * y + rotation_matrix[7] * z,
        rotation_matrix[2] * x + rotation_matrix[6] * y + rotation_matrix[8] * z,
    ]
}


#[rustfmt::skip]
pub fn get_rotation_matrix(rotation : Rotation) -> [f32; 9] {
    let (qx, qy, qz, qw) = (
        rotation.value[0],
        rotation.value[1],
        rotation.value[2],
        rotation.value[3],
    );

    let xx = qx * qx;
    let yy = qy * qy;
    let zz = qz * qz;
    let xy = qx * qy;
    let xz = qx * qz;
    let yz = qy * qz;
    let wx = qw * qx;
    let wy = qw * qy;
    let wz = qw * qz;

    [
        1.0 - 2.0 * (yy + zz), 2.0 * (xy + wz), 2.0 * (xz - wy),//col 0
        2.0 * (xy - wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz + wx),//col 1
        2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (xx + yy) //col 2
    ]
}

// todo scale is not implemented yet in the equation.
#[rustfmt::skip]
pub fn get_transformation_matrix(position : Translation, rotation : Rotation, scale : Scale) -> [f32; 16] {
    let rotation_matrix = get_rotation_matrix(rotation);
    [
        rotation_matrix[0], rotation_matrix[1], rotation_matrix[2], 0.0, // col 0
        rotation_matrix[3], rotation_matrix[4], rotation_matrix[5], 0.0, // col 1
        rotation_matrix[6], rotation_matrix[7], rotation_matrix[8], 0.0, // col 2
        position.value[0], position.value[1],  position.value[2], 1.0 // col 3
    ]

}

pub fn get_axis_angle(rotation: Rotation) -> ([f32; 3], f32) {
    const SQR_EPSILON: f32 = f32::EPSILON * f32::EPSILON;

    let (i, j, k, w) = (
        rotation.value[0],
        rotation.value[1],
        rotation.value[2],
        rotation.value[3],
    );

    let scale_sq = (1.0 - w * w).max(0.0);

    let angle = 2.0 * acos(w);

    if scale_sq < SQR_EPSILON {
        ([1.0, 0.0, 0.0], angle)
    } else {
        let inv_sqrt_scale = scale_sq.sqrt().recip();
        (
            [i * inv_sqrt_scale, j * inv_sqrt_scale, k * inv_sqrt_scale],
            angle,
        )
    }
}

pub fn get_angle_axis_magnitude(rotation: Rotation) -> [f32; 3] {
    let (axis, angle) = get_axis_angle(rotation);

    [axis[0] * angle, axis[1] * angle, axis[2] * angle]
}

pub fn get_euler_angle(rotation: Rotation) -> [f32; 3] {
    let (i, j, k, w) = (
        rotation.value[0],
        rotation.value[1],
        rotation.value[2],
        rotation.value[3],
    );

    let xx = i * i;
    let xy = i * j;
    let xz = i * k;
    let xw = i * w;

    let yy = j * j;
    let yz = j * k;
    let yw = j * w;

    let zz = k * k;
    let zw = k * w;

    let ww = w * w;

    let x = (-2.0 * (yz - xw)).atan2(ww - xx - yy + zz);

    let unsafe_y = 2.0 * (xz + yw);

    let y = unsafe_y.clamp(-1.0, 1.0).asin();

    let z = (-2.0 * (xy - zw)).atan2(ww + xx - yy - zz);

    [x, y, z]
}

#[cfg(test)]
mod transform_test {
    use crate::{
        forward, get_angle_axis_magnitude, get_axis_angle, get_euler_angle, get_rotation_matrix,
        get_transformation_matrix, right, up, Rotation, Translation,
    };

    #[test]
    fn transformation_matrix() {
        let threshold = 0.0000001;

        let quaternion = glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            128.0f32.to_radians(),
            18.5f32.to_radians(),
            90.4f32.to_radians(),
        );


        let position = Translation {
            value: [3.153, 100.1, 1.5, 1.0],
        };

        let rotation = Rotation {
            value: [quaternion.x, quaternion.y, quaternion.z, quaternion.w],
        };

        let transformation_matrix =
            get_transformation_matrix(position, rotation, Default::default());

        let m4_transformation_representation = glam::Mat4::from_cols_array(&transformation_matrix);

        let decomposed_translation = m4_transformation_representation.w_axis.truncate();

        println!("{:?}", position);
        println!("{:?}", decomposed_translation);

        assert!(position
            .value
            .eq(&decomposed_translation.extend(1.0).to_array()));

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

        let position = Translation {
            value: [3.0, 2.0, 1.5, 1.0],
        };

        let rotation = Rotation {
            value: [quaternion.x, quaternion.y, quaternion.z, quaternion.w],
        };


        let rotation_matrix = get_rotation_matrix(rotation);

        let transformation_matrix =
            get_transformation_matrix(position, rotation, Default::default());

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

        assert!((quaternion.x - rotation.value[0]).abs() < threshold);
        assert!((quaternion.y - rotation.value[1]).abs() < threshold);
        assert!((quaternion.z - rotation.value[2]).abs() < threshold);
        assert!((quaternion.w - rotation.value[3]).abs() < threshold);
    }

    #[test]
    fn angle_axis() {
        const THRESHOLD: f32 = 0.0001;

        let quaternion: [f32; 4] = [0.3441577, 0.9188383, 0.1917763, 0.0226621];

        let rotation = Rotation { value: quaternion };
        let (axis, angle) = get_axis_angle(rotation);

        assert!(axis[0] < axis[1]);
        assert!(axis[2] < axis[1]);

        let glm_quaternion = glam::quat(0.3441577, 0.9188383, 0.1917763, 0.0226621);

        let (glm_axis, glm_angle) = glm_quaternion.to_axis_angle();

        let (proven_axis, proven_angle) =
            ([0.3442461_f32, 0.9190743_f32, 0.1918255_f32], 3.0962646);


        let difference = [
            (axis[0] - glm_axis.x).abs(),
            (axis[1] - glm_axis.y).abs(),
            (axis[2] - glm_axis.z).abs(),
            (angle - glm_angle).abs(),
        ];

        assert!(difference[0] <= THRESHOLD);
        assert!(difference[1] <= THRESHOLD);
        assert!(difference[2] <= THRESHOLD);
        assert!(difference[3] <= THRESHOLD);

        let difference = [
            (axis[0] - proven_axis[0]).abs(),
            (axis[1] - proven_axis[1]).abs(),
            (axis[2] - proven_axis[2]).abs(),
            (angle - proven_angle).abs(),
        ];

        assert!(difference[0] <= THRESHOLD);
        assert!(difference[1] <= THRESHOLD);
        assert!(difference[2] <= THRESHOLD);
        assert!(difference[3] <= THRESHOLD);
    }

    #[test]
    fn angle_axis_magnitude() {
        const THRESHOLD: f32 = 0.0001;

        // XYZ Euler angle (degree) of 20,170,39.3;
        let quaternion = [0.3441577_f32, 0.9188383, 0.1917763, 0.0226621];
        let euler_quaternion = glam::Quat::from_euler(
            glam::EulerRot::XYZ,
            20_f32.to_radians(),
            170_f32.to_radians(),
            39.3_f32.to_radians(),
        );

        let difference = [
            (quaternion[0] - euler_quaternion.x).abs(),
            (quaternion[1] - euler_quaternion.y).abs(),
            (quaternion[2] - euler_quaternion.z).abs(),
            (quaternion[3] - euler_quaternion.w).abs(),
        ];

        assert!(difference[0] <= THRESHOLD);
        assert!(difference[1] <= THRESHOLD);
        assert!(difference[2] <= THRESHOLD);

        let rotation = Rotation { value: quaternion };

        // return in radians;
        let angle_axis_mag = get_angle_axis_magnitude(rotation);

        // https://www.andre-gaschler.com/rotationconverter/
        // proven quaternion rotation calculator.
        let result_angle_axis_mag = [1.065877, 2.8456972, 0.5939426];

        let difference = [
            (angle_axis_mag[0] - result_angle_axis_mag[0]).abs(),
            (angle_axis_mag[1] - result_angle_axis_mag[1]).abs(),
            (angle_axis_mag[2] - result_angle_axis_mag[2]).abs(),
        ];

        assert!(difference[0] <= THRESHOLD);
        assert!(difference[1] <= THRESHOLD);
        assert!(difference[2] <= THRESHOLD);
    }

    #[test]
    fn euler_angle() {
        const THRESHOLD: f32 = 0.0000001;

        let quaternion: [f32; 4] = [0.2284545, 0.255438, 0.1609776, 0.9255518];

        let rotation = Rotation { value: quaternion };

        let euler_0 = get_euler_angle(rotation);

        let glm_quaternion = glam::Quat::from_array(quaternion);

        let euler_1 = glm_quaternion.to_euler(glam::EulerRot::XYZ);

        assert!(euler_0[0].eq(&euler_1.0));
        assert!(euler_0[1].eq(&euler_1.1));
        assert!(euler_0[2].eq(&euler_1.2));


        let proven_euler_radian = [0.418879, 0.578053, 0.2181662];


        let difference = [
            (euler_0[0] - proven_euler_radian[0]).abs(),
            (euler_0[1] - proven_euler_radian[1]).abs(),
            (euler_0[2] - proven_euler_radian[2]).abs(),
        ];

        assert!(difference[0] <= THRESHOLD);
        assert!(difference[1] <= THRESHOLD);
        assert!(difference[2] <= THRESHOLD);
    }

    #[test]
    fn update_vector() {
        const THRESHOLD: f32 = 0.1;

        let rotation = Rotation {
            value: [0.2284545, 0.255438, 0.1609776, 0.9255518],
        };

        let forward_direction = forward(rotation);
        let right_direction = right(rotation);
        let up_direction = up(rotation);

        // retrieved from a proven game engine. Editor rounds to integer, so there will
        // be an error_threshold, sine we don't round our calculation to integer
        let proven_forward_direction = [0.5, -0.3, 0.8];
        let proven_right_direction = [0.8, 0.4, -0.4];
        let proven_up_direction = [-0.2, 0.8, 0.5];

        let forward_difference = [
            (forward_direction[0] - proven_forward_direction[0]).abs(),
            (forward_direction[1] - proven_forward_direction[1]).abs(),
            (forward_direction[2] - proven_forward_direction[2]).abs(),
        ];

        let right_difference = [
            (right_direction[0] - proven_right_direction[0]).abs(),
            (right_direction[1] - proven_right_direction[1]).abs(),
            (right_direction[2] - proven_right_direction[2]).abs(),
        ];

        let up_difference = [
            (up_direction[0] - proven_up_direction[0]).abs(),
            (up_direction[1] - proven_up_direction[1]).abs(),
            (up_direction[2] - proven_up_direction[2]).abs(),
        ];

        assert!(forward_difference[0] <= THRESHOLD);
        assert!(forward_difference[1] <= THRESHOLD);
        assert!(forward_difference[2] <= THRESHOLD);

        assert!(right_difference[0] <= THRESHOLD);
        assert!(right_difference[1] <= THRESHOLD);
        assert!(right_difference[2] <= THRESHOLD);

        assert!(up_difference[0] <= THRESHOLD);
        assert!(up_difference[1] <= THRESHOLD);
        assert!(up_difference[2] <= THRESHOLD);
    }
}
