use crate::util::acos;
use crate::{Rotation, Scale, ScaleType, Translation};

pub fn forward(rotation: Rotation) -> [f32; 3] {
    let [i, j, k, w] = rotation.value;

    let quat_dot = i * i + j * j + k * k;

    let k2 = k * 2.0;
    let w2 = w * 2.0;

    let result_upper = [k2 * i, k2 * j, k2 * k];

    let quaternion_scalar_mul_quat_dot = w * w - quat_dot;

    let result_middle = [0.0, 0.0, quaternion_scalar_mul_quat_dot];

    let result_bottom = [w2 * j, w2 * -i, 0.0];

    [
        result_upper[0] + result_middle[0] + result_bottom[0],
        result_upper[1] + result_middle[1] + result_bottom[1],
        result_upper[2] + result_middle[2] + result_bottom[2],
    ]
}

pub fn right(rotation: Rotation) -> [f32; 3] {
    let [i, j, k, w] = rotation.value;

    let quat_dot = i * i + j * j + k * k;

    let quaternion_scalar_mul_quat_dot = w * w - quat_dot;

    let i2 = i * 2.0;
    let w2 = w * 2.0;

    let result_upper = [i2 * i, i2 * j, i2 * k];

    let result_middle = [quaternion_scalar_mul_quat_dot, 0.0, 0.0];

    let result_bottom = [0.0, w2 * k, w2 * -j];

    [
        result_upper[0] + result_middle[0] + result_bottom[0],
        result_upper[1] + result_middle[1] + result_bottom[1],
        result_upper[2] + result_middle[2] + result_bottom[2],
    ]
}

