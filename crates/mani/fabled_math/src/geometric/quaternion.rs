use crate::geometric::quaternion_math::inverse;
use crate::math::{dot, mul_add};
use crate::{cross, Vector3, Vector4};
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

// todo convert simd type to respective struct type in the math module vbn

#[derive(Clone, Copy)]
pub struct Quaternion {
    pub value: std::simd::f32x4,
}

impl Default for Quaternion {
    fn default() -> Self {
        Self {
            value: [0.0, 0.0, 0.0, 1.0].into(),
        }
    }
}

impl Quaternion {
    pub const IDENTITY: Quaternion = Quaternion {
        value: std::simd::f32x4::from_array([0.0, 0.0, 0.0, 1.0]),
    };

    pub const fn set(i: f32, j: f32, k: f32, w: f32) -> Quaternion {
        Quaternion {
            value: std::simd::f32x4::from_array([i, j, k, w]),
        }
    }
}

impl Display for Quaternion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let array_representation: [f32; 4] = *self.value.as_array();

        write!(
            f,
            "Quaternion (i : {}, j : {}, k : {}, w : {})",
            array_representation[0],
            array_representation[1],
            array_representation[2],
            array_representation[3]
        )
    }
}

impl Add for Quaternion {
    type Output = Quaternion;

    fn add(self, rhs: Self) -> Self::Output {
        Quaternion {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign for Quaternion {
    fn add_assign(&mut self, rhs: Self) {
        self.value = self.value + rhs.value
    }
}

impl Sub for Quaternion {
    type Output = Quaternion;

    fn sub(self, rhs: Self) -> Self::Output {
        Quaternion {
            value: self.value - rhs.value,
        }
    }
}

impl SubAssign for Quaternion {
    fn sub_assign(&mut self, rhs: Self) {
        self.value = self.value - rhs.value;
    }
}


impl Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result *= rhs;

        result
    }
}

impl MulAssign for Quaternion {
    fn mul_assign(&mut self, rhs: Self) {
        let scalar_lhs = self.value[3];
        let scalar_rhs = rhs.value[3];

        let complex_lhs = Vector4 { value: self.value }.trunc_vec3();
        let complex_rhs = Vector4 { value: rhs.value }.trunc_vec3();

        let imaginary = (complex_rhs * scalar_lhs + complex_lhs * scalar_rhs).value
            + cross(complex_lhs.value, complex_rhs.value);

        let real = scalar_lhs * scalar_rhs - dot(complex_lhs.value, complex_rhs.value);


        self.value = imaginary + Vector4::set(0.0, 0.0, 0.0, real).value;
    }
}

impl Div for Quaternion {
    type Output = Quaternion;
    fn div(self, rhs: Self) -> Self::Output {
        self * inverse(rhs)
    }
}

impl DivAssign for Quaternion {
    fn div_assign(&mut self, rhs: Self) {
        *self = Quaternion { value: self.value } * inverse(rhs)
    }
}

pub mod quaternion_math {
    use crate::math::{
        component_sum, copysign, cos, dot, length, lerp, mul_add, normalize, rcp, select, sin, sqrt,
    };
    use crate::{ror, Matrix3x3, Matrix4x4, Quaternion, Vector3, Vector4};

    pub fn rotate_x(angle_radians: f32) -> Quaternion {
        const W_VECTOR4: Vector4 = Vector4::set(0.0, 0.0, 0.0, 1.0);
        const X_VECTOR4: Vector4 = Vector4::set(1.0, 0.0, 0.0, 0.0);

        let half_angle: Vector4 = Vector4::splat(angle_radians * 0.5f32);

        let quaternion_w = cos(half_angle.value) * W_VECTOR4.value;
        let quaternion_i = sin(half_angle.value) * X_VECTOR4.value;

        Quaternion {
            value: quaternion_i + quaternion_w,
        }
    }

