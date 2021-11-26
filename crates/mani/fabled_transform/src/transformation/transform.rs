use crate::util::acos;
use crate::{Rotation, Scale, Translation};

#[rustfmt::skip]
pub fn get_rotation_matrix(rotation : Rotation) -> [f32; 9] {
    let (qx, qy, qz, qw) = (
        rotation.value[0],
        rotation.value[1],
        rotation.value[2],
        rotation.value[3],
    );

    let x2 = qx * qx;
    let y2 = qy * qy;
    let z2 = qz * qz;
    let xy = qx * qy;
    let xz = qx * qz;
    let yz = qy * qz;
    let wx = qw * qx;
    let wy = qw * qy;
    let wz = qw * qz;

    [
        1.0 - 2.0 * (y2 + z2), 2.0 * (xy + wz), 2.0 * (xz - wy),//col 0
        2.0 * (xy - wz), 1.0 - 2.0 * (x2 + z2), 2.0 * (yz + wx),//col 1
        2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (x2 + y2) //col 2
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

    let (qx, qy, qz, qw) = (
        rotation.value[0],
        rotation.value[1],
        rotation.value[2],
        rotation.value[3],
    );

    let scale_sq = (1.0 - qw * qw).max(0.0);

    let angle = 2.0 * acos(qw);

    if scale_sq < SQR_EPSILON {
        ([1.0, 0.0, 0.0], angle)
    } else {
        let inv_sqrt_scale = scale_sq.sqrt().recip();
        (
            [
                qx * inv_sqrt_scale,
                qy * inv_sqrt_scale,
                qz * inv_sqrt_scale,
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
    let (qx, qy, qz, qw) = (
        rotation.value[0],
        rotation.value[1],
        rotation.value[2],
        rotation.value[3],
    );

    let xx = qx * qx;
    let xy = qx * qy;
    let xz = qx * qz;
    let xw = qx * qw;

    let yy = qy * qy;
    let yz = qy * qz;
    let yw = qy * qw;

    let zz = qz * qz;
    let zw = qz * qw;

    let ww = qw * qw;

    let x = (-2.0 * (yz - xw)).atan2(ww - xx - yy + zz);

    let unsafe_y = 2.0 * (xz + yw);

    let y = unsafe_y.clamp(-1.0, 1.0).asin();

    let z = (-2.0 * (xy - zw)).atan2(ww + xx - yy - zz);

    [x, y, z]
}

#[cfg(test)]
mod transform_test {
    use crate::{
        get_angle_axis_magnitude, get_axis_angle, get_euler_angle, get_rotation_matrix,
        get_transformation_matrix, Rotation, Translation,
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
}
