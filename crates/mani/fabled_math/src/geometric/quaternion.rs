use crate::geometric::quaternion_math::inverse;
use crate::math::{dot, mul_add};
use crate::{cross, Vector3, Vector4};
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Quaternion {
    pub value: std::simd::f32x4,
}

impl Default for Quaternion {
    fn default() -> Self {
        Quaternion::IDENTITY
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


    pub  fn additive_form(
        real_quaternion: Quaternion,
        pure_quaternion: Quaternion,
    ) -> Quaternion {
        use std::simd::Which;
        Quaternion {
            value: std::simd::simd_swizzle!(
                real_quaternion.value,
                pure_quaternion.value,
                [
                    Which::Second(0),
                    Which::Second(1),
                    Which::Second(2),
                    Which::First(3)
                ]
            ),
        }
    }

    pub const fn to_primitive(self) -> [f32; 4] {
        self.value.to_array()
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
        self.value += rhs.value
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
        self.value -= rhs.value;
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

        let complex_lhs = Vector3 { value: self.value } * Vector3::ONE;
        let complex_rhs = Vector3 { value: rhs.value } * Vector3::ONE;

        let real = scalar_lhs * scalar_rhs - dot(complex_lhs.value, complex_rhs.value);


        let imaginary_intermediate = Vector3 {
            value: mul_add(
                complex_rhs.value,
                std::simd::f32x4::splat(scalar_lhs),
                complex_lhs.value,
            ),
        };

        let imaginary = (imaginary_intermediate * scalar_rhs).value
            + cross(complex_rhs.value, complex_lhs.value);


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
            value: quaternion_j + quaternion_w,
        }
    }

    pub fn rotate_z(angle_radians: f32) -> Quaternion {
        const W_VECTOR4: Vector4 = Vector4::set(0.0, 0.0, 0.0, 1.0);
        const Z_VECTOR4: Vector4 = Vector4::set(0.0, 0.0, 1.0, 0.0);
        let half_angle: Vector4 = Vector4::splat(angle_radians * 0.5f32);

        let quaternion_w = cos(half_angle.value) * W_VECTOR4.value;
        let quaternion_k = sin(half_angle.value) * Z_VECTOR4.value;


        Quaternion {
            value: quaternion_k + quaternion_w,
        }
    }

    pub fn from_angle_axis_mag(axis_mag: Vector3) -> Quaternion {
        let angle = length(axis_mag.value);

        let rcp_angle = rcp(Vector4::splat(angle).value);

        let axis = axis_mag.value * rcp_angle;

        from_angle_axis(Vector3 { value: axis }, angle)
    }


    pub fn from_angle_axis(normalized_axis: Vector3, angle_radians: f32) -> Quaternion {
        let half_angle: Vector4 = Vector4::splat(angle_radians * 0.5f32);
        let quaternion_imaginary = sin(half_angle.value) * normalized_axis.value;

        let quaternion_real = cos(half_angle.value) * Vector4::set(0.0, 0.0, 0.0, 1.0).value;

        Quaternion {
            value: quaternion_imaginary + quaternion_real,
        }
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

        (Vector3 { value: axis } * Vector3::ONE , angle)
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

    pub fn real_quaternion(quaternion: Quaternion) -> Quaternion {
        Quaternion {
            value: quaternion.value * Vector4::set(0.0, 0.0, 0.0, 1.0).value,
        }
    }

    pub fn pure_quaternion(quaternion: Quaternion) -> Quaternion {
        Quaternion {
            value: quaternion.value * Vector3::ONE.value,
        }
    }

    pub fn quaternion_log(quaternion : Quaternion) -> Quaternion {

        todo!()
    }

    pub fn quaternion_exp(quaternion : Quaternion) -> Quaternion{

        let scalar : f32 = quaternion.value[3];
        let vector : Vector3 = Vector3{ value: quaternion.value * Vector3::ONE.value };

        let quaternion_length : f32 = length(vector.value);

        let (sin_len, cos_len) = quaternion_length.sin_cos();

       let real_quaternion = vector * quaternion_length.recip() * sin_len;

        let pure_quaternion = Vector4::set(0.0, 0.0, 0.0, cos_len);

        let result = Vector4{ value: real_quaternion.value + pure_quaternion.value } * scalar.exp();

        Quaternion{ value: result.value }

    }

    pub fn quaternion_pow(quaternion: Quaternion, factor: f32) -> Quaternion {

        todo!()

    }

}

// todo Write test for all quaternion and quaternion math implementation
#[cfg(test)]
mod quaternion_test {
    use rand::Rng;
    use crate::{conjugate, from_angle_axis, from_rotation_matrix, inverse, Matrix3x3, Quaternion, quaternion_exp, rotate_x, rotate_y, rotate_z, to_angle_axis, to_rotation_matrix, Vector3};
    use crate::math::{abs, component_sum, normalize};

