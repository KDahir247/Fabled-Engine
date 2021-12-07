use crate::util::acos;
use crate::{Rotation, Scale, ScaleType, Translation};

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

    let dir_x = direction_vector[0];
    let dir_y = direction_vector[1];
    let dir_z = direction_vector[2];

    [
        rotation_matrix[0] * dir_x + rotation_matrix[3] * dir_y + rotation_matrix[6] * dir_z,
        rotation_matrix[1] * dir_x + rotation_matrix[4] * dir_y + rotation_matrix[7] * dir_z,
        rotation_matrix[2] * dir_x + rotation_matrix[6] * dir_y + rotation_matrix[8] * dir_z,
    ]
}


#[rustfmt::skip]
pub fn get_rotation_matrix(rotation : Rotation) -> [f32; 9] {
    let (quat_i, quat_j, quat_k, quat_w) = (
        rotation.value[0],
        rotation.value[1],
        rotation.value[2],
        rotation.value[3],
    );

    let quat_ii = quat_i * quat_i;
    let quat_jj = quat_j * quat_j;
    let quat_kk = quat_k * quat_k;
    let quat_ij = quat_i * quat_j;
    let quat_ik = quat_i * quat_k;
    let quat_jk = quat_j * quat_k;
    let quat_wi = quat_w * quat_i;
    let quat_wj = quat_w * quat_j;
    let quat_wk = quat_w * quat_k;

    [
        1.0 - 2.0 * (quat_jj + quat_kk), 2.0 * (quat_ij + quat_wk), 2.0 * (quat_ik - quat_wj),//col 0
        2.0 * (quat_ij - quat_wk), 1.0 - 2.0 * (quat_ii + quat_kk), 2.0 * (quat_jk + quat_wi),//col 1
        2.0 * (quat_ik + quat_wj), 2.0 * (quat_jk - quat_wi), 1.0 - 2.0 * (quat_ii + quat_jj) //col 2
    ]
}

// world
#[rustfmt::skip]
pub fn get_transformation_matrix(position : Translation, rotation : Rotation, scale : Scale) -> [f32; 16] {
    let rotation_matrix = get_rotation_matrix(rotation);
    
    let scalar = match scale.value{
        ScaleType::Uniform(uniform) => [uniform; 3],
        ScaleType::NonUniform(non_uniform) => non_uniform
    };

    let inner_position = position.value;
    
    let rcp_position_scalar = 1.0 / inner_position[3];
    
    let norm_position = [inner_position[0] * rcp_position_scalar, inner_position[1] * rcp_position_scalar, inner_position[2] * rcp_position_scalar];
    
    [
        rotation_matrix[0] * scalar[0], rotation_matrix[1] * scalar[0], rotation_matrix[2] * scalar[0], 0.0, // col 0
        rotation_matrix[3] * scalar[1], rotation_matrix[4] * scalar[1], rotation_matrix[5] * scalar[1], 0.0, // col 1
        rotation_matrix[6] * scalar[2], rotation_matrix[7] * scalar[2], rotation_matrix[8] * scalar[2], 0.0, // col 2
        norm_position[0], norm_position[1],  norm_position[2], 1.0 // col 3
    ]

}

pub fn get_axis_angle(rotation: Rotation) -> ([f32; 3], f32) {
    const SQR_EPSILON: f32 = f32::EPSILON * f32::EPSILON;

    let (quat_i, quat_j, quat_k, quat_w) = (
        rotation.value[0],
        rotation.value[1],
        rotation.value[2],
        rotation.value[3],
    );

    let scale_sq = (1.0 - quat_w * quat_w).max(0.0);

    let angle = 2.0 * acos(quat_w);

    if scale_sq < SQR_EPSILON {
        ([1.0, 0.0, 0.0], angle)
    } else {
        let inv_sqrt_scale = scale_sq.sqrt().recip();
        (
            [
                quat_i * inv_sqrt_scale,
                quat_j * inv_sqrt_scale,
                quat_k * inv_sqrt_scale,
            ],
            angle,
        )
    }
}

