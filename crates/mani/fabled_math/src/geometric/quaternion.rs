use crate::math_trait::QuaternionSwizzles;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign,  Mul, MulAssign, Sub, SubAssign};

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

    #[inline(always)]
    pub const fn set(i: f32, j: f32, k: f32, w: f32) -> Quaternion {
        Quaternion {
            value: std::simd::f32x4::from_array([i, j, k, w]),
        }
    }


    #[inline]
    pub const fn splat(val: f32) -> Quaternion {
        Quaternion {
            value: std::simd::f32x4::from_array([val; 4]),
        }
    }

    #[inline]
    pub const fn from_array(val: [f32; 4]) -> Quaternion {
        Quaternion {
            value: std::simd::f32x4::from_array(val),
        }
    }

    #[inline]
    pub fn additive_form(real_quaternion: Quaternion, pure_quaternion: Quaternion) -> Quaternion {
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

    #[inline]
    pub const fn to_primitive(self) -> [f32; 4] {
        self.value.to_array()
    }

    #[inline]
    pub const fn i(self) -> f32 {
        let array_vec3: [f32; 4] = self.value.to_array();

        array_vec3[0]
    }

    #[inline]
    pub const fn j(self) -> f32 {
        let array_vec3: [f32; 4] = self.value.to_array();

        array_vec3[1]
    }

    #[inline]
    pub const fn k(self) -> f32 {
        let array_vec3: [f32; 4] = self.value.to_array();

        array_vec3[2]
    }

    #[inline]
    pub const fn w(self) -> f32 {
        let array_vec3: [f32; 4] = self.value.to_array();

        array_vec3[3]
    }
}

impl Display for Quaternion {

    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            _f,
            "Quaternion (i : {}, j : {}, k : {}, w : {})",
            self.i(),
            self.j(),
            self.k(),
            self.w()
        )
    }
}

impl Add for Quaternion {
    type Output = Quaternion;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Quaternion {
            value: self.value + rhs.value,
        }
    }
}

impl AddAssign for Quaternion {

    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value
    }
}

impl Sub for Quaternion {
    type Output = Quaternion;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Quaternion {
            value: self.value - rhs.value,
        }
    }
}

impl SubAssign for Quaternion {

    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.value -= rhs.value;
    }
}


impl Mul for Quaternion {
    type Output = Quaternion;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        let [lhs_i, lhs_j, lhs_k, lhs_w] = self.value.to_array();
        let [rhs_i, rhs_j, rhs_k, rhs_w] = rhs.value.to_array();

        let w = (lhs_w * rhs_w) - (lhs_i * rhs_i) - (lhs_j * rhs_j) - (lhs_k * rhs_k);
        let i = (lhs_w * rhs_i) + (lhs_i * rhs_w) + (lhs_j * rhs_k) - (lhs_k * rhs_j);
        let j = (lhs_w * rhs_j) + (lhs_j * rhs_w) + (lhs_k * rhs_i) - (lhs_i * rhs_k);
        let k = (lhs_w * rhs_k) + (lhs_k * rhs_w) + (lhs_i * rhs_j) - (lhs_j * rhs_i);

        Quaternion {
            value: std::simd::f32x4::from_array([i, j, k, w]),
        }
    }
}

impl MulAssign for Quaternion {

    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

pub mod quaternion_math {
    use crate::math::{
        component_sum, cos, dot, length, length_squared, lerp, mul_add, normalize, rcp, select,
        sin, sqrt,
    };
    use crate::math_trait::QuaternionSwizzles;
    use crate::{Matrix3x3, Quaternion, Vector3, Vector4};
    use std::ops::Neg;
    use std::simd::SimdPartialOrd;

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
    pub fn from_angle_axis_mag(axis_mag: Vector3) -> Quaternion {
        let angle = length(axis_mag.value);

        let rcp_angle = rcp(Vector4::splat(angle).value);

        let axis = axis_mag.value * rcp_angle;

        from_angle_axis(Vector3 { value: axis }, angle)
    }


    #[inline]
    pub fn from_angle_axis(normalized_axis: Vector3, angle_radians: f32) -> Quaternion {
        let half_angle: Vector4 = Vector4::splat(angle_radians * 0.5f32);
        let quaternion_imaginary = sin(half_angle.value) * normalized_axis.value;

        let quaternion_real = cos(half_angle.value) * Vector4::set(0.0, 0.0, 0.0, 1.0).value;

        Quaternion {
            value: quaternion_imaginary + quaternion_real,
        }
    }

    #[inline]
    pub fn to_angle_axis(quaternion: Quaternion) -> (Vector3, f32) {
        let real: f32 = quaternion.value[3];

        let angle = 2.0 * real.acos();

        let scale = Vector4::splat(1.0 - real * real);

        let mask = scale.value.simd_lt(Vector4::splat(f32::EPSILON).value);

        let axis = select(
            Vector3::set(1.0, 0.0, 0.0).value,
            quaternion.value * rcp(sqrt(scale.value)),
            mask,
        );

        (Vector3 { value: axis } * Vector3::ONE, angle)
    }

    pub fn from_rotation_matrix(rotation_matrix: Matrix3x3) -> Quaternion {
        const HALF_VEC4: Vector4 = Vector4::from_array([0.5; 4]);

        let mut quaternion_array = std::simd::f32x4::from_array([0.0; 4]);

        let mut _s0 = 0.0;
        let mut _s1 = 0.0;
        let mut _s2 = 0.0;

        let mut _k0 = 0;
        let mut _k1 = 0;
        let mut _k2 = 0;
        let mut _k3 = 0;

        let diagonal_col_x = rotation_matrix.column_x.x();
        let diagonal_col_y = rotation_matrix.column_y.y();
        let diagonal_col_z = rotation_matrix.column_z.z();

        let sum_diagonal = component_sum(std::simd::f32x4::from_array([
            diagonal_col_x,
            diagonal_col_y,
            diagonal_col_z,
            0.0,
        ]));

        if sum_diagonal > 0.0 {
            _k0 = 3;
            _k1 = 2;
            _k2 = 1;
            _k3 = 0;

            _s0 = 1.0;
            _s1 = 1.0;
            _s2 = 1.0;
        } else if diagonal_col_x > diagonal_col_y && diagonal_col_x > diagonal_col_z {
            _k0 = 0;
            _k1 = 1;
            _k2 = 2;
            _k3 = 3;

            _s0 = 1.0;
            _s1 = -1.0;
            _s2 = -1.0;
        } else if diagonal_col_y > diagonal_col_z {
            _k0 = 1;
            _k1 = 0;
            _k2 = 3;
            _k3 = 2;

            _s0 = -1.0;
            _s1 = 1.0;
            _s2 = -1.0;
        } else {
            _k0 = 2;
            _k1 = 3;
            _k2 = 0;
            _k3 = 1;

            _s0 = -1.0;
            _s1 = -1.0;
            _s2 = 1.0;
        }

        let t = (_s0 * diagonal_col_x) + (_s1 * diagonal_col_y) + (_s2 * diagonal_col_z) + 1.0;

        let s2_yx = _s2 * rotation_matrix.column_y.x();
        let s1_xz = _s1 * rotation_matrix.column_x.z();
        let s0_zy = _s0 * rotation_matrix.column_z.y();

        quaternion_array[_k1] = rotation_matrix.column_x.y() - s2_yx;
        quaternion_array[_k2] = rotation_matrix.column_z.x() - s1_xz;
        quaternion_array[_k3] = rotation_matrix.column_y.z() - s0_zy;
        quaternion_array[_k0] = t;

        let s = Vector4 {
            value: rcp(sqrt(std::simd::f32x4::splat(t))),
        } * HALF_VEC4;

        Quaternion {
            value: quaternion_array * s.value,
        }
    }

    #[rustfmt::skip]
    pub fn to_rotation_matrix(quaternion: Quaternion) -> Matrix3x3 {
        let quaternion2 = quaternion + quaternion;
        
        let a = quaternion.jiij().value * quaternion2.jjkk().value;

        let b = quaternion.kwww().value * quaternion2.kkji().value;

        let ii2 = quaternion.i() * quaternion2.i();

        let interleave = a.interleave(b);

        let first = interleave.0.to_array();
        let second = interleave.1.to_array();

        let col_0 = Vector3::set(-first[1] - first[0] + 1.0,first[3] + first[2],  second[0] - second[1]);
        let col_1 = Vector3::set( first[2] - first[3], -ii2 - first[1] + 1.0, second[3] + second[2]);
        let col_2 = Vector3::set(second[1] + second[0],   second[2] - second[3], -ii2 - first[0] + 1.0);

        Matrix3x3::set_from_columns(col_0, col_1, col_2)
    }

    #[inline]
    pub fn conjugate(quaternion: Quaternion) -> Quaternion {
        Quaternion {
            value: quaternion.value.neg(),
        }
    }

    #[inline]
    pub fn inverse(quaternion: Quaternion) -> Quaternion {
        Quaternion {
            value: conjugate(quaternion).value
                * rcp(Vector4::splat(length_squared(quaternion.value)).value),
        }
    }


    #[inline]
    pub fn forward(quaternion: Quaternion) -> Vector3 {
        Vector3::FORWARD * quaternion
    }