    #[test]
    fn simple_quaternion_test() {
        let quaternion_identity : Quaternion = Quaternion::IDENTITY;

        let quaternion_default : Quaternion = Quaternion::default();

        let set_quaternion : Quaternion = Quaternion::set(0.0, 0.0, 0.0, 1.0);

        let manual_quaternion : Quaternion = Quaternion {
            value: [0.0, 0.0, 0.0, 1.0].into(),
        };

        assert_eq!(quaternion_default, quaternion_identity);
        assert_eq!(quaternion_identity, set_quaternion);
        assert_eq!(set_quaternion, manual_quaternion);
    }

    #[test]
    fn quaternion_add_sub_test() {
        let quaternion_add_values_a : std::simd::f32x4 = [2.1, 4.12, 0.9512, 2.021].into();

        let quaternion_add_values_b : std::simd::f32x4 = [1.1123, 5.247, 0.2431, 8.721].into();

        let quaternion_add_a : Quaternion = Quaternion {
            value: quaternion_add_values_a,
        };

        let quaternion_addition_b : Quaternion = Quaternion {
            value: quaternion_add_values_b,
        };

        let mut quaternion_addition_sum : Quaternion = quaternion_add_a + quaternion_addition_b;


        assert_eq!(
            quaternion_addition_sum.value,
            quaternion_add_values_a + quaternion_add_values_b
        );

        quaternion_addition_sum += Quaternion::IDENTITY;

        assert_eq!(
            quaternion_addition_sum.value,
            quaternion_add_values_a + quaternion_add_values_b + Quaternion::IDENTITY.value
        );


        let mut quaternion_subtraction_sum : Quaternion = quaternion_add_a - quaternion_addition_b;

        assert_eq!(
            quaternion_subtraction_sum.value,
            quaternion_add_values_a - quaternion_add_values_b
        );

        quaternion_subtraction_sum -= Quaternion::IDENTITY;

        assert_eq!(
            quaternion_subtraction_sum.value,
            quaternion_add_values_a - quaternion_add_values_b - Quaternion::IDENTITY.value
        );
    }