pub fn get_angle_axis_magnitude(rotation: Rotation) -> [f32; 3] {
    let (axis, angle) = get_axis_angle(rotation);

    [axis[0] * angle, axis[1] * angle, axis[2] * angle]
}

pub fn get_euler_angle(rotation: Rotation) -> [f32; 3] {
    let (quat_i, quat_j, quat_k, quat_w) = (
        rotation.value[0],
        rotation.value[1],
        rotation.value[2],
        rotation.value[3],
    );

    let quat_ii = quat_i * quat_i;
    let quat_ij = quat_i * quat_j;
    let quat_ik = quat_i * quat_k;
    let quat_iw = quat_i * quat_w;

    let quat_jj = quat_j * quat_j;
    let quat_jk = quat_j * quat_k;
    let quat_jw = quat_j * quat_w;

    let quat_kk = quat_k * quat_k;
    let quat_kw = quat_k * quat_w;

    let quat_ww = quat_w * quat_w;

    let x = (-2.0 * (quat_jk - quat_iw)).atan2(quat_ww - quat_ii - quat_jj + quat_kk);

    let unsafe_y = 2.0 * (quat_ik + quat_jw);

    let y = unsafe_y.clamp(-1.0, 1.0).asin();

    let z = (-2.0 * (quat_ij - quat_kw)).atan2(quat_ww + quat_ii - quat_jj - quat_kk);

    [x, y, z]
}