    #[inline]
    pub fn right(quaternion: Quaternion) -> Vector3 {
        Vector3::RIGHT * quaternion
    }

    #[inline]
    pub fn up(quaternion: Quaternion) -> Vector3 {
        Vector3::UP * quaternion
    }

    #[inline]
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

    #[inline]
    pub fn slerp(
        start_quaternion: Quaternion,
        target_quaternion: Quaternion,
        delta: f32,
    ) -> Quaternion {
        let normalized_dot = dot(start_quaternion.value, target_quaternion.value);

        let angle = normalized_dot.acos();

        let rcp_sin_angle = angle.sin().recip();

        let delta_difference = 1.0 - delta;

        let delta_mul_angle = delta * angle;
        let delta_diff_mul_angle = delta_difference * angle;

        let weight_start: Vector4 = Vector4::splat(delta_diff_mul_angle.sin() * rcp_sin_angle);
        let weight_target: Vector4 = Vector4::splat(delta_mul_angle.sin() * rcp_sin_angle);

        let result_slerp = mul_add(
            start_quaternion.value,
            weight_start.value,
            target_quaternion.value * weight_target.value,
        );

        Quaternion {
            value: result_slerp,
        }
    }

    #[inline]
    pub fn real_quaternion(quaternion: Quaternion) -> Quaternion {
        Quaternion {
            value: quaternion.value * Vector4::set(0.0, 0.0, 0.0, 1.0).value,
        }
    }

    #[inline]
    pub fn pure_quaternion(quaternion: Quaternion) -> Quaternion {
        Quaternion {
            value: quaternion.value * Vector3::ONE.value,
        }
    }

    pub fn quaternion_log(_quaternion: Quaternion) -> Quaternion {
        todo!()
    }


    pub fn quaternion_exp(quaternion: Quaternion) -> Quaternion {
        let scalar: f32 = quaternion.value[3];
        let vector: Vector3 = Vector3 {
            value: quaternion.value * Vector3::ONE.value,
        };

        let quaternion_length: f32 = length(vector.value);

        let (sin_len, cos_len) = quaternion_length.sin_cos();

        let real_quaternion = vector * quaternion_length.recip() * sin_len;

        let pure_quaternion = Vector4::set(0.0, 0.0, 0.0, cos_len);

        let result = Vector4 {
            value: real_quaternion.value + pure_quaternion.value,
        } * scalar.exp();

        Quaternion {
            value: result.value,
        }
    }

    pub fn quaternion_pow(_quaternion: Quaternion, _factor: f32) -> Quaternion {
        todo!()
    }
}


impl QuaternionSwizzles for Quaternion {

    #[inline]
    fn iiii(self) -> Self {
        Quaternion::splat(self.i())
    }

    #[inline]
    fn iiij(self) -> Self {
        let iiij = std::simd::simd_swizzle!(self.value, [0, 0, 0, 1]);

        Quaternion { value: iiij }
    }

    #[inline]
    fn iiik(self) -> Self {
        let iiik = std::simd::simd_swizzle!(self.value, [0, 0, 0, 2]);

        Quaternion { value: iiik }
    }

    #[inline]
    fn iiiw(self) -> Self {
        let iiiw = std::simd::simd_swizzle!(self.value, [0, 0, 0, 3]);

        Quaternion { value: iiiw }
    }

    #[inline]
    fn iiji(self) -> Self {
        let iiji = std::simd::simd_swizzle!(self.value, [0, 0, 1, 0]);

        Quaternion { value: iiji }
    }

    #[inline]
    fn iijj(self) -> Self {
        let iijj = std::simd::simd_swizzle!(self.value, [0, 0, 1, 1]);

        Quaternion { value: iijj }
    }

    #[inline]
    fn iijk(self) -> Self {
        let iijk = std::simd::simd_swizzle!(self.value, [0, 0, 1, 2]);

        Quaternion { value: iijk }
    }

    #[inline]
    fn iijw(self) -> Self {
        let iijw = std::simd::simd_swizzle!(self.value, [0, 0, 1, 3]);

        Quaternion { value: iijw }
    }

    #[inline]
    fn iiki(self) -> Self {
        let iiki = std::simd::simd_swizzle!(self.value, [0, 0, 2, 0]);

        Quaternion { value: iiki }
    }

    #[inline]
    fn iikj(self) -> Self {
        let iikj = std::simd::simd_swizzle!(self.value, [0, 0, 2, 1]);

        Quaternion { value: iikj }
    }

    #[inline]
    fn iikk(self) -> Self {
        let iikk = std::simd::simd_swizzle!(self.value, [0, 0, 2, 2]);

        Quaternion { value: iikk }
    }

    #[inline]
    fn iikw(self) -> Self {
        let iikw = std::simd::simd_swizzle!(self.value, [0, 0, 2, 3]);

        Quaternion { value: iikw }
    }

    #[inline]
    fn iiwi(self) -> Self {
        let iiwi = std::simd::simd_swizzle!(self.value, [0, 0, 3, 0]);

        Quaternion { value: iiwi }
    }

    #[inline]
    fn iiwj(self) -> Self {
        let iiwj = std::simd::simd_swizzle!(self.value, [0, 0, 3, 1]);

        Quaternion { value: iiwj }
    }

    #[inline]
    fn iiwk(self) -> Self {
        let iiwk = std::simd::simd_swizzle!(self.value, [0, 0, 3, 2]);

        Quaternion { value: iiwk }
    }

    #[inline]
    fn iiww(self) -> Self {
        let iiww = std::simd::simd_swizzle!(self.value, [0, 0, 3, 3]);

        Quaternion { value: iiww }
    }

    #[inline]
    fn ijii(self) -> Self {
        let ijii = std::simd::simd_swizzle!(self.value, [0, 1, 0, 0]);

        Quaternion { value: ijii }
    }

    #[inline]
    fn ijij(self) -> Self {
        let ijij = std::simd::simd_swizzle!(self.value, [0, 1, 0, 1]);

        Quaternion { value: ijij }
    }

    #[inline]
    fn ijik(self) -> Self {
        let ijik = std::simd::simd_swizzle!(self.value, [0, 1, 0, 2]);

        Quaternion { value: ijik }
    }

    #[inline]
    fn ijiw(self) -> Self {
        let ijiw = std::simd::simd_swizzle!(self.value, [0, 1, 0, 3]);

        Quaternion { value: ijiw }
    }

    #[inline]
    fn ijji(self) -> Self {
        let ijji = std::simd::simd_swizzle!(self.value, [0, 1, 1, 0]);

        Quaternion { value: ijji }
    }

    #[inline]
    fn ijjj(self) -> Self {
        let ijjj = std::simd::simd_swizzle!(self.value, [0, 1, 1, 1]);

        Quaternion { value: ijjj }
    }

    #[inline]
    fn ijjk(self) -> Self {
        let ijjk = std::simd::simd_swizzle!(self.value, [0, 1, 1, 2]);

        Quaternion { value: ijjk }
    }

    #[inline]
    fn ijjw(self) -> Self {
        let ijjw = std::simd::simd_swizzle!(self.value, [0, 1, 1, 3]);

        Quaternion { value: ijjw }
    }

    #[inline]
    fn ijki(self) -> Self {
        let ijki = std::simd::simd_swizzle!(self.value, [0, 1, 2, 0]);

        Quaternion { value: ijki }
    }

    #[inline]
    fn ijkj(self) -> Self {
        let ijkj = std::simd::simd_swizzle!(self.value, [0, 1, 2, 1]);

        Quaternion { value: ijkj }
    }

    #[inline]
    fn ijkk(self) -> Self {
        let ijkk = std::simd::simd_swizzle!(self.value, [0, 1, 2, 2]);

        Quaternion { value: ijkk }
    }

    #[inline]
    fn ijwi(self) -> Self {
        let ijwi = std::simd::simd_swizzle!(self.value, [0, 1, 3, 0]);

        Quaternion { value: ijwi }
    }

    #[inline]
    fn ijwj(self) -> Self {
        let ijwj = std::simd::simd_swizzle!(self.value, [0, 1, 3, 1]);

        Quaternion { value: ijwj }
    }

    #[inline]
    fn ijwk(self) -> Self {
        let ijwk = std::simd::simd_swizzle!(self.value, [0, 1, 3, 2]);

        Quaternion { value: ijwk }
    }

    #[inline]
    fn ijww(self) -> Self {
        let ijww = std::simd::simd_swizzle!(self.value, [0, 1, 3, 3]);

        Quaternion { value: ijww }
    }

    #[inline]
    fn ikii(self) -> Self {
        let ikii = std::simd::simd_swizzle!(self.value, [0, 2, 0, 0]);

        Quaternion { value: ikii }
    }

    #[inline]
    fn ikij(self) -> Self {
        let ikij = std::simd::simd_swizzle!(self.value, [0, 2, 0, 1]);

        Quaternion { value: ikij }
    }

    #[inline]
    fn ikik(self) -> Self {
        let ikik = std::simd::simd_swizzle!(self.value, [0, 2, 0, 2]);

        Quaternion { value: ikik }
    }