    #[test]
    fn quaternion_mul_div_test() {
        const MULTIPLICATION_FP_ERROR_THRESHOLD: std::simd::f32x4 = std::simd::f32x4::from_array([0.05f32,0.05f32,0.0532, 0.05f32]);

        // look at the quaternion multiplication table for reference
        // i * i == 1 * i = 1,    j * 1 == 1 * j = 1,     k * 1 == 1 * k = 1
        // i * i = -1,  j * j = -1,   k * k = -1
        // i * j = k,  i * k = -j,
        // j * i = -k,  j * k = i,
        // k * i = j,  k * j = -i,

        // we will use hamilton product
        //(a1 + b1 i + c1 j + d1 k)(a2 + b2 i + c2 j + d2 k)

        //    (a1 * a2) + (a1 * b2)i + (a1 * c2)j + (a1 * d2)k
        //  + (b1 * a2)i + (b1 * b2)ii + (b1 * c2)ij + (b1 * d2)ik
        //  + (c1 * a2)j + (c1 * b2)ji + (c1 * c2)jj + (c1 * d2)jk
        //  + (d1 * a2)k + (d1 * b2)ki + (d1 * c2)kj + (d1 * d2)kk

        // since i * i == -1 , j * j == -1, k * k == -1
        // w = (a1 * a2) -(b1 * b2) - (c1 * c2) - (d1 * d2)

        // since jk = i and kj = -i
        // i = (a1 * b2) + (b1 * a2) + (c1 * d2) - (d1 * c2)

        // since ki = j and ik = -j
        // j = (a1 * c2) + (c1 * a2) + (d1 * b2) - (b1 * d2)

        // since ij = k and ji = -k
        // k = (d1 * a2) + (a1 * d2) + (b1 * c2) - (c1 * b2)

        // w, i, j, k
        // 45, 21, 0
        let quaternion_mul_values_a : [f32;4] = [0.3762754, 0.1683637, 0.0697385, 0.9084091];

        // 21, 3, 46
        let quaternion_mul_values_b : [f32; 4] = [0.1777481, -0.0474882, 0.3884478, 0.9029168];

        // 0 = 3 and shuffle rest
        let w : f32 = (quaternion_mul_values_a[3] * quaternion_mul_values_b[3])
            - (quaternion_mul_values_a[0] * quaternion_mul_values_b[0])
            - (quaternion_mul_values_a[1] * quaternion_mul_values_b[1])
            - (quaternion_mul_values_a[2] * quaternion_mul_values_b[2]);

        let i : f32 = quaternion_mul_values_a[3] * quaternion_mul_values_b[0]
            + quaternion_mul_values_a[0] * quaternion_mul_values_b[3]
            + quaternion_mul_values_a[2] * quaternion_mul_values_b[1]
            - quaternion_mul_values_a[1] * quaternion_mul_values_b[2];

        let j : f32 = (quaternion_mul_values_a[3] * quaternion_mul_values_b[1])
            + (quaternion_mul_values_a[1] * quaternion_mul_values_b[3])
            + (quaternion_mul_values_a[0] * quaternion_mul_values_b[2])
            - (quaternion_mul_values_a[2] * quaternion_mul_values_b[0]);

        let k : f32 = (quaternion_mul_values_a[2] * quaternion_mul_values_b[3])
            + (quaternion_mul_values_a[3] * quaternion_mul_values_b[2])
            + (quaternion_mul_values_a[1] * quaternion_mul_values_b[0])
            - (quaternion_mul_values_a[0] * quaternion_mul_values_b[1]);

        let quaternion_proven_mul : Quaternion = Quaternion::set(i, j, k, w);

        let quaternion_mul_a : Quaternion = Quaternion {
            value: quaternion_mul_values_a.into(),
        };

        let quaternion_mul_b : Quaternion = Quaternion {
            value: quaternion_mul_values_b.into(),
        };

        let quaternion_product = quaternion_mul_a * quaternion_mul_b;

        assert!(abs((quaternion_proven_mul - quaternion_product).value).lanes_lt(MULTIPLICATION_FP_ERROR_THRESHOLD).all());

        let quaternion_revert_a = quaternion_product / quaternion_mul_b;

        assert!(abs((quaternion_mul_a - quaternion_revert_a).value).lanes_lt(MULTIPLICATION_FP_ERROR_THRESHOLD).all());
    }