pub fn decompose_transformation_matrix(
    transformation_matrix: [f32; 16],
) -> (Translation, Rotation, Scale) {
    let rcp_translation_scalar = 1.0 / transformation_matrix[15];

    let translation_inner = [
        transformation_matrix[12] * rcp_translation_scalar,
        transformation_matrix[13] * rcp_translation_scalar,
        transformation_matrix[14] * rcp_translation_scalar,
        1.0,
    ];

    let sqr_scale_x = transformation_matrix[0] * transformation_matrix[0]
        + transformation_matrix[1] * transformation_matrix[1]
        + transformation_matrix[2] * transformation_matrix[2];
    let sqr_scale_y = transformation_matrix[4] * transformation_matrix[4]
        + transformation_matrix[5] * transformation_matrix[5]
        + transformation_matrix[6] * transformation_matrix[6];
    let sqr_scale_z = transformation_matrix[8] * transformation_matrix[8]
        + transformation_matrix[9] * transformation_matrix[9]
        + transformation_matrix[10] * transformation_matrix[10];

    let scale_inner = [sqr_scale_x.sqrt(), sqr_scale_y.sqrt(), sqr_scale_z.sqrt()];

    let rcp_scale_x = scale_inner[0].recip();
    let rcp_scale_y = scale_inner[1].recip();
    let rcp_scale_z = scale_inner[2].recip();

    let rotation_matrix = [
        transformation_matrix[0] * rcp_scale_x,
        transformation_matrix[1] * rcp_scale_x,
        transformation_matrix[2] * rcp_scale_x, // col 0
        transformation_matrix[4] * rcp_scale_y,
        transformation_matrix[5] * rcp_scale_y,
        transformation_matrix[6] * rcp_scale_y, // col 1
        transformation_matrix[8] * rcp_scale_z,
        transformation_matrix[9] * rcp_scale_z,
        transformation_matrix[10] * rcp_scale_z, // col 2
    ];

    let mut e0 = 0.0;

    let intermediate_quaternion = if rotation_matrix[8] >= 0.0 {
        let permutation_sum_diagonal = rotation_matrix[0] + rotation_matrix[4];
        let permutation_diff_diagonal = rotation_matrix[1] - rotation_matrix[3];
        let c = 1.0 + rotation_matrix[8];

        if permutation_sum_diagonal >= 0.0 {
            e0 = c + permutation_sum_diagonal;

            [
                rotation_matrix[5] - rotation_matrix[7],
                rotation_matrix[6] - rotation_matrix[2],
                permutation_diff_diagonal,
                e0,
            ]
        } else {
            e0 = c - permutation_sum_diagonal;

            [
                rotation_matrix[6] + rotation_matrix[2],
                rotation_matrix[5] + rotation_matrix[7],
                e0,
                permutation_diff_diagonal,
            ]
        }
    } else {
        let permutation_diff_diagonal = rotation_matrix[0] - rotation_matrix[4];
        let permutation_sum_diagonal = rotation_matrix[1] + rotation_matrix[3];
        let c = 1.0 - rotation_matrix[8];

        if permutation_diff_diagonal >= 0.0 {
            e0 = c + permutation_diff_diagonal;

            [
                e0,
                permutation_sum_diagonal,
                rotation_matrix[6] + rotation_matrix[2],
                rotation_matrix[5] - rotation_matrix[7],
            ]
        } else {
            e0 = c - permutation_diff_diagonal;
            [
                permutation_sum_diagonal,
                e0,
                rotation_matrix[5] + rotation_matrix[7],
                rotation_matrix[6] - rotation_matrix[2],
            ]
        }
    };

    let sqrt_e0 = e0.sqrt();

    let x2 =
        (intermediate_quaternion[0] * 0.5 * sqrt_e0) * (intermediate_quaternion[0] * 0.5 * sqrt_e0);

    let y2 =
        (intermediate_quaternion[1] * 0.5 * sqrt_e0) * (intermediate_quaternion[1] * 0.5 * sqrt_e0);

    let z2 =
        (intermediate_quaternion[2] * 0.5 * sqrt_e0) * (intermediate_quaternion[2] * 0.5 * sqrt_e0);

    let w2 =
        (intermediate_quaternion[3] * 0.5 * sqrt_e0) * (intermediate_quaternion[3] * 0.5 * sqrt_e0);

    let rcp_quaternion_magnitude = (x2 + y2 + z2 + w2).sqrt().recip();

    let quaternion_inner = [
        (intermediate_quaternion[0] * 0.5 * sqrt_e0) * rcp_quaternion_magnitude,
        (intermediate_quaternion[1] * 0.5 * sqrt_e0) * rcp_quaternion_magnitude,
        (intermediate_quaternion[2] * 0.5 * sqrt_e0) * rcp_quaternion_magnitude,
        (intermediate_quaternion[3] * 0.5 * sqrt_e0) * rcp_quaternion_magnitude,
    ];

    let translation = Translation {
        value: translation_inner,
    };

    let rotation = Rotation {
        value: quaternion_inner,
    };

    let scale = Scale {
        value: ScaleType::NonUniform(scale_inner),
    };


    (translation, rotation, scale)
}

pub fn transform(vector: [f32; 4], quaternion: [f32; 4]) -> [f32; 3] {
    let inv_scalar = 1.0 / vector[3];

    let norm_vector = [
        vector[0] * inv_scalar,
        vector[1] * inv_scalar,
        vector[2] * inv_scalar,
    ];

    let quaternion_vector = [quaternion[0], quaternion[1], quaternion[2]];

    let quaternion_scalar = quaternion[3];

    let quaternion_dot_vector = quaternion_vector[0] * norm_vector[0]
        + quaternion_vector[1] * norm_vector[1]
        + quaternion_vector[2] * norm_vector[2];

    let quaternion_vector_dot_quaternion_vector = quaternion_vector[0] * quaternion_vector[0]
        + quaternion_vector[1] * quaternion_vector[1]
        + quaternion_vector[2] * quaternion_vector[2];

    let quaternion_vector_cross_vector = [
        quaternion_vector[1] * norm_vector[2] - quaternion_vector[2] * norm_vector[1],
        quaternion_vector[2] * norm_vector[0] - quaternion_vector[0] * norm_vector[2],
        quaternion_vector[0] * norm_vector[1] - quaternion_vector[1] * norm_vector[0],
    ];

    [
        2.0 * quaternion_dot_vector * quaternion_vector[0]
            + (quaternion_scalar * quaternion_scalar - quaternion_vector_dot_quaternion_vector)
                * norm_vector[0]
            + 2.0 * quaternion_scalar * quaternion_vector_cross_vector[0],
        2.0 * quaternion_dot_vector * quaternion_vector[1]
            + (quaternion_scalar * quaternion_scalar - quaternion_vector_dot_quaternion_vector)
                * norm_vector[1]
            + 2.0 * quaternion_scalar * quaternion_vector_cross_vector[1],
        2.0 * quaternion_dot_vector * quaternion_vector[2]
            + (quaternion_scalar * quaternion_scalar - quaternion_vector_dot_quaternion_vector)
                * norm_vector[2]
            + 2.0 * quaternion_scalar * quaternion_vector_cross_vector[2],
    ]
}