    pub fn rotate_y(angle_radians: f32) -> Quaternion {
        const W_VECTOR4: Vector4 = Vector4::set(0.0, 0.0, 0.0, 1.0);
        const Y_VECTOR4: Vector4 = Vector4::set(0.0, 1.0, 0.0, 0.0);
        let half_angle: Vector4 = Vector4::splat(angle_radians * 0.5f32);

        let quaternion_w = cos(half_angle.value) * W_VECTOR4.value;
        let quaternion_j = sin(half_angle.value) * Y_VECTOR4.value;

        Quaternion {
            value: quaternion_j * quaternion_w,
        }
    }

    pub fn rotate_z(angle_radians: f32) -> Quaternion {
        const W_VECTOR4: Vector4 = Vector4::set(0.0, 0.0, 0.0, 1.0);
        const Z_VECTOR4: Vector4 = Vector4::set(0.0, 0.0, 1.0, 0.0);
        let half_angle: Vector4 = Vector4::splat(angle_radians * 0.5f32);

        let quaternion_w = cos(half_angle.value) * W_VECTOR4.value;
        let quaternion_k = sin(half_angle.value) * Z_VECTOR4.value;


        Quaternion {
            value: quaternion_k * quaternion_w,
        }
    }

    pub fn angle_axis_mag(axis_mag: Vector3) -> Quaternion {
        let angle = length(axis_mag.value);

        let rcp_angle = rcp(Vector4::splat(angle).value);

        let axis = axis_mag.value * rcp_angle;

        angle_axis(Vector3 { value: axis }, angle)
    }


    pub fn angle_axis(normalized_axis: Vector3, angle_radians: f32) -> Quaternion {
        let half_angle: Vector4 = Vector4::splat(angle_radians * 0.5f32);
        let quaternion_imaginary = sin(half_angle.value) * normalized_axis.value;

        let quaternion_real = cos(half_angle.value) * Vector4::set(0.0, 0.0, 0.0, 1.0).value;

        Quaternion {
            value: quaternion_imaginary + quaternion_real,
        }
    }

    pub fn from_rotation_matrix(rotation_matrix: Matrix3x3) -> Quaternion {
        let rotation_representation = rotation_matrix.value;

        let sign_lhs = Vector3::set(
            rotation_representation[6],
            rotation_representation[8],
            rotation_representation[1],
        );

        let sign_rhs = Vector3::set(
            rotation_representation[9],
            rotation_representation[2],
            rotation_representation[4],
        );

        let sign_result = sign_lhs - sign_rhs;

        let diagonal_matrix_vector: Vector3 = Vector3::set(
            rotation_representation[0],
            rotation_representation[5],
            rotation_representation[10],
        );

        const X_VECTOR4: Vector4 = Vector4::set(1.0, -1.0, -1.0, -1.0);

        const RIGHT_VECTOR3: Vector3 = Vector3::RIGHT;

        let quaternion_unsigned = Quaternion {
            value: sqrt(
                [
                    component_sum(
                        diagonal_matrix_vector.value * X_VECTOR4.value + RIGHT_VECTOR3.value,
                    ),
                    component_sum(
                        diagonal_matrix_vector.value * ror::<1>(X_VECTOR4.value)
                            + RIGHT_VECTOR3.value,
                    ),
                    component_sum(
                        diagonal_matrix_vector.value * ror::<2>(X_VECTOR4.value)
                            + RIGHT_VECTOR3.value,
                    ),
                    component_sum(
                        (diagonal_matrix_vector.value * Vector4::ONE.value) + RIGHT_VECTOR3.value,
                    ),
                ]
                .into(),
            ) * Vector4::splat(0.5).value,
        };

        Quaternion {
            value: copysign(quaternion_unsigned.value, sign_result.value),
        }
    }