    #[test]
    fn quaternion_rotate_axis_test(){

        const FP_ERROR_THRESHOLD: std::simd::f32x4 =  std::simd::f32x4::from_array([0.0000003,0.0000003,0.0000003,0.0000003]);

        let angle_1: f32 = 31.11f32.to_radians();
        let angle_2: f32 = 241.12f32.to_radians();

        let rotate_x_result_1 : Quaternion = Quaternion::set(0.2681633, 0.0, 0.0, 0.9633735);

        let rotate_x_result_2 : Quaternion = Quaternion::set( 0.8610972, 0.0, 0.0, -0.5084404);

        let rotate_x_1 : Quaternion = rotate_x(angle_1);

        let rotate_x_2 : Quaternion  = rotate_x(angle_2);

        assert!(abs((rotate_x_1 - rotate_x_result_1).value).lanes_lt(FP_ERROR_THRESHOLD).all());

        assert!(abs((rotate_x_2 - rotate_x_result_2).value).lanes_lt(FP_ERROR_THRESHOLD).all());



        let rotation_y_result_1 : Quaternion = Quaternion::set(0.0, 0.2681633, 0.0, 0.9633735);

        let rotation_y_result_2 : Quaternion = Quaternion::set(0.0, 0.8610972, 0.0, -0.5084404);

        let rotate_y_1 : Quaternion = rotate_y(angle_1);

        let rotate_y_2 : Quaternion = rotate_y(angle_2);

        assert!(abs((rotate_y_1 - rotation_y_result_1).value).lanes_lt(FP_ERROR_THRESHOLD).all());

        assert!(abs((rotate_y_2 - rotation_y_result_2).value).lanes_lt(FP_ERROR_THRESHOLD).all());




        let rotation_z_result_1 : Quaternion = Quaternion::set(0.0, 0.0, 0.2681633, 0.9633735);

        let rotation_z_result_2 : Quaternion = Quaternion::set(0.0, 0.0, 0.8610972, -0.5084404);

        let rotate_z_1 : Quaternion = rotate_z(angle_1);

        let rotate_z_2 : Quaternion = rotate_z(angle_2);

        assert!(abs((rotate_z_1 - rotation_z_result_1).value).lanes_lt(FP_ERROR_THRESHOLD).all());

        assert!(abs((rotate_z_2 - rotation_z_result_2).value).lanes_lt(FP_ERROR_THRESHOLD).all());



        let (Vector3{ value : axis_x_1 }, angle_rad_x_1) = to_angle_axis(rotate_x_1);

        assert!(abs(axis_x_1 - Vector3::RIGHT.value).lanes_lt(FP_ERROR_THRESHOLD).all());

        assert_eq!(angle_rad_x_1, angle_1);

        let (Vector3{ value : axis_x_2 }, angle_rad_x_2) = to_angle_axis(rotate_x_2);

        assert!(abs(axis_x_2 - Vector3::RIGHT.value).lanes_lt(FP_ERROR_THRESHOLD).all());

        assert_eq!(angle_rad_x_2, angle_2);



        let (Vector3{ value : axis_y_1 }, angle_rad_y_1) = to_angle_axis(rotate_y_1);

        assert!(abs(axis_y_1 - Vector3::UP.value).lanes_lt(FP_ERROR_THRESHOLD).all());

        assert_eq!(angle_rad_y_1, angle_1);

        let (Vector3{ value  : axis_y_2}, angle_rad_y_2) = to_angle_axis(rotate_y_2);

        assert!(abs(axis_y_2 - Vector3::UP.value).lanes_lt(FP_ERROR_THRESHOLD).all());

        assert_eq!(angle_rad_y_2, angle_2);



        let (Vector3{ value : axis_z_1 }, angle_rad_z_1) = to_angle_axis(rotate_z_1);

        assert!(abs(axis_z_1 - Vector3::FORWARD.value).lanes_lt(FP_ERROR_THRESHOLD).all());

        assert_eq!(angle_rad_z_1, angle_1);

        let (Vector3{ value : axis_z_2}, angle_rad_z_2) = to_angle_axis(rotate_z_2);

        assert!(abs(axis_z_2 - Vector3::FORWARD.value).lanes_lt(FP_ERROR_THRESHOLD).all());

        assert_eq!(angle_rad_z_2, angle_2);

        let axis_norm = normalize([58.23, 12.124, 0.42, 0.0].into());

        let angle_axis_quaternion = from_angle_axis(Vector3{ value: axis_norm }, 24.24f32.to_radians());

        let (Vector3{ value }, angle) = to_angle_axis(angle_axis_quaternion);

        println!("{:?}", abs(value - axis_norm).lanes_lt(FP_ERROR_THRESHOLD));

        assert!(abs(value - axis_norm).lanes_lt(FP_ERROR_THRESHOLD).all());

        assert!((24.24f32.to_radians() - angle).abs() < component_sum(FP_ERROR_THRESHOLD) / 3.0f32);

    }
    #[test]
    fn quaternion_matrix_test(){
        const FP_ERROR_THRESHOLD_MATRIX: std::simd::f32x16 =  std::simd::f32x16::from_array([0.0000003;16]);
        const FP_ERROR_THRESHOLD : std::simd::f32x4 = std::simd::f32x4::from_array([0.0000003;4]);

        let identity_matrix: Matrix3x3 = Matrix3x3::IDENTITY;

        let identity_quaternion : Quaternion = from_rotation_matrix(identity_matrix);

        assert_eq!(identity_quaternion, Quaternion::IDENTITY);

        // euler angle (-34.2, 2.57, 121.11)
        let quaternion_rot_result : Quaternion = Quaternion::set(-0.1258448, 0.266531, 0.8288804, 0.4754803);

        let rotation_matrix_result : Matrix3x3 = Matrix3x3::set_from_rows(Vector3::set(-0.5161631, -0.8553157,  0.0448399), Vector3::set(0.7211497, -0.4057594,  0.5615180), Vector3::set(-0.4620810,  0.3221712,  0.8262487));

        let rnd_quat = to_rotation_matrix(quaternion_rot_result);

        assert!((rotation_matrix_result - rnd_quat).value.lanes_lt(FP_ERROR_THRESHOLD_MATRIX).all());

        let rotation_mat_to_quaternion : Quaternion = from_rotation_matrix(rnd_quat);

        assert!((rotation_mat_to_quaternion - quaternion_rot_result).value.lanes_lt(FP_ERROR_THRESHOLD).all());

        println!("{}", rotation_mat_to_quaternion - quaternion_rot_result);

    }