    #[inline]
    fn ikiw(self) -> Self {
        let ikiw = std::simd::simd_swizzle!(self.value, [0, 2, 0, 3]);

        Quaternion { value: ikiw }
    }

    #[inline]
    fn ikji(self) -> Self {
        let ikji = std::simd::simd_swizzle!(self.value, [0, 2, 1, 0]);

        Quaternion { value: ikji }
    }

    #[inline]
    fn ikjj(self) -> Self {
        let ikjj = std::simd::simd_swizzle!(self.value, [0, 2, 1, 1]);

        Quaternion { value: ikjj }
    }

    #[inline]
    fn ikjk(self) -> Self {
        let ikjk = std::simd::simd_swizzle!(self.value, [0, 2, 1, 2]);

        Quaternion { value: ikjk }
    }

    #[inline]
    fn ikjw(self) -> Self {
        let ikjw = std::simd::simd_swizzle!(self.value, [0, 2, 1, 3]);

        Quaternion { value: ikjw }
    }

    #[inline]
    fn ikki(self) -> Self {
        let ikki = std::simd::simd_swizzle!(self.value, [0, 2, 2, 0]);

        Quaternion { value: ikki }
    }

    #[inline]
    fn ikkj(self) -> Self {
        let ikkj = std::simd::simd_swizzle!(self.value, [0, 2, 2, 1]);

        Quaternion { value: ikkj }
    }

    #[inline]
    fn ikkk(self) -> Self {
        let ikkk = std::simd::simd_swizzle!(self.value, [0, 2, 2, 2]);

        Quaternion { value: ikkk }
    }

    #[inline]
    fn ikkw(self) -> Self {
        let ikkw = std::simd::simd_swizzle!(self.value, [0, 2, 2, 3]);

        Quaternion { value: ikkw }
    }

    #[inline]
    fn ikwi(self) -> Self {
        let ikwi = std::simd::simd_swizzle!(self.value, [0, 2, 3, 0]);

        Quaternion { value: ikwi }
    }

    #[inline]
    fn ikwj(self) -> Self {
        let ikwj = std::simd::simd_swizzle!(self.value, [0, 2, 3, 1]);

        Quaternion { value: ikwj }
    }

    #[inline]
    fn ikwk(self) -> Self {
        let ikwk = std::simd::simd_swizzle!(self.value, [0, 2, 3, 2]);

        Quaternion { value: ikwk }
    }

    #[inline]
    fn ikww(self) -> Self {
        let ikww = std::simd::simd_swizzle!(self.value, [0, 2, 3, 3]);

        Quaternion { value: ikww }
    }

    #[inline]
    fn iwii(self) -> Self {
        let iwii = std::simd::simd_swizzle!(self.value, [0, 3, 0, 0]);

        Quaternion { value: iwii }
    }

    #[inline]
    fn iwij(self) -> Self {
        let iwij = std::simd::simd_swizzle!(self.value, [0, 3, 0, 1]);

        Quaternion { value: iwij }
    }

    #[inline]
    fn iwik(self) -> Self {
        let iwik = std::simd::simd_swizzle!(self.value, [0, 3, 0, 2]);

        Quaternion { value: iwik }
    }

    #[inline]
    fn iwiw(self) -> Self {
        let iwiw = std::simd::simd_swizzle!(self.value, [0, 3, 0, 3]);

        Quaternion { value: iwiw }
    }

    #[inline]
    fn iwji(self) -> Self {
        let iwji = std::simd::simd_swizzle!(self.value, [0, 3, 1, 0]);

        Quaternion { value: iwji }
    }

    #[inline]
    fn iwjj(self) -> Self {
        let iwjj = std::simd::simd_swizzle!(self.value, [0, 3, 1, 1]);

        Quaternion { value: iwjj }
    }

    #[inline]
    fn iwjk(self) -> Self {
        let iwjk = std::simd::simd_swizzle!(self.value, [0, 3, 1, 2]);

        Quaternion { value: iwjk }
    }

    #[inline]
    fn iwjw(self) -> Self {
        let iwjw = std::simd::simd_swizzle!(self.value, [0, 3, 1, 3]);

        Quaternion { value: iwjw }
    }

    #[inline]
    fn iwki(self) -> Self {
        let iwki = std::simd::simd_swizzle!(self.value, [0, 3, 2, 0]);

        Quaternion { value: iwki }
    }

    #[inline]
    fn iwkj(self) -> Self {
        let iwkj = std::simd::simd_swizzle!(self.value, [0, 3, 2, 1]);

        Quaternion { value: iwkj }
    }

    #[inline]
    fn iwkk(self) -> Self {
        let iwkk = std::simd::simd_swizzle!(self.value, [0, 3, 2, 2]);

        Quaternion { value: iwkk }
    }

    #[inline]
    fn iwkw(self) -> Self {
        let iwkw = std::simd::simd_swizzle!(self.value, [0, 3, 2, 3]);

        Quaternion { value: iwkw }
    }

    #[inline]
    fn iwwi(self) -> Self {
        let iwwi = std::simd::simd_swizzle!(self.value, [0, 3, 3, 0]);

        Quaternion { value: iwwi }
    }

    #[inline]
    fn iwwj(self) -> Self {
        let iwwj = std::simd::simd_swizzle!(self.value, [0, 3, 3, 1]);

        Quaternion { value: iwwj }
    }

    #[inline]
    fn iwwk(self) -> Self {
        let iwwk = std::simd::simd_swizzle!(self.value, [0, 3, 3, 2]);

        Quaternion { value: iwwk }
    }

    #[inline]
    fn iwww(self) -> Self {
        let iwww = std::simd::simd_swizzle!(self.value, [0, 3, 3, 3]);

        Quaternion { value: iwww }
    }

    #[inline]
    fn jiii(self) -> Self {
        let jiii = std::simd::simd_swizzle!(self.value, [1, 0, 0, 0]);

        Quaternion { value: jiii }
    }

    #[inline]
    fn jiij(self) -> Self {
        let jiij = std::simd::simd_swizzle!(self.value, [1, 0, 0, 1]);

        Quaternion { value: jiij }
    }

    #[inline]
    fn jiik(self) -> Self {
        let jiik = std::simd::simd_swizzle!(self.value, [1, 0, 0, 2]);

        Quaternion { value: jiik }
    }

    #[inline]
    fn jiiw(self) -> Self {
        let jiiw = std::simd::simd_swizzle!(self.value, [1, 0, 0, 3]);

        Quaternion { value: jiiw }
    }

    #[inline]
    fn jiji(self) -> Self {
        let jiji = std::simd::simd_swizzle!(self.value, [1, 0, 1, 0]);

        Quaternion { value: jiji }
    }

    #[inline]
    fn jijj(self) -> Self {
        let jijj = std::simd::simd_swizzle!(self.value, [1, 0, 1, 1]);

        Quaternion { value: jijj }
    }

    #[inline]
    fn jijk(self) -> Self {
        let jijk = std::simd::simd_swizzle!(self.value, [1, 0, 1, 2]);

        Quaternion { value: jijk }
    }

    #[inline]
    fn jijw(self) -> Self {
        let jijw = std::simd::simd_swizzle!(self.value, [1, 0, 1, 3]);

        Quaternion { value: jijw }
    }

    #[inline]
    fn jiki(self) -> Self {
        let jiki = std::simd::simd_swizzle!(self.value, [1, 0, 2, 0]);

        Quaternion { value: jiki }
    }

    #[inline]
    fn jikj(self) -> Self {
        let jikj = std::simd::simd_swizzle!(self.value, [1, 0, 2, 1]);

        Quaternion { value: jikj }
    }

    #[inline]
    fn jikk(self) -> Self {
        let jikk = std::simd::simd_swizzle!(self.value, [1, 0, 2, 2]);

        Quaternion { value: jikk }
    }

    #[inline]
    fn jikw(self) -> Self {
        let jikw = std::simd::simd_swizzle!(self.value, [1, 0, 2, 3]);

        Quaternion { value: jikw }
    }

    #[inline]
    fn jiwi(self) -> Self {
        let jiwi = std::simd::simd_swizzle!(self.value, [1, 0, 3, 0]);

        Quaternion { value: jiwi }
    }

    #[inline]
    fn jiwj(self) -> Self {
        let jiwj = std::simd::simd_swizzle!(self.value, [1, 0, 3, 1]);

        Quaternion { value: jiwj }
    }

    #[inline]
    fn jiwk(self) -> Self {
        let jiwk = std::simd::simd_swizzle!(self.value, [1, 0, 3, 2]);

        Quaternion { value: jiwk }
    }

    #[inline]
    fn jiww(self) -> Self {
        let jiww = std::simd::simd_swizzle!(self.value, [1, 0, 3, 3]);

        Quaternion { value: jiww }
    }

    #[inline]
    fn jjii(self) -> Self {
        let jjii = std::simd::simd_swizzle!(self.value, [1, 1, 0, 0]);

        Quaternion { value: jjii }
    }

    #[inline]
    fn jjij(self) -> Self {
        let jjij = std::simd::simd_swizzle!(self.value, [1, 1, 0, 1]);

        Quaternion { value: jjij }
    }