#[cfg(test)]
mod transform_test {
    use crate::{
        decompose_transformation_matrix, forward, get_angle_axis_magnitude, get_axis_angle,
        get_euler_angle, get_rotation_matrix, get_transformation_matrix, right, transform, up,
        Rotation, Scale, ScaleType, Translation,
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

    #[test]
    fn calculate_decompose_transform() {
        const ROTATION_THRESHOLD: f32 = 0.0001;

        let translation = Translation {
            value: [23.3, 12.12, 100.29, 1.0],
        };
        let rotation = Rotation {
            value: [0.0815932, 0.2447796, 0.5303558, 0.8075569],
        };
        let scale = Scale {
            value: ScaleType::NonUniform([1.123, 2.0, 3.3]),
        };

        let transform_matrix = get_transformation_matrix(translation, rotation, scale);

        let (decomposed_translation, decomposed_rotation, decomposed_scale) =
            decompose_transformation_matrix(transform_matrix);

        assert!(translation.value[0].eq(&decomposed_translation.value[0]));
        assert!(translation.value[1].eq(&decomposed_translation.value[1]));
        assert!(translation.value[2].eq(&decomposed_translation.value[2]));

        assert!((rotation.value[0] - decomposed_rotation.value[0]).abs() < ROTATION_THRESHOLD);
        assert!((rotation.value[1] - decomposed_rotation.value[1]).abs() < ROTATION_THRESHOLD);
        assert!((rotation.value[2] - decomposed_rotation.value[2]).abs() < ROTATION_THRESHOLD);

        let scale = match scale.value {
            ScaleType::Uniform(uniform) => [uniform; 3],
            ScaleType::NonUniform(non_uniform) => non_uniform,
        };

        let decomposed_scale = match decomposed_scale.value {
            ScaleType::Uniform(uniform) => [uniform; 3],
            ScaleType::NonUniform(non_uniform) => non_uniform,
        };

        assert!(scale[0].eq(&decomposed_scale[0]));
        assert!(scale[1].eq(&decomposed_scale[1]));
        assert!(scale[2].eq(&decomposed_scale[2]));
    }

    #[test]
    fn calculate_transform() {
        const THRESHOLD: f32 = 0.0001;

        let translation = Translation {
            value: [12.2, 9.45, 11.0, 1.0],
        };
        let rotation = Rotation {
            value: [0.4780559, 0.3243005, 0.7935674, 0.1911612],
        };

        let transform = transform(translation.value, rotation.value);

        // C# System.Numerics.Vector4.transform result (4.040963, 4.363978, 17.99358, 1)

        const TARGET_RESULT: [f32; 3] = [4.040963, 4.363978, 17.99358];

        assert!((transform[0] - TARGET_RESULT[0]).abs() <= THRESHOLD);
        assert!((transform[1] - TARGET_RESULT[1]).abs() <= THRESHOLD);
        assert!((transform[2] - TARGET_RESULT[2]).abs() <= THRESHOLD);
    }
}