    #[test]
    fn quaternion_exp_test(){
        let quaternion_rot_result : Quaternion = Quaternion::set(-0.1258448, 0.266531, 0.8288804, 0.4754803);

        quaternion_exp(quaternion_rot_result);

    }


    #[test]
    fn quaternion_conj_inv_test(){

        const FP_ERROR_THRESHOLD : std::simd::f32x4 = std::simd::f32x4::from_array([0.000003;4]);

        let quaternion : Quaternion = Quaternion::set(0.1479202, -0.1477288, 0.7230414, 0.6584125);

        let conjugate_quaternion: Quaternion = conjugate(quaternion);

        let inverse_quaternion : Quaternion = inverse(quaternion);

        assert_eq!(abs(conjugate_quaternion.value - inverse_quaternion.value), FP_ERROR_THRESHOLD);


        println!("conjugate quaternion {}", conjugate_quaternion);

        println!("inverse quaternion {}", inverse_quaternion);

        assert_ne!(conjugate_quaternion, inverse_quaternion);

        let normalized_quaternion : Quaternion = Quaternion{ value: normalize(quaternion.value) };

        let norm_conjugate_quaternion : Quaternion = conjugate(normalized_quaternion);

        let norm_inverse_quaternion : Quaternion = inverse(normalized_quaternion);

        println!("normalized conjugate quaternion {}", norm_conjugate_quaternion);

        println!("normalized inverse quaternion {}", norm_inverse_quaternion);


    }

}