    #[inline]
    fn jjik(self) -> Self {
        let jjik = std::simd::simd_swizzle!(self.value, [1, 1, 0, 2]);

        Quaternion { value: jjik }
    }

    #[inline]
    fn jjiw(self) -> Self {
        let jjiw = std::simd::simd_swizzle!(self.value, [1, 1, 0, 3]);

        Quaternion { value: jjiw }
    }

    #[inline]
    fn jjji(self) -> Self {
        let jjji = std::simd::simd_swizzle!(self.value, [1, 1, 1, 0]);

        Quaternion { value: jjji }
    }

    #[inline]
    fn jjjj(self) -> Self {
        Quaternion::splat(self.j())
    }

    #[inline]
    fn jjjk(self) -> Self {
        let jjjk = std::simd::simd_swizzle!(self.value, [1, 1, 1, 2]);

        Quaternion { value: jjjk }
    }

    #[inline]
    fn jjjw(self) -> Self {
        let jjjw = std::simd::simd_swizzle!(self.value, [1, 1, 1, 3]);

        Quaternion { value: jjjw }
    }

    #[inline]
    fn jjki(self) -> Self {
        let jjki = std::simd::simd_swizzle!(self.value, [1, 1, 2, 0]);

        Quaternion { value: jjki }
    }

    #[inline]
    fn jjkj(self) -> Self {
        let jjkj = std::simd::simd_swizzle!(self.value, [1, 1, 2, 1]);

        Quaternion { value: jjkj }
    }

    #[inline]
    fn jjkk(self) -> Self {
        let jjkk = std::simd::simd_swizzle!(self.value, [1, 1, 2, 2]);

        Quaternion { value: jjkk }
    }

    #[inline]
    fn jjkw(self) -> Self {
        let jjkw = std::simd::simd_swizzle!(self.value, [1, 1, 2, 3]);

        Quaternion { value: jjkw }
    }

    #[inline]
    fn jjwi(self) -> Self {
        let jjwi = std::simd::simd_swizzle!(self.value, [1, 1, 3, 0]);

        Quaternion { value: jjwi }
    }

    #[inline]
    fn jjwj(self) -> Self {
        let jjwj = std::simd::simd_swizzle!(self.value, [1, 1, 3, 1]);

        Quaternion { value: jjwj }
    }

    #[inline]
    fn jjwk(self) -> Self {
        let jjwk = std::simd::simd_swizzle!(self.value, [1, 1, 3, 2]);

        Quaternion { value: jjwk }
    }

    #[inline]
    fn jjww(self) -> Self {
        let jjww = std::simd::simd_swizzle!(self.value, [1, 1, 3, 3]);

        Quaternion { value: jjww }
    }

    #[inline]
    fn jkii(self) -> Self {
        let jkii = std::simd::simd_swizzle!(self.value, [1, 2, 0, 0]);

        Quaternion { value: jkii }
    }

    #[inline]
    fn jkij(self) -> Self {
        let jkij = std::simd::simd_swizzle!(self.value, [1, 2, 0, 1]);

        Quaternion { value: jkij }
    }

    #[inline]
    fn jkik(self) -> Self {
        let jkik = std::simd::simd_swizzle!(self.value, [1, 2, 0, 2]);

        Quaternion { value: jkik }
    }

    #[inline]
    fn jkiw(self) -> Self {
        let jkiw = std::simd::simd_swizzle!(self.value, [1, 2, 0, 3]);

        Quaternion { value: jkiw }
    }

    #[inline]
    fn jkji(self) -> Self {
        let jkji = std::simd::simd_swizzle!(self.value, [1, 2, 1, 0]);

        Quaternion { value: jkji }
    }

    #[inline]
    fn jkjj(self) -> Self {
        let jkjj = std::simd::simd_swizzle!(self.value, [1, 2, 1, 1]);

        Quaternion { value: jkjj }
    }

    #[inline]
    fn jkjk(self) -> Self {
        let jkjk = std::simd::simd_swizzle!(self.value, [1, 2, 1, 2]);

        Quaternion { value: jkjk }
    }

    #[inline]
    fn jkjw(self) -> Self {
        let jkjw = std::simd::simd_swizzle!(self.value, [1, 2, 1, 3]);

        Quaternion { value: jkjw }
    }

    #[inline]
    fn jkki(self) -> Self {
        let jkki = std::simd::simd_swizzle!(self.value, [1, 2, 2, 0]);

        Quaternion { value: jkki }
    }

    #[inline]
    fn jkkj(self) -> Self {
        let jkkj = std::simd::simd_swizzle!(self.value, [1, 2, 2, 1]);

        Quaternion { value: jkkj }
    }

    #[inline]
    fn jkkk(self) -> Self {
        let jkkk = std::simd::simd_swizzle!(self.value, [1, 2, 2, 2]);

        Quaternion { value: jkkk }
    }

    #[inline]
    fn jkkw(self) -> Self {
        let jkkw = std::simd::simd_swizzle!(self.value, [1, 2, 2, 3]);

        Quaternion { value: jkkw }
    }

    #[inline]
    fn jkwi(self) -> Self {
        let jkwi = std::simd::simd_swizzle!(self.value, [1, 2, 3, 0]);

        Quaternion { value: jkwi }
    }

    #[inline]
    fn jkwj(self) -> Self {
        let jkwj = std::simd::simd_swizzle!(self.value, [1, 2, 3, 1]);

        Quaternion { value: jkwj }
    }

    #[inline]
    fn jkwk(self) -> Self {
        let jkwk = std::simd::simd_swizzle!(self.value, [1, 2, 3, 2]);

        Quaternion { value: jkwk }
    }

    #[inline]
    fn jkww(self) -> Self {
        let jkww = std::simd::simd_swizzle!(self.value, [1, 2, 3, 3]);

        Quaternion { value: jkww }
    }

    #[inline]
    fn jwii(self) -> Self {
        let jwii = std::simd::simd_swizzle!(self.value, [1, 3, 0, 0]);

        Quaternion { value: jwii }
    }

    #[inline]
    fn jwij(self) -> Self {
        let jwij = std::simd::simd_swizzle!(self.value, [1, 3, 0, 1]);

        Quaternion { value: jwij }
    }

    #[inline]
    fn jwik(self) -> Self {
        let jwik = std::simd::simd_swizzle!(self.value, [1, 3, 0, 2]);

        Quaternion { value: jwik }
    }

    #[inline]
    fn jwiw(self) -> Self {
        let jwiw = std::simd::simd_swizzle!(self.value, [1, 3, 0, 3]);

        Quaternion { value: jwiw }
    }

    #[inline]
    fn jwji(self) -> Self {
        let jwji = std::simd::simd_swizzle!(self.value, [1, 3, 1, 0]);

        Quaternion { value: jwji }
    }

    #[inline]
    fn jwjj(self) -> Self {
        let jwjj = std::simd::simd_swizzle!(self.value, [1, 3, 1, 1]);

        Quaternion { value: jwjj }
    }

    #[inline]
    fn jwjk(self) -> Self {
        let jwjk = std::simd::simd_swizzle!(self.value, [1, 3, 1, 2]);

        Quaternion { value: jwjk }
    }

    #[inline]
    fn jwjw(self) -> Self {
        let jwjw = std::simd::simd_swizzle!(self.value, [1, 3, 1, 3]);

        Quaternion { value: jwjw }
    }

    #[inline]
    fn jwki(self) -> Self {
        let jwki = std::simd::simd_swizzle!(self.value, [1, 3, 2, 0]);

        Quaternion { value: jwki }
    }

    #[inline]
    fn jwkj(self) -> Self {
        let jwkj = std::simd::simd_swizzle!(self.value, [1, 3, 2, 1]);

        Quaternion { value: jwkj }
    }

    #[inline]
    fn jwkk(self) -> Self {
        let jwkk = std::simd::simd_swizzle!(self.value, [1, 3, 2, 2]);

        Quaternion { value: jwkk }
    }

    #[inline]
    fn jwkw(self) -> Self {
        let jwkw = std::simd::simd_swizzle!(self.value, [1, 3, 2, 3]);

        Quaternion { value: jwkw }
    }

    #[inline]
    fn jwwi(self) -> Self {
        let jwwi = std::simd::simd_swizzle!(self.value, [1, 3, 3, 0]);

        Quaternion { value: jwwi }
    }

    #[inline]
    fn jwwj(self) -> Self {
        let jwwj = std::simd::simd_swizzle!(self.value, [1, 3, 3, 1]);

        Quaternion { value: jwwj }
    }

    #[inline]
    fn jwwk(self) -> Self {
        let jwwk = std::simd::simd_swizzle!(self.value, [1, 3, 3, 2]);

        Quaternion { value: jwwk }
    }

    #[inline]
    fn jwww(self) -> Self {
        let jwww = std::simd::simd_swizzle!(self.value, [1, 3, 3, 3]);

        Quaternion { value: jwww }
    }

    #[inline]
    fn kiii(self) -> Self {
        let kiii = std::simd::simd_swizzle!(self.value, [2, 0, 0, 0]);

        Quaternion { value: kiii }
    }

    #[inline]
    fn kiij(self) -> Self {
        let kiij = std::simd::simd_swizzle!(self.value, [2, 0, 0, 1]);

        Quaternion { value: kiij }
    }