    #[rustfmt::skip]
    pub fn to_rotation_matrix(quaternion: Quaternion) -> Matrix3x3 {
        
        // i * i,       j * j,      k * k,        w * w
        let quaternion_sqr = quaternion.value * quaternion.value;

        // i * k,       j * w,      k * i,      w * j
        let quaternion_rot_2 = quaternion.value * ror::<2>(quaternion.value);

        // i * j,       j * k,      k * w,       w * i
        let quaternion_rot_3 = quaternion.value * ror::<3>(quaternion.value);

        let identity_mat4 = Matrix4x4::default();

        let splat_2 = std::simd::f32x16::splat(2.0);

        let intermediate_step = [
            -(quaternion_sqr[1] + quaternion_sqr[2]), (quaternion_rot_3[0] + quaternion_rot_3[2]), (quaternion_rot_2[0] - quaternion_rot_2[1]), 0.0,
            (quaternion_rot_3[0] - quaternion_rot_3[2]), -(quaternion_sqr[0] + quaternion_sqr[2]), (quaternion_rot_3[1] + quaternion_rot_3[3]), 0.0,
            (quaternion_rot_2[0] + quaternion_rot_2[1]), (quaternion_rot_3[1] - quaternion_rot_3[3]), -(quaternion_sqr[0] + quaternion_sqr[1]), 0.0,
            0.0, 0.0, 0.0, 0.0
        ];

        let simd_repr_step = std::simd::f32x16::from_array(intermediate_step);

        Matrix3x3 { value: identity_mat4.value + simd_repr_step * splat_2 }
    }

    pub fn to_angle_axis(quaternion: Quaternion) -> (Vector3, f32) {
        let real: f32 = quaternion.value[3];

        let angle = 2.0 * real.acos();

        let scale = Vector4::splat(1.0 - real * real);

        let mask = scale.value.lanes_lt(Vector4::splat(f32::EPSILON).value);

        let axis = select(
            Vector3::set(1.0, 0.0, 0.0).value,
            quaternion.value * rcp(sqrt(scale.value)),
            mask,
        );

        (Vector3 { value: axis }, angle)
    }


    pub fn conjugate(quaternion: Quaternion) -> Quaternion {
        Quaternion {
            value: quaternion.value * Vector4::set(-1.0, -1.0, -1.0, 1.0).value,
        }
    }


    pub fn inverse(quaternion: Quaternion) -> Quaternion {
        Quaternion {
            value: conjugate(quaternion).value
                * rcp(Vector4::splat(dot(quaternion.value, quaternion.value)).value),
        }
    }

    pub fn forward(quaternion: Quaternion) -> Vector3 {
        Vector3::FORWARD * quaternion
    }

    pub fn right(quaternion: Quaternion) -> Vector3 {
        Vector3::RIGHT * quaternion
    }

    pub fn up(quaternion: Quaternion) -> Vector3 {
        Vector3::UP * quaternion
    }

    pub fn normalized_lerp(
        start_quaternion: Quaternion,
        target_quaternion: Quaternion,
        delta: f32,
    ) -> Quaternion {
        let target_quaternion = select(
            target_quaternion.value,
            -target_quaternion.value,
            std::simd::mask32x4::splat(
                dot(start_quaternion.value, target_quaternion.value).ge(&0.0),
            ),
        );

        let linear_interpolated_quaternion = lerp(
            start_quaternion.value,
            target_quaternion,
            Vector4::splat(delta).value,
        );

        Quaternion {
            value: normalize(linear_interpolated_quaternion),
        }
    }

    pub fn slerp(
        start_quaternion: Quaternion,
        target_quaternion: Quaternion,
        delta: f32,
    ) -> Quaternion {
        let normalized_dot = dot(start_quaternion.value, target_quaternion.value);

        let angle = normalized_dot.acos();

        let rcp_sin_angle = angle.sin().recip();

        let weight_start: Vector4 = Vector4::splat(((1.0 - delta) * angle).sin() * rcp_sin_angle);
        let weight_target: Vector4 = Vector4::splat((delta * angle).sin() * rcp_sin_angle);

        let result_slerp = mul_add(
            start_quaternion.value,
            weight_start.value,
            target_quaternion.value * weight_target.value,
        );


        Quaternion {
            value: result_slerp,
        }
    }
}

// todo Write test for all quaternion and quaternion math implementation
#[cfg(test)]
mod quaternion_test {}
