use crate::math::{component_sum, copysign, cos, dot, length, rcp, saturate, select, sin, sqrt};
use crate::{cross, ror, Matrix3x3, Matrix4x4, Vector3, Vector4};
use std::fmt::{Display, Formatter};
use std::ops::{Div, DivAssign, Mul, MulAssign};

// todo convert simd type to respective struct type in the math module vbn

#[allow(clippy::suspicious_arithmetic_impl)]
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

    pub fn set_from_matrix(matrix: std::simd::f32x16) -> Quaternion {
        todo!()
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


impl Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Self) -> Self::Output {
        let scalar_lhs = self.value[3];
        let scalar_rhs = rhs.value[3];

        let complex_lhs = Vector4 { value: self.value }.trunc_vec3();
        let complex_rhs = Vector4 { value: rhs.value }.trunc_vec3();

        let imaginary = (complex_rhs * scalar_lhs + complex_lhs * scalar_rhs).value
            + cross(complex_lhs.value, complex_rhs.value);

        let real = scalar_lhs * scalar_rhs - dot(complex_lhs.value, complex_rhs.value);


        Quaternion {
            value: imaginary + Vector4::set(0.0, 0.0, 0.0, real).value,
        }
    }
}

impl MulAssign for Quaternion {
    fn mul_assign(&mut self, rhs: Self) {
        let scalar_lhs = self.value[3];
        let scalar_rhs = rhs.value[3];

        let complex_lhs = Vector4 { value: self.value }.trunc_vec3();
        let complex_rhs = Vector4 { value: rhs.value }.trunc_vec3();

        let img = (complex_rhs * scalar_lhs + complex_lhs * scalar_rhs).value
            + cross(complex_lhs.value, complex_rhs.value);

        let real = scalar_lhs * scalar_rhs - dot(complex_lhs.value, complex_rhs.value);


        self.value = img + Vector4::set(0.0, 0.0, 0.0, real).value;
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

pub fn rotate_x(angle_radians: f32) -> Quaternion {
    const W_VECTOR4: Vector4 = Vector4::set(0.0, 0.0, 0.0, 1.0);
    const X_VECTOR4: Vector4 = Vector4::set(1.0, 0.0, 0.0, 0.0);
    let half_angle: std::simd::f32x4 = std::simd::f32x4::splat(angle_radians * 0.5f32);

    let quaternion_w = cos(half_angle) * W_VECTOR4.value;
    let quaternion_i = sin(half_angle) * X_VECTOR4.value;

    Quaternion {
        value: quaternion_i + quaternion_w,
    }
}

pub fn rotate_y(angle_radians: f32) -> Quaternion {
    const W_VECTOR4: Vector4 = Vector4::set(0.0, 0.0, 0.0, 1.0);
    const Y_VECTOR4: Vector4 = Vector4::set(0.0, 1.0, 0.0, 0.0);
    let half_angle: std::simd::f32x4 = std::simd::f32x4::splat(angle_radians * 0.5f32);

    let quaternion_w = cos(half_angle) * W_VECTOR4.value;
    let quaternion_j = sin(half_angle) * Y_VECTOR4.value;

    Quaternion {
        value: quaternion_j * quaternion_w,
    }
}

pub fn rotate_z(angle_radians: f32) -> Quaternion {
    const W_VECTOR4: Vector4 = Vector4::set(0.0, 0.0, 0.0, 1.0);
    const Z_VECTOR4: Vector4 = Vector4::set(0.0, 0.0, 1.0, 0.0);
    let half_angle: std::simd::f32x4 = std::simd::f32x4::splat(angle_radians * 0.5f32);

    let quaternion_w = cos(half_angle) * W_VECTOR4.value;
    let quaternion_k = sin(half_angle) * Z_VECTOR4.value;


    Quaternion {
        value: quaternion_k * quaternion_w,
    }
}

pub fn angle_axis_mag(axis_mag: Vector3) -> Quaternion {
    let angle = length(axis_mag.value);

    let rcp_angle = rcp(std::simd::f32x4::splat(angle));

    let axis = axis_mag.value * rcp_angle;

    angle_axis(Vector3 { value: axis }, angle)
}


pub fn angle_axis(normalized_axis: Vector3, angle_radians: f32) -> Quaternion {
    let half_angle = std::simd::f32x4::splat(angle_radians * 0.5f32);
    let quat_img = sin(half_angle) * normalized_axis.value;

    let quat_real = cos(half_angle) * std::simd::f32x4::from_array([0.0, 0.0, 0.0, 1.0]);

    Quaternion {
        value: quat_img + quat_real,
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

    const VEC4_X: Vector4 = Vector4::set(1.0, -1.0, -1.0, -1.0);

    const VEC3_RIGHT: Vector3 = Vector3::RIGHT;

    let quaternion_w =
        component_sum((diagonal_matrix_vector.value * Vector4::ONE.value) + VEC3_RIGHT.value)
            .sqrt()
            * 0.5;

    let quaternion_i =
        component_sum(diagonal_matrix_vector.value * VEC4_X.value + VEC3_RIGHT.value).sqrt() * 0.5;

    let quaternion_j =
        component_sum(diagonal_matrix_vector.value * ror::<1>(VEC4_X.value) + VEC3_RIGHT.value)
            .sqrt()
            * 0.5;

    let quaternion_k =
        component_sum(diagonal_matrix_vector.value * ror::<2>(VEC4_X.value) + VEC3_RIGHT.value)
            .sqrt()
            * 0.5;

    let quaternion_unsigned = Vector4::set(quaternion_i, quaternion_j, quaternion_k, quaternion_w);

    let quaternion_result = copysign(quaternion_unsigned.value, sign_result.value);

    Quaternion {
        value: quaternion_result,
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

  let intermediate_step =  [
        -(quaternion_sqr[1] + quaternion_sqr[2]), (quaternion_rot_3[0] + quaternion_rot_3[2]), (quaternion_rot_2[0] - quaternion_rot_2[1]), 0.0,
        (quaternion_rot_3[0] - quaternion_rot_3[2]), -(quaternion_sqr[0] + quaternion_sqr[2]), (quaternion_rot_3[1] + quaternion_rot_3[3]), 0.0,
        (quaternion_rot_2[0] + quaternion_rot_2[1]), (quaternion_rot_3[1] - quaternion_rot_3[3]), -(quaternion_sqr[0] + quaternion_sqr[1]), 0.0,
        0.0, 0.0, 0.0, 0.0
    ];

    let simd_repr_step = std::simd::f32x16::from_array(intermediate_step);
    
    let result = identity_mat4.value + simd_repr_step * splat_2;

    Matrix3x3{ value: result }

}

pub fn to_angle_axis(quaternion: Quaternion) -> (Vector3, f32) {
    let real: f32 = quaternion.value[3];

    let angle = 2.0 * real.acos();

    let scale = std::simd::f32x4::splat(1.0 - real * real);

    let mask = std::simd::mask32x4::splat(scale[0] < f32::EPSILON);

    let axis = select(
        Vector3::set(1.0, 0.0, 0.0).value,
        quaternion.value * rcp(sqrt(scale)),
        mask,
    );

    (Vector3 { value: axis }, angle)
}


pub fn conjugate(quaternion: Quaternion) -> Quaternion {
    Quaternion {
        value: quaternion.value * std::simd::f32x4::from_array([-1.0, -1.0, -1.0, 1.0]),
    }
}

pub fn inverse(quaternion: Quaternion) -> Quaternion {
    Quaternion {
        value: conjugate(quaternion).value
            * rcp(std::simd::f32x4::splat(dot(
                quaternion.value,
                quaternion.value,
            ))),
    }
}

#[cfg(test)]
mod quaternion_test {
    use crate::math::normalize;
    use crate::{
        angle_axis, from_rotation_matrix, ror, rotate_x, to_angle_axis, to_rotation_matrix,
        Quaternion, Vector3,
    };

    #[test]
    fn rotation_x_quaternion_test() {
        let non_normalized_vector3 = Vector3::set(0.5, 0.2, 0.1);

        println!("rotate 2 {:?}", ror::<2>(non_normalized_vector3.value));

        let normalized_vector3 = Vector3 {
            value: normalize(non_normalized_vector3.value),
        };

        println!("normalized vector : {}", normalized_vector3);

        let quat_x = angle_axis(normalized_vector3, 45.0f32.to_radians());


        let quat_y = Quaternion {
            value: [0.2078066, 0.0914342, 0.1157831, 0.9669801].into(),
        };

        println!("start {}", quat_x);

        let new = quat_x * quat_y;
        println!("end {}", new);


        println!("reverse {}", new / quat_y);

        println!("{}", to_angle_axis(quat_x).0);

        panic!()
    }

    #[test]
    fn rotation_to_matrix() {
        let quaternion = Quaternion {
            value: [0.5085492, -0.0231115, 0.2273495, 0.8301541].into(),
        };

        let rotation_matrix = to_rotation_matrix(quaternion);

        println!("{}", rotation_matrix);

        let quat = from_rotation_matrix(rotation_matrix);

        println!("{}", quat);

        panic!()
    }
}