    #[inline]
    fn kiik(self) -> Self {
        let kiik = std::simd::simd_swizzle!(self.value, [2,0,0,2]);

        Quaternion{ value: kiik }
    }

    #[inline]
    fn kiiw(self) -> Self {
        let kiiw = std::simd::simd_swizzle!(self.value, [2, 0, 0, 3]);

        Quaternion { value: kiiw }
    }

    #[inline]
    fn kiji(self) -> Self {
        let kiji = std::simd::simd_swizzle!(self.value, [2,0,1,0]);

        Quaternion { value: kiji }
    }

    #[inline]
    fn kijj(self) -> Self {
        let kijj = std::simd::simd_swizzle!(self.value, [2, 0, 1, 1]);

        Quaternion { value: kijj }
    }

    #[inline]
    fn kijk(self) -> Self {
        let kijk = std::simd::simd_swizzle!(self.value, [2,0,1,2]);

        Quaternion { value: kijk }
    }

    #[inline]
    fn kijw(self) -> Self {
        let kijw = std::simd::simd_swizzle!(self.value, [2,0,1,3]);

        Quaternion { value: kijw }
    }

    #[inline]
    fn kiki(self) -> Self {
        let kiki = std::simd::simd_swizzle!(self.value, [2,0,2,0]);

        Quaternion { value: kiki }
    }

    #[inline]
    fn kikj(self) -> Self {
        let kikj = std::simd::simd_swizzle!(self.value, [2,0,2,1]);

        Quaternion { value: kikj }
    }

    #[inline]
    fn kikk(self) -> Self {
        let kikk = std::simd::simd_swizzle!(self.value, [2, 0, 2, 2]);

        Quaternion { value: kikk }
    }

    #[inline]
    fn kikw(self) -> Self {
        let kikw = std::simd::simd_swizzle!(self.value, [2, 0, 2, 3]);

        Quaternion { value: kikw }
    }

    #[inline]
    fn kiwi(self) -> Self {
        let kiwi = std::simd::simd_swizzle!(self.value, [2, 0, 3, 0]);

        Quaternion { value: kiwi }
    }

    #[inline]
    fn kiwj(self) -> Self {
        let kiwj = std::simd::simd_swizzle!(self.value, [2, 0, 3, 1]);

        Quaternion { value: kiwj }
    }

    #[inline]
    fn kiwk(self) -> Self {
        let kiwk = std::simd::simd_swizzle!(self.value, [2, 0, 3, 2]);

        Quaternion { value: kiwk }
    }

    #[inline]
    fn kiww(self) -> Self {
        let kiww = std::simd::simd_swizzle!(self.value, [2, 0, 3, 3]);

        Quaternion { value: kiww }
    }

    #[inline]
    fn kjii(self) -> Self {
        let kjii = std::simd::simd_swizzle!(self.value, [2, 1, 0, 0]);

        Quaternion { value: kjii }
    }

    #[inline]
    fn kjij(self) -> Self {
        let kjij = std::simd::simd_swizzle!(self.value, [2, 1, 0, 1]);

        Quaternion { value: kjij }
    }

    #[inline]
    fn kjik(self) -> Self {
        let kjik = std::simd::simd_swizzle!(self.value, [2,1,0,2]);

        Quaternion { value: kjik }
    }

    #[inline]
    fn kjiw(self) -> Self {
        let kjiw = std::simd::simd_swizzle!(self.value, [2,1,0,3]);

        Quaternion { value: kjiw }
    }

    #[inline]
    fn kjji(self) -> Self {
        let kjji = std::simd::simd_swizzle!(self.value, [2,1,1,0]);

        Quaternion { value: kjji }
    }

    #[inline]
    fn kjjj(self) -> Self {
        let kjjj = std::simd::simd_swizzle!(self.value, [2,1,1,1]);

        Quaternion { value: kjjj }
    }

    #[inline]
    fn kjjk(self) -> Self {
        let kjjk = std::simd::simd_swizzle!(self.value, [2,1,1,2]);

        Quaternion { value: kjjk }
    }

    #[inline]
    fn kjjw(self) -> Self {
        let kjjw = std::simd::simd_swizzle!(self.value, [2,1,1,3]);

        Quaternion { value: kjjw }
    }

    #[inline]
    fn kjki(self) -> Self {
        let kjki = std::simd::simd_swizzle!(self.value, [2,1,2,0]);

        Quaternion { value: kjki }
    }

    #[inline]
    fn kjkj(self) -> Self {
        let kjkj = std::simd::simd_swizzle!(self.value, [2,1,2,1]);

        Quaternion { value: kjkj }
    }

    #[inline]
    fn kjkk(self) -> Self {
        let kjkk = std::simd::simd_swizzle!(self.value, [2, 1, 2, 2]);

        Quaternion { value: kjkk }
    }

    #[inline]
    fn kjkw(self) -> Self {
        let kjkw = std::simd::simd_swizzle!(self.value, [2, 1, 2, 3]);

        Quaternion { value: kjkw }
    }

    #[inline]
    fn kjwi(self) -> Self {
        let kjwi = std::simd::simd_swizzle!(self.value, [2, 1, 3, 0]);

        Quaternion { value: kjwi }
    }

    #[inline]
    fn kjwj(self) -> Self {
        let kjwj = std::simd::simd_swizzle!(self.value, [2,1,3,1]);

        Quaternion { value: kjwj }
    }

    #[inline]
    fn kjwk(self) -> Self {
        let kjwk = std::simd::simd_swizzle!(self.value, [2, 1, 3, 2]);

        Quaternion { value: kjwk }
    }

    #[inline]
    fn kjww(self) -> Self {
        let kjww = std::simd::simd_swizzle!(self.value, [2,1,3,3]);

        Quaternion { value: kjww }
    }

    #[inline]
    fn kkii(self) -> Self {
        let kkii = std::simd::simd_swizzle!(self.value, [2,2,0,0]);

        Quaternion { value: kkii }
    }

    #[inline]
    fn kkij(self) -> Self {
        let kkij = std::simd::simd_swizzle!(self.value, [2,2,0,1]);

        Quaternion { value: kkij }
    }

    #[inline]
    fn kkik(self) -> Self {
        let kkik = std::simd::simd_swizzle!(self.value, [2,2,0,2]);

        Quaternion { value: kkik }
    }

    #[inline]
    fn kkiw(self) -> Self {
        let kkiw = std::simd::simd_swizzle!(self.value, [2,2,0,3]);

        Quaternion { value: kkiw }
    }

    #[inline]
    fn kkji(self) -> Self {
        let kkji = std::simd::simd_swizzle!(self.value, [2, 2, 1, 0]);

        Quaternion { value: kkji }
    }

    #[inline]
    fn kkjj(self) -> Self {
        let kkjj = std::simd::simd_swizzle!(self.value, [2,2,1,1]);

        Quaternion { value: kkjj }
    }

    #[inline]
    fn kkjk(self) -> Self {
        let kkjk = std::simd::simd_swizzle!(self.value, [2,2,1,2]);

        Quaternion { value: kkjk }
    }

    #[inline]
    fn kkjw(self) -> Self {
        let kkjw = std::simd::simd_swizzle!(self.value, [2,2,1,3]);

        Quaternion { value: kkjw }
    }

    #[inline]
    fn kkki(self) -> Self {
        let kkki = std::simd::simd_swizzle!(self.value, [2,2,2,0]);

        Quaternion { value: kkki }
    }

    #[inline]
    fn kkkj(self) -> Self {
        let kkkj = std::simd::simd_swizzle!(self.value, [2,2,2,1]);

        Quaternion { value: kkkj }
    }

    #[inline]
    fn kkkk(self) -> Self {
        Quaternion::splat(self.k())
    }

    #[inline]
    fn kkkw(self) -> Self {
        let kkkw = std::simd::simd_swizzle!(self.value, [2,2,2,3]);

        Quaternion { value: kkkw }
    }

    #[inline]
    fn kkwi(self) -> Self {
        let kkwi = std::simd::simd_swizzle!(self.value, [2,2,3,0]);

        Quaternion { value: kkwi }
    }

    #[inline]
    fn kkwj(self) -> Self {
        let kkwj = std::simd::simd_swizzle!(self.value, [2,2,3,1]);

        Quaternion { value: kkwj }
    }

    #[inline]
    fn kkwk(self) -> Self {
        let kkwk = std::simd::simd_swizzle!(self.value, [2,2,3,2]);

        Quaternion { value: kkwk }
    }

    #[inline]
    fn kkww(self) -> Self {
        let kkww = std::simd::simd_swizzle!(self.value, [2,2,3,3]);

        Quaternion { value: kkww }
    }

    #[inline]
    fn kwii(self) -> Self {
        let kwii = std::simd::simd_swizzle!(self.value, [2,3,0,0]);

        Quaternion { value: kwii }
    }

    #[inline]
    fn kwij(self) -> Self {
        let kwij = std::simd::simd_swizzle!(self.value, [2,3,0,1]);

        Quaternion { value: kwij }
    }