#[rustfmt::skip]
pub fn get_rotation_matrix(rotation : Rotation) -> [f32; 9] {
    let [quat_i, quat_j, quat_k, quat_w] = rotation.value;
    
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

pub fn rotation_matrix_to_quaternion(rotation_matrix: [f32; 9]) -> Rotation {
    // 0, 3, 6,
    // 1, 4, 7,
    // 2, 5, 8,

    let matrix_00 = rotation_matrix[0];
    let matrix_11 = rotation_matrix[4];
    let matrix_22 = rotation_matrix[8];

    let matrix_21 = rotation_matrix[5];
    let matrix_12 = rotation_matrix[7];
    let matrix_02 = rotation_matrix[6];
    let matrix_20 = rotation_matrix[2];
    let matrix_10 = rotation_matrix[1];
    let matrix_01 = rotation_matrix[3];

    let w = 0.0f32
        .max(1.0 + matrix_00 + matrix_11 + matrix_22)
        .sqrt()
        * 0.5;

    let i = 0.0f32
        .max(1.0 + matrix_00 - matrix_11 - matrix_22)
        .sqrt()
        * 0.5;

    let j = 0.0f32
        .max(1.0 - matrix_00 + matrix_11 - matrix_22)
        .sqrt()
        * 0.5;

    let k = 0.0f32
        .max(1.0 - matrix_00 - matrix_11 + matrix_22)
        .sqrt()
        * 0.5;

    let signed_i = i.copysign(matrix_21 - matrix_12);
    let signed_j = j.copysign(matrix_02 - matrix_20);
    let signed_k = k.copysign(matrix_10 - matrix_01);

    Rotation{
        value: [signed_i, signed_j, signed_k,w]
    }
}

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

    let [quat_i, quat_j, quat_k, quat_w] = rotation.value;

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

pub fn axis_angle_to_quaternion(axis: [f32; 3], angle: f32) -> Rotation {
    let [x, y, z] = axis;

    let (sin_half_angle, cos_half_angle) = (angle * 0.5).sin_cos();

    let quat_i = x * sin_half_angle;
    let quat_j = y * sin_half_angle;
    let quat_k = z * sin_half_angle;
    let quat_w = cos_half_angle;

    Rotation {
        value: [quat_i, quat_j, quat_k, quat_w],
    }
}

pub fn get_angle_axis_magnitude(rotation: Rotation) -> [f32; 3] {
    let ([axis_x, axis_y, axis_z], angle) = get_axis_angle(rotation);

    [axis_x * angle, axis_y * angle, axis_z * angle]
}

pub fn angle_axis_mag_to_quaternion(axis_mag: [f32; 3]) -> Rotation {
    let angle =
        (axis_mag[0] * axis_mag[0] + axis_mag[1] * axis_mag[1] + axis_mag[2] * axis_mag[2]).sqrt();

    let rcp_angle = angle.recip();

    let axis = [
        axis_mag[0] * rcp_angle,
        axis_mag[1] * rcp_angle,
        axis_mag[2] * rcp_angle,
    ];

    axis_angle_to_quaternion(axis, angle)
}

pub fn get_euler_angle(rotation: Rotation) -> [f32; 3] {
    let [quat_i, quat_j, quat_k, quat_w] = rotation.value;

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

    let unsafe_y = 2.0 * (quat_ik + quat_jw);

    let x = (-2.0 * (quat_jk - quat_iw)).atan2(quat_ww - quat_ii - quat_jj + quat_kk);

    let y = unsafe_y.clamp(-1.0, 1.0).asin();

    let z = (-2.0 * (quat_ij - quat_kw)).atan2(quat_ww + quat_ii - quat_jj - quat_kk);

    [x, y, z]
}

pub fn euler_to_quaternion(euler: [f32; 3]) -> Rotation {
    let [x, y, z] = euler;

    let (sin_quat_i, cos_quat_iw) = (x * 0.5).sin_cos();
    let (sin_quat_j, cos_quat_jw) = (y * 0.5).sin_cos();
    let (sin_quat_k, cos_quat_kw) = (z * 0.5).sin_cos();

    let [quat_axis_ix, _, _, quat_axis_iw] = [sin_quat_i, 0.0, 0.0, cos_quat_iw];

    let [_, quat_axis_jy, _, quat_axis_jw] = [0.0, sin_quat_j, 0.0, cos_quat_jw];

    let [_, _, quat_axis_kz, quat_axis_kw] = [0.0, 0.0, sin_quat_k, cos_quat_kw];

    let w = quat_axis_iw * quat_axis_jw;

    let i = quat_axis_ix * quat_axis_jw;

    let j = quat_axis_iw * quat_axis_jy;

    let k = quat_axis_ix * quat_axis_jy;

    Rotation {
        value: [
            i * quat_axis_kw + j * quat_axis_kz,
            -i * quat_axis_kz + j * quat_axis_kw,
            w * quat_axis_kz + k * quat_axis_kw,
            w * quat_axis_kw - k * quat_axis_kz,
        ],
    }
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

pub fn transform(vector: [f32; 4], quaternion: Rotation) -> [f32; 3] {
    let [i, j, k, w] = quaternion.value;

    let inv_scalar = 1.0 / vector[3];


    let norm_vector = [
        vector[0] * inv_scalar,
        vector[1] * inv_scalar,
        vector[2] * inv_scalar,
    ];

    let quaternion_vector = [i, j, k];

    let quaternion_scalar = w;

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

#[rustfmt::skip]
pub fn inverse_rotation_matrix(rotation_matrix: [f32; 9]) -> [f32; 9] {
    let cross_yz = [
        rotation_matrix[4] * rotation_matrix[8] - rotation_matrix[5] * rotation_matrix[7],
        rotation_matrix[5] * rotation_matrix[6] - rotation_matrix[3] * rotation_matrix[8],
        rotation_matrix[3] * rotation_matrix[7] - rotation_matrix[4] * rotation_matrix[6],
    ];
    let cross_zx = [
        rotation_matrix[7] * rotation_matrix[2] - rotation_matrix[8] * rotation_matrix[1],
        rotation_matrix[8] * rotation_matrix[0] - rotation_matrix[6] * rotation_matrix[2],
        rotation_matrix[6] * rotation_matrix[1] - rotation_matrix[7] * rotation_matrix[0],
    ];

    let cross_xy = [
        rotation_matrix[1] * rotation_matrix[5] - rotation_matrix[2] * rotation_matrix[4],
        rotation_matrix[2] * rotation_matrix[3] - rotation_matrix[0] * rotation_matrix[5],
        rotation_matrix[0] * rotation_matrix[4] - rotation_matrix[1] * rotation_matrix[3],
    ];

    let determinant = rotation_matrix[6] * cross_xy[0]
        + rotation_matrix[7] * cross_xy[1]
        + rotation_matrix[8] * cross_xy[2];

    assert!(determinant.ne(&0.0));

    let inv_determinant = 1.0 / determinant;

    [
        cross_yz[0] * inv_determinant, cross_zx[0] * inv_determinant, cross_xy[0] * inv_determinant, // col 0
        cross_yz[1] * inv_determinant, cross_zx[1] * inv_determinant, cross_xy[1] * inv_determinant, // col 1
        cross_yz[2] * inv_determinant, cross_zx[2] * inv_determinant, cross_xy[2] * inv_determinant, // col 2
    ]
}

#[cfg(test)]
mod transform_test {
    use crate::{
        angle_axis_mag_to_quaternion, axis_angle_to_quaternion, decompose_transformation_matrix,
        euler_to_quaternion, forward, get_angle_axis_magnitude, get_axis_angle, get_euler_angle,
        get_rotation_matrix, get_transformation_matrix, right, rotation_matrix_to_quaternion,
        transform, Rotation, Scale, ScaleType, Translation,
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

        assert!(position
            .value
            .eq(&decomposed_translation.extend(1.0).to_array()));

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

        // retrieved from a proven game engine. Editor rounds to integer, so there will
        // be an error_threshold, sine we don't round our calculation to integer
        let proven_forward_direction = [0.5, -0.3, 0.8];
        let proven_right_direction = [0.8, 0.4, -0.4];

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

        assert!(forward_difference[0] <= THRESHOLD);
        assert!(forward_difference[1] <= THRESHOLD);
        assert!(forward_difference[2] <= THRESHOLD);

        assert!(right_difference[0] <= THRESHOLD);
        assert!(right_difference[1] <= THRESHOLD);
        assert!(right_difference[2] <= THRESHOLD);
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

        let transform = transform(translation.value, rotation);

        // C# System.Numerics.Vector4.transform result (4.040963, 4.363978, 17.99358, 1)

        const TARGET_RESULT: [f32; 3] = [4.040963, 4.363978, 17.99358];

        assert!((transform[0] - TARGET_RESULT[0]).abs() <= THRESHOLD);
        assert!((transform[1] - TARGET_RESULT[1]).abs() <= THRESHOLD);
        assert!((transform[2] - TARGET_RESULT[2]).abs() <= THRESHOLD);
    }


    #[test]
    fn euler_to_quat() {
        const THRESHOLD: f32 = 0.1;

        let euler = [
            45.25_f32.to_radians(),
            11.11_f32.to_radians(),
            1.123_f32.to_radians(),
        ];

        let quat = euler_to_quaternion(euler).value;
        let proven_quat = glam::Quat::from_euler(glam::EulerRot::XYZ, euler[0], euler[1], euler[2]);


        let difference = [
            (quat[0] - proven_quat.x).abs(),
            (quat[1] - proven_quat.y).abs(),
            (quat[2] - proven_quat.z).abs(),
            (quat[3] - proven_quat.w).abs(),
        ];

        assert!(difference[0] <= THRESHOLD);
        assert!(difference[1] <= THRESHOLD);
        assert!(difference[2] <= THRESHOLD);
        assert!(difference[3] <= THRESHOLD);
    }

    #[test]
    fn angle_axis_to_quat() {
        const THRESHOLD: f32 = 0.0001;

        let quaternion = Rotation {
            value: [0.3004045, -0.03679, 0.4609919, 0.8342003],
        };

        let (axis, angle) = get_axis_angle(quaternion);

        let quat = axis_angle_to_quaternion(axis, angle);

        let difference = [
            (quat.value[0] - quaternion.value[0]).abs(),
            (quat.value[1] - quaternion.value[1]).abs(),
            (quat.value[2] - quaternion.value[2]).abs(),
            (quat.value[3] - quaternion.value[3]).abs(),
        ];

        assert!(difference[0] <= THRESHOLD);
        assert!(difference[1] <= THRESHOLD);
        assert!(difference[2] <= THRESHOLD);
        assert!(difference[3] <= THRESHOLD);
    }

    #[test]
    fn angle_axis_mag_to_quat() {
        const THRESHOLD: f32 = 0.0001;

        let quaternion = Rotation {
            value: [0.5085492, -0.0231115, 0.2273495, 0.8301541],
        };

        let angle_mag = get_angle_axis_magnitude(quaternion);

        let quat = angle_axis_mag_to_quaternion(angle_mag);

        let difference = [
            (quaternion.value[0] - quat.value[0]).abs(),
            (quaternion.value[1] - quat.value[1]).abs(),
            (quaternion.value[2] - quat.value[2]).abs(),
            (quaternion.value[3] - quat.value[3]).abs(),
        ];

        assert!(difference[0] <= THRESHOLD);
        assert!(difference[1] <= THRESHOLD);
        assert!(difference[2] <= THRESHOLD);
        assert!(difference[3] <= THRESHOLD);
    }

    #[test]
    fn rotation_matrix_to_quat() {
        const THRESHOLD: f32 = 0.0001;

        let quaternion = Rotation {
            value: [0.5085492, -0.0231115, 0.2273495, 0.8301541],
        };

        let rotation_matrix = get_rotation_matrix(quaternion);

        let quat = rotation_matrix_to_quaternion(rotation_matrix);

        let difference = [
            (quaternion.value[0] - quat.value[0]).abs(),
            (quaternion.value[1] - quat.value[1]).abs(),
            (quaternion.value[2] - quat.value[2]).abs(),
            (quaternion.value[3] - quat.value[3]).abs(),
        ];

        assert!(difference[0] <= THRESHOLD);
        assert!(difference[1] <= THRESHOLD);
        assert!(difference[2] <= THRESHOLD);
        assert!(difference[3] <= THRESHOLD);
    }
}