    #[inline]
    fn kwik(self) -> Self {
        let kwik = std::simd::simd_swizzle!(self.value, [2,3,0,2]);

        Quaternion { value: kwik }
    }

    #[inline]
    fn kwiw(self) -> Self {
        let kwiw = std::simd::simd_swizzle!(self.value, [2,3,0,3]);

        Quaternion { value: kwiw }
    }

    #[inline]
    fn kwji(self) -> Self {
        let kwji = std::simd::simd_swizzle!(self.value, [2,3,1,0]);

        Quaternion { value: kwji }
    }

    #[inline]
    fn kwjj(self) -> Self {
        let kwjj = std::simd::simd_swizzle!(self.value, [2,3,1,1]);

        Quaternion { value: kwjj }
    }

    #[inline]
    fn kwjk(self) -> Self {
        let kwjk = std::simd::simd_swizzle!(self.value, [2,3,1,2]);

        Quaternion { value: kwjk }
    }

    #[inline]
    fn kwjw(self) -> Self {
        let kwjw = std::simd::simd_swizzle!(self.value, [2,3,1,3]);

        Quaternion { value: kwjw }
    }

    #[inline]
    fn kwki(self) -> Self {
        let kwki = std::simd::simd_swizzle!(self.value, [2,3,2,0]);

        Quaternion { value: kwki }
    }

    #[inline]
    fn kwkj(self) -> Self {
        let kwkj = std::simd::simd_swizzle!(self.value, [2,3,2,1]);

        Quaternion { value: kwkj }
    }

    #[inline]
    fn kwkk(self) -> Self {
        let kwkk = std::simd::simd_swizzle!(self.value, [2,3,2,2]);

        Quaternion { value: kwkk }
    }

    #[inline]
    fn kwkw(self) -> Self {
        let kwkw = std::simd::simd_swizzle!(self.value, [2,3,2,3]);

        Quaternion { value: kwkw }
    }

    #[inline]
    fn kwwi(self) -> Self {
        let kwwi = std::simd::simd_swizzle!(self.value, [2,3,3,0]);

        Quaternion { value: kwwi }
    }

    #[inline]
    fn kwwj(self) -> Self {
        let kwwj = std::simd::simd_swizzle!(self.value, [2,3,3,1]);

        Quaternion { value: kwwj }
    }

    #[inline]
    fn kwwk(self) -> Self {
        let kwwk = std::simd::simd_swizzle!(self.value, [2,3,3,2]);

        Quaternion { value: kwwk }
    }

    #[inline]
    fn kwww(self) -> Self {
        let kwww = std::simd::simd_swizzle!(self.value, [2, 3, 3, 3]);

        Quaternion { value: kwww }
    }

    #[inline]
    fn wiii(self) -> Self {
        let wiii = std::simd::simd_swizzle!(self.value, [3,0,0,0]);

        Quaternion { value: wiii }
    }

    #[inline]
    fn wiij(self) -> Self {
        let wiij = std::simd::simd_swizzle!(self.value, [3,0,0,1]);

        Quaternion { value: wiij }
    }

    #[inline]
    fn wiik(self) -> Self {
        let wiik = std::simd::simd_swizzle!(self.value, [3,0,0,2]);

        Quaternion { value: wiik }
    }

    #[inline]
    fn wiiw(self) -> Self {
        let wiiw = std::simd::simd_swizzle!(self.value, [3,0,0,3]);

        Quaternion { value: wiiw }
    }

    #[inline]
    fn wiji(self) -> Self {
        let wiji = std::simd::simd_swizzle!(self.value, [3,0,1,0]);

        Quaternion { value: wiji }
    }

    #[inline]
    fn wijj(self) -> Self {
        let wijj = std::simd::simd_swizzle!(self.value, [3,0,1,1]);

        Quaternion { value: wijj }
    }

    #[inline]
    fn wijk(self) -> Self {
        let wijk = std::simd::simd_swizzle!(self.value, [3,0,1,2]);

        Quaternion { value: wijk }
    }

    #[inline]
    fn wijw(self) -> Self {
        let wijw = std::simd::simd_swizzle!(self.value, [3,0,1,3]);

        Quaternion { value: wijw }
    }

    #[inline]
    fn wiki(self) -> Self {
        let wiki = std::simd::simd_swizzle!(self.value, [3,0,2,0]);

        Quaternion { value: wiki }
    }

    #[inline]
    fn wikj(self) -> Self {
        let wikj = std::simd::simd_swizzle!(self.value, [3,0,2,1]);

        Quaternion { value: wikj }
    }

    #[inline]
    fn wikk(self) -> Self {
         let wikk = std::simd::simd_swizzle!(self.value, [3,0,2,2]);

         Quaternion { value: wikk }
    }

    #[inline]
    fn wikw(self) -> Self {
        let wikw = std::simd::simd_swizzle!(self.value, [3,0,2,3]);

        Quaternion { value: wikw }
    }

    #[inline]
    fn wiwi(self) -> Self {
        let wiwi = std::simd::simd_swizzle!(self.value, [3,0,3,0]);

        Quaternion { value: wiwi }
    }

    #[inline]
    fn wiwj(self) -> Self {
        let wiwj = std::simd::simd_swizzle!(self.value, [3,0,3,1]);

        Quaternion { value: wiwj }
    }

    #[inline]
    fn wiwk(self) -> Self {
        let wiwk = std::simd::simd_swizzle!(self.value, [3,0,3,2]);

        Quaternion { value: wiwk }
    }

    #[inline]
    fn wiww(self) -> Self {
        let wiww = std::simd::simd_swizzle!(self.value, [3,0,3,3]);

        Quaternion { value: wiww }
    }

    #[inline]
    fn wjii(self) -> Self {
        let wjii = std::simd::simd_swizzle!(self.value, [3,1,0,0]);

        Quaternion { value: wjii }
    }

    #[inline]
    fn wjij(self) -> Self {
        let wjij = std::simd::simd_swizzle!(self.value, [3,1,0,1]);

        Quaternion { value: wjij }
    }

    #[inline]
    fn wjik(self) -> Self {
        let wjik = std::simd::simd_swizzle!(self.value, [3,1,0,2]);

        Quaternion { value: wjik }
    }

    #[inline]
    fn wjiw(self) -> Self {
        let wjiw = std::simd::simd_swizzle!(self.value, [3,1,0,3]);

        Quaternion { value: wjiw }
    }

    #[inline]
    fn wjji(self) -> Self {
        let wjji = std::simd::simd_swizzle!(self.value, [3,1,1,0]);

        Quaternion { value: wjji }
    }

    #[inline]
    fn wjjj(self) -> Self {
        let wjjj = std::simd::simd_swizzle!(self.value, [3,1,1,1]);

        Quaternion { value: wjjj }
    }

    #[inline]
    fn wjjk(self) -> Self {
        let wjjk = std::simd::simd_swizzle!(self.value, [3,1,1,2]);

        Quaternion { value: wjjk }
    }

    #[inline]
    fn wjjw(self) -> Self {
        let wjjw = std::simd::simd_swizzle!(self.value, [3,1,1,3]);

        Quaternion { value: wjjw }
    }

    #[inline]
    fn wjki(self) -> Self {
        let wjki = std::simd::simd_swizzle!(self.value, [3,1,2,0]);

        Quaternion { value: wjki }
    }

    #[inline]
    fn wjkj(self) -> Self {
        let wjkj = std::simd::simd_swizzle!(self.value, [3,1,2,1]);

        Quaternion { value: wjkj }
    }

    #[inline]
    fn wjkk(self) -> Self {
        let wjkk = std::simd::simd_swizzle!(self.value, [3,1,2,2]);

        Quaternion { value: wjkk }
    }

    #[inline]
    fn wjkw(self) -> Self {
        let wjkw = std::simd::simd_swizzle!(self.value, [3,1,2,3]);

        Quaternion { value: wjkw }
    }

    #[inline]
    fn wjwi(self) -> Self {
        let wjwi = std::simd::simd_swizzle!(self.value, [3,1,3,0]);

        Quaternion { value: wjwi }
    }

    #[inline]
    fn wjwj(self) -> Self {
        let wjwj = std::simd::simd_swizzle!(self.value, [3,1,3,1]);

        Quaternion { value: wjwj }
    }

    #[inline]
    fn wjwk(self) -> Self {
        let wjwk = std::simd::simd_swizzle!(self.value, [3,1,3,2]);

        Quaternion { value: wjwk }
    }

    #[inline]
    fn wjww(self) -> Self {
        let wjww = std::simd::simd_swizzle!(self.value, [3,1,3,3]);

        Quaternion { value: wjww }
    }

    #[inline]
    fn wkii(self) -> Self {
        let wkii = std::simd::simd_swizzle!(self.value, [3,2,0,0]);

        Quaternion { value: wkii }
    }

    #[inline]
    fn wkij(self) -> Self {
        let wkij = std::simd::simd_swizzle!(self.value, [3,2,0,1]);

        Quaternion { value: wkij }
    }

    #[inline]
    fn wkik(self) -> Self {
        let wkik = std::simd::simd_swizzle!(self.value, [3,2,0,2]);

        Quaternion { value: wkik }
    }

    #[inline]
    fn wkiw(self) -> Self {
        let wkiw = std::simd::simd_swizzle!(self.value, [3,2,0,3]);

        Quaternion { value: wkiw }
    }

    #[inline]
    fn wkji(self) -> Self {
        let wkji = std::simd::simd_swizzle!(self.value, [3,2,1,0]);

        Quaternion { value: wkji }
    }

    #[inline]
    fn wkjj(self) -> Self {
        let wkjj = std::simd::simd_swizzle!(self.value, [3,2,1,1]);

        Quaternion { value: wkjj }
    }

    #[inline]
    fn wkjk(self) -> Self {
        let wkjk = std::simd::simd_swizzle!(self.value, [3,2,1,2]);

        Quaternion { value: wkjk }
    }

    #[inline]
    fn wkjw(self) -> Self {
        let wkjw = std::simd::simd_swizzle!(self.value, [3,2,1,3]);

        Quaternion { value: wkjw }
    }

    #[inline]
    fn wkki(self) -> Self {
        let wkki = std::simd::simd_swizzle!(self.value, [3,2,2,0]);

        Quaternion { value: wkki }
    }

    #[inline]
    fn wkkj(self) -> Self {
        let wkkj = std::simd::simd_swizzle!(self.value, [3,2,2,1]);

        Quaternion { value: wkkj }
    }

    #[inline]
    fn wkkk(self) -> Self {
        let wkkk = std::simd::simd_swizzle!(self.value, [3,2,2,2]);

        Quaternion { value: wkkk }
    }

    #[inline]
    fn wkkw(self) -> Self {
        let wkkw = std::simd::simd_swizzle!(self.value, [3,2,2,3]);

        Quaternion { value: wkkw }
    }

    #[inline]
    fn wkwi(self) -> Self {
        let wkwi = std::simd::simd_swizzle!(self.value, [3,2,3,0]);

        Quaternion { value: wkwi }
    }

    #[inline]
    fn wkwj(self) -> Self {
        let wkwj = std::simd::simd_swizzle!(self.value, [3,2,3,1]);

        Quaternion { value: wkwj }
    }

    #[inline]
    fn wkwk(self) -> Self {
        let wkwk = std::simd::simd_swizzle!(self.value, [3,2,3,2]);

        Quaternion { value: wkwk }
    }

    #[inline]
    fn wkww(self) -> Self {
        let wkww = std::simd::simd_swizzle!(self.value, [3,2,3,3]);

        Quaternion { value: wkww }
    }

    #[inline]
    fn wwii(self) -> Self {
        let wwii = std::simd::simd_swizzle!(self.value, [3,3,0,0]);

        Quaternion { value: wwii }
    }

    #[inline]
    fn wwij(self) -> Self {
        let wwij = std::simd::simd_swizzle!(self.value, [3,3,0,1]);

        Quaternion { value: wwij }
    }

    #[inline]
    fn wwik(self) -> Self {
        let wwik = std::simd::simd_swizzle!(self.value, [3,3,0,2]);

        Quaternion { value: wwik }
    }

    #[inline]
    fn wwiw(self) -> Self {
        let wwiw = std::simd::simd_swizzle!(self.value, [3,3,0,3]);

        Quaternion { value: wwiw }
    }

    #[inline]
    fn wwji(self) -> Self {
        let wwji = std::simd::simd_swizzle!(self.value, [3,3,1,0]);

        Quaternion { value: wwji }
    }

    #[inline]
    fn wwjj(self) -> Self {
        let wwjj = std::simd::simd_swizzle!(self.value, [3,3,1,1]);

        Quaternion { value: wwjj }
    }

    #[inline]
    fn wwjk(self) -> Self {
        let wwjk = std::simd::simd_swizzle!(self.value, [3,3,1,2]);

        Quaternion { value: wwjk }
    }

    #[inline]
    fn wwjw(self) -> Self {
        let wwjw = std::simd::simd_swizzle!(self.value, [3,3,1,3]);

        Quaternion { value: wwjw }
    }

    #[inline]
    fn wwki(self) -> Self {
        let wwki = std::simd::simd_swizzle!(self.value, [3,3,2,0]);

        Quaternion { value: wwki }
    }

    #[inline]
    fn wwkj(self) -> Self {
        let wwkj = std::simd::simd_swizzle!(self.value, [3,3,2,1]);

        Quaternion { value: wwkj }
    }

    #[inline]
    fn wwkk(self) -> Self {
        let wwkk = std::simd::simd_swizzle!(self.value, [3,3,2,2]);

        Quaternion { value: wwkk }
    }

    #[inline]
    fn wwkw(self) -> Self {
        let wwkw = std::simd::simd_swizzle!(self.value, [3,3,2,3]);

        Quaternion { value: wwkw }
    }

    #[inline]
    fn wwwi(self) -> Self {
        let wwwi = std::simd::simd_swizzle!(self.value, [3,3,3,0]);

        Quaternion { value: wwwi }
    }

    #[inline]
    fn wwwj(self) -> Self {
        let wwwj = std::simd::simd_swizzle!(self.value, [3,3,3,1]);

        Quaternion { value: wwwj }
    }

    #[inline]
    fn wwwk(self) -> Self {
        let wwwk = std::simd::simd_swizzle!(self.value, [3,3,3,2]);

        Quaternion { value: wwwk }
    }

    #[inline]
    fn wwww(self) -> Self {
        Quaternion::splat(self.w())
    }
}

// todo Write test for all quaternion and quaternion math implementation
#[cfg(test)]
mod quaternion_test {
    use crate::math::{abs, component_sum, normalize};

    use crate::quaternion_math::{
        conjugate, from_angle_axis, from_rotation_matrix, inverse, quaternion_exp, rotate_x,
        rotate_y, rotate_z, to_angle_axis, to_rotation_matrix,
    };
    use crate::{Matrix3x3, Quaternion, Vector3};
    use std::simd::SimdPartialOrd;

    #[test]
    fn simple_quaternion_test() {
        let quaternion_identity: Quaternion = Quaternion::IDENTITY;

        let quaternion_default: Quaternion = Quaternion::default();

        let set_quaternion: Quaternion = Quaternion::set(0.0, 0.0, 0.0, 1.0);

        let manual_quaternion: Quaternion = Quaternion {
            value: [0.0, 0.0, 0.0, 1.0].into(),
        };

        assert_eq!(quaternion_default, quaternion_identity);
        assert_eq!(quaternion_identity, set_quaternion);
        assert_eq!(set_quaternion, manual_quaternion);
    }

    #[test]
    fn quaternion_add_sub_test() {
        let quaternion_add_values_a: std::simd::f32x4 = [2.1, 4.12, 0.9512, 2.021].into();

        let quaternion_add_values_b: std::simd::f32x4 = [1.1123, 5.247, 0.2431, 8.721].into();

        let quaternion_add_a: Quaternion = Quaternion {
            value: quaternion_add_values_a,
        };

        let quaternion_addition_b: Quaternion = Quaternion {
            value: quaternion_add_values_b,
        };

        let mut quaternion_addition_sum: Quaternion = quaternion_add_a + quaternion_addition_b;


        assert_eq!(
            quaternion_addition_sum.value,
            quaternion_add_values_a + quaternion_add_values_b
        );

        quaternion_addition_sum += Quaternion::IDENTITY;

        assert_eq!(
            quaternion_addition_sum.value,
            quaternion_add_values_a + quaternion_add_values_b + Quaternion::IDENTITY.value
        );


        let mut quaternion_subtraction_sum: Quaternion = quaternion_add_a - quaternion_addition_b;

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
        const MULTIPLICATION_FP_ERROR_THRESHOLD: std::simd::f32x4 =
            std::simd::f32x4::from_array([0.05f32, 0.05f32, 0.0532, 0.05f32]);

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
        let quaternion_mul_values_a: [f32; 4] = [0.3762754, 0.1683637, 0.0697385, 0.9084091];

        // 21, 3, 46
        let quaternion_mul_values_b: [f32; 4] = [0.1777481, -0.0474882, 0.3884478, 0.9029168];


        let w: f32 = (quaternion_mul_values_a[3] * quaternion_mul_values_b[3])
                     - (quaternion_mul_values_a[0] * quaternion_mul_values_b[0])
                     - (quaternion_mul_values_a[1] * quaternion_mul_values_b[1])
                     - (quaternion_mul_values_a[2] * quaternion_mul_values_b[2]);

        let i: f32 = quaternion_mul_values_a[3] * quaternion_mul_values_b[0]
                     + quaternion_mul_values_a[0] * quaternion_mul_values_b[3]
                     + quaternion_mul_values_a[1] * quaternion_mul_values_b[2]
                     - quaternion_mul_values_a[2] * quaternion_mul_values_b[1];

        let j: f32 = (quaternion_mul_values_a[3] * quaternion_mul_values_b[1])
                     + (quaternion_mul_values_a[1] * quaternion_mul_values_b[3])
                     + (quaternion_mul_values_a[2] * quaternion_mul_values_b[0])
                     - (quaternion_mul_values_a[0] * quaternion_mul_values_b[2]);

        let k: f32 = (quaternion_mul_values_a[3] * quaternion_mul_values_b[2])
                     + (quaternion_mul_values_a[2] * quaternion_mul_values_b[3])
                     + (quaternion_mul_values_a[0] * quaternion_mul_values_b[1])
                     - (quaternion_mul_values_a[1] * quaternion_mul_values_b[0]);

        let quaternion_proven_mul: Quaternion = Quaternion::set(i, j, k, w);

        let quaternion_mul_a: Quaternion = Quaternion {
            value: quaternion_mul_values_a.into(),
        };

        let quaternion_mul_b: Quaternion = Quaternion {
            value: quaternion_mul_values_b.into(),
        };

        let quaternion_product = quaternion_mul_a * quaternion_mul_b;

        assert!(abs((quaternion_proven_mul - quaternion_product).value)
            .simd_lt(MULTIPLICATION_FP_ERROR_THRESHOLD)
            .all());
    }

    #[test]
    fn quaternion_rotate_axis_test() {
        const FP_ERROR_THRESHOLD: std::simd::f32x4 =
            std::simd::f32x4::from_array([0.0000003, 0.0000003, 0.0000003, 0.0000003]);

        let angle_1: f32 = 31.11f32.to_radians();
        let angle_2: f32 = 241.12f32.to_radians();

        let rotate_x_result_1: Quaternion = Quaternion::set(0.2681633, 0.0, 0.0, 0.9633735);

        let rotate_x_result_2: Quaternion = Quaternion::set(0.8610972, 0.0, 0.0, -0.5084404);

        let rotate_x_1: Quaternion = rotate_x(angle_1);

        let rotate_x_2: Quaternion = rotate_x(angle_2);

        assert!(abs((rotate_x_1 - rotate_x_result_1).value)
            .simd_lt(FP_ERROR_THRESHOLD)
            .all());

        assert!(abs((rotate_x_2 - rotate_x_result_2).value)
            .simd_lt(FP_ERROR_THRESHOLD)
            .all());


        let rotation_y_result_1: Quaternion = Quaternion::set(0.0, 0.2681633, 0.0, 0.9633735);

        let rotation_y_result_2: Quaternion = Quaternion::set(0.0, 0.8610972, 0.0, -0.5084404);

        let rotate_y_1: Quaternion = rotate_y(angle_1);

        let rotate_y_2: Quaternion = rotate_y(angle_2);

        assert!(abs((rotate_y_1 - rotation_y_result_1).value)
            .simd_lt(FP_ERROR_THRESHOLD)
            .all());

        assert!(abs((rotate_y_2 - rotation_y_result_2).value)
            .simd_lt(FP_ERROR_THRESHOLD)
            .all());


        let rotation_z_result_1: Quaternion = Quaternion::set(0.0, 0.0, 0.2681633, 0.9633735);

        let rotation_z_result_2: Quaternion = Quaternion::set(0.0, 0.0, 0.8610972, -0.5084404);

        let rotate_z_1: Quaternion = rotate_z(angle_1);

        let rotate_z_2: Quaternion = rotate_z(angle_2);

        assert!(abs((rotate_z_1 - rotation_z_result_1).value)
            .simd_lt(FP_ERROR_THRESHOLD)
            .all());

        assert!(abs((rotate_z_2 - rotation_z_result_2).value)
            .simd_lt(FP_ERROR_THRESHOLD)
            .all());


        let (Vector3 { value: axis_x_1 }, angle_rad_x_1) = to_angle_axis(rotate_x_1);

        assert!(abs(axis_x_1 - Vector3::RIGHT.value)
            .simd_lt(FP_ERROR_THRESHOLD)
            .all());

        assert_eq!(angle_rad_x_1, angle_1);

        let (Vector3 { value: axis_x_2 }, angle_rad_x_2) = to_angle_axis(rotate_x_2);

        assert!(abs(axis_x_2 - Vector3::RIGHT.value)
            .simd_lt(FP_ERROR_THRESHOLD)
            .all());

        assert_eq!(angle_rad_x_2, angle_2);


        let (Vector3 { value: axis_y_1 }, angle_rad_y_1) = to_angle_axis(rotate_y_1);

        assert!(abs(axis_y_1 - Vector3::UP.value)
            .simd_lt(FP_ERROR_THRESHOLD)
            .all());

        assert_eq!(angle_rad_y_1, angle_1);

        let (Vector3 { value: axis_y_2 }, angle_rad_y_2) = to_angle_axis(rotate_y_2);

        assert!(abs(axis_y_2 - Vector3::UP.value)
            .simd_lt(FP_ERROR_THRESHOLD)
            .all());

        assert_eq!(angle_rad_y_2, angle_2);


        let (Vector3 { value: axis_z_1 }, angle_rad_z_1) = to_angle_axis(rotate_z_1);

        assert!(abs(axis_z_1 - Vector3::FORWARD.value)
            .simd_lt(FP_ERROR_THRESHOLD)
            .all());

        assert_eq!(angle_rad_z_1, angle_1);

        let (Vector3 { value: axis_z_2 }, angle_rad_z_2) = to_angle_axis(rotate_z_2);

        assert!(abs(axis_z_2 - Vector3::FORWARD.value)
            .simd_lt(FP_ERROR_THRESHOLD)
            .all());

        assert_eq!(angle_rad_z_2, angle_2);

        let axis_norm = normalize([58.23, 12.124, 0.42, 0.0].into());

        let angle_axis_quaternion =
            from_angle_axis(Vector3 { value: axis_norm }, 24.24f32.to_radians());

        let (Vector3 { value }, angle) = to_angle_axis(angle_axis_quaternion);

        assert!(abs(value - axis_norm).simd_lt(FP_ERROR_THRESHOLD).all());

        assert!((24.24f32.to_radians() - angle).abs() < component_sum(FP_ERROR_THRESHOLD) / 3.0f32);
    }

    #[test]
    fn quaternion_matrix_test() {
        const FP_ERROR_THRESHOLD_MATRIX: Matrix3x3 = Matrix3x3::splat(0.0000003);

        const FP_ERROR_THRESHOLD: std::simd::f32x4 = std::simd::f32x4::from_array([0.0000003; 4]);

        let identity_matrix: Matrix3x3 = Matrix3x3::IDENTITY;

        let identity_quaternion: Quaternion = from_rotation_matrix(identity_matrix);

        assert_eq!(identity_quaternion, Quaternion::IDENTITY);

        // euler angle (-34.2, 2.57, 121.11)
        let quaternion_rot_result: Quaternion =
            Quaternion::set(-0.1258448, 0.266531, 0.8288804, 0.4754803);

        let rotation_matrix_result: Matrix3x3 = Matrix3x3::set_from_columns(
            Vector3::set(-0.5161631, 0.7211497, -0.462_081),
            Vector3::set(-0.8553157, -0.4057594, 0.3221712),
            Vector3::set(0.0448399, 0.5615181, 0.8262486),
        );
        let rnd_quat = to_rotation_matrix(quaternion_rot_result);

        assert!((rotation_matrix_result.column_x - rnd_quat.column_x)
            .value
            .simd_lt(FP_ERROR_THRESHOLD_MATRIX.column_x.value)
            .all());

        assert!((rotation_matrix_result.column_y - rnd_quat.column_y)
            .value
            .simd_lt(FP_ERROR_THRESHOLD_MATRIX.column_y.value)
            .all());

        assert!((rotation_matrix_result.column_z - rnd_quat.column_z)
            .value
            .simd_lt(FP_ERROR_THRESHOLD_MATRIX.column_z.value)
            .all());

        let rotation_mat_to_quaternion: Quaternion = from_rotation_matrix(rotation_matrix_result);

        assert!((rotation_mat_to_quaternion - quaternion_rot_result)
            .value
            .simd_lt(FP_ERROR_THRESHOLD)
            .all());
    }


    #[test]
    fn quaternion_exp_test() {
        let quaternion_rot_result: Quaternion =
            Quaternion::set(-0.1258448, 0.266531, 0.8288804, 0.4754803);

        quaternion_exp(quaternion_rot_result);
    }


    #[test]
    fn quaternion_conj_inv_test() {
        const FP_ERROR_THRESHOLD: std::simd::f32x4 = std::simd::f32x4::from_array([0.000003; 4]);

        let quaternion: Quaternion = Quaternion::set(0.1479202, -0.1477288, 0.7230414, 0.6584125);

        let conjugate_quaternion: Quaternion = conjugate(quaternion);

        let inverse_quaternion: Quaternion = inverse(quaternion);

        assert!(abs(conjugate_quaternion.value - inverse_quaternion.value) < FP_ERROR_THRESHOLD);


        println!("conjugate quaternion {}", conjugate_quaternion);

        println!("inverse quaternion {}", inverse_quaternion);

        assert_ne!(conjugate_quaternion, inverse_quaternion);

        let normalized_quaternion: Quaternion = Quaternion {
            value: normalize(quaternion.value),
        };

        let norm_conjugate_quaternion: Quaternion = conjugate(normalized_quaternion);

        let norm_inverse_quaternion: Quaternion = inverse(normalized_quaternion);

        println!(
            "normalized conjugate quaternion {}",
            norm_conjugate_quaternion
        );

        println!("normalized inverse quaternion {}", norm_inverse_quaternion);

    }
}
