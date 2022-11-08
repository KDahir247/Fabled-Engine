use crate::{Vector3, Vector4};

use crate::math_trait::QuaternionSwizzles;

use crate::vector_math::{cross, component_sum};

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign, Neg};

use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
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

    pub const ZERO : Quaternion = Quaternion{
        value: std::simd::f32x4::from_array([0.0; 4]),
    };

    #[inline(always)]
    pub const fn set(i: f32, j: f32, k: f32, w: f32) -> Quaternion {
        Quaternion {
            value: std::simd::f32x4::from_array([i, j, k, w]),
        }
    }

    #[inline(always)]
    pub const fn broadcast(val: f32) -> Quaternion {
        Quaternion {
            value: std::simd::f32x4::from_array([val; 4]),
        }
    }

    #[inline(always)]
    pub const fn i(self) -> f32 {
        let array_vec3: [f32; 4] = self.value.to_array();

        array_vec3[0]
    }

    #[inline(always)]
    pub const fn j(self) -> f32 {
        let array_vec3: [f32; 4] = self.value.to_array();

        array_vec3[1]
    }

    #[inline(always)]
    pub const fn k(self) -> f32 {
        let array_vec3: [f32; 4] = self.value.to_array();

        array_vec3[2]
    }

    #[inline(always)]
    pub const fn w(self) -> f32 {
        let array_vec3: [f32; 4] = self.value.to_array();

        array_vec3[3]
    }

    #[inline(always)]
    pub const fn to_pure(self) -> Vector3 {
        Vector3::set(self.i(), self.j(), self.k())
    }

    #[inline(always)]
    pub const fn to_real(self) -> f32 {
        self.w()
    }

    #[inline(always)]
    pub const fn from_additive_form(real: f32, pure: Vector3) -> Quaternion {
        Quaternion {
            value: std::simd::f32x4::from_array([pure.x(), pure.y(), pure.z(), real]),
        }
    }

    #[inline]
    pub const fn to_primitive(self) -> [f32; 4] {
        self.value.to_array()
    }

    #[inline]
    pub const fn from_primitive(val: [f32; 4]) -> Quaternion {
        Quaternion {
            value: std::simd::f32x4::from_array(val),
        }
    }

    #[inline]
    pub fn scale_quaternion(self, scale_vector4: Vector4) -> Quaternion {
        Quaternion {
            value: self.value * scale_vector4.value,
        }
    }
}

impl Display for Quaternion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Quaternion (i : {}, j : {}, k : {}, w : {})",
            self.i(),
            self.j(),
            self.k(),
            self.w()
        )
    }
}

// Component-Wise
impl Add<f32> for Quaternion {
    type Output = Quaternion;

    #[inline]
    fn add(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Quaternion {
            value: self.value + splat_f32x4,
        }
    }
}

impl AddAssign<f32> for Quaternion {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value += splat_f32x4;
    }
}

impl Sub<f32> for Quaternion {
    type Output = Quaternion;

    #[inline]
    fn sub(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Quaternion {
            value: self.value - splat_f32x4,
        }
    }
}

impl SubAssign<f32> for Quaternion {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value -= splat_f32x4;
    }
}

impl Mul<f32> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: f32) -> Self::Output {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        Quaternion {
            value: self.value * splat_f32x4,
        }
    }
}

impl MulAssign<f32> for Quaternion {
    fn mul_assign(&mut self, rhs: f32) {
        let splat_f32x4: std::simd::f32x4 = std::simd::f32x4::splat(rhs);

        self.value *= splat_f32x4;
    }
}

// Quaternion-Wise
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
        let lhs_pure_vector: Vector3 = self.to_pure();
        let rhs_pure_vector: Vector3 = rhs.to_pure();

        let lhs_real: f32 = self.to_real();
        let rhs_real: f32 = rhs.to_real();

        Quaternion::from_additive_form(
            (lhs_real * rhs_real) - component_sum(lhs_pure_vector.value * rhs_pure_vector.value),
            (rhs_pure_vector * lhs_real) + (lhs_pure_vector * rhs_real)
                + Vector3 { value: cross(lhs_pure_vector.value, rhs_pure_vector.value),},
        )
    }
}

impl MulAssign for Quaternion {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.value = (*self * rhs).value;
    }
}

impl Neg for Quaternion{
    type Output = Quaternion;

    #[inline]
    fn neg(self) -> Self::Output {
        Quaternion{ value: self.value.neg() }
    }
}

pub mod quaternion_math {
    use crate::Bool4;

    use crate::vector_math::{
        component_sum, cos, dot, length, length_squared, lerp, normalize, rcp, select, sin, mul_add,
    };
    use crate::{EulerOrder, Matrix3x3, Matrix4x4, Quaternion, Vector3, Vector4};

    #[inline]
    pub fn rotate_x_quat(angle_radian: f32) -> Quaternion {
        let half_angle: f32 = angle_radian * 0.5f32;

        Quaternion::from_primitive([half_angle.sin(), 0.0, 0.0, half_angle.cos()])
    }

    #[inline]
    pub fn rotate_y_quat(angle_radian: f32) -> Quaternion {
        let half_angle: f32 = angle_radian * 0.5f32;

        Quaternion::from_primitive([0.0, half_angle.sin(), 0.0, half_angle.cos()])
    }

    #[inline]
    pub fn rotate_z_quat(angle_radian: f32) -> Quaternion {
        let half_angle: f32 = angle_radian * 0.5f32;

        Quaternion::from_primitive([0.0, 0.0, half_angle.sin(), half_angle.cos()])
    }

    #[inline]
    pub fn to_angle_axis_vec4(quaternion: Quaternion) -> Vector4 {
        let quaternion_real_acos = quaternion.to_real().acos();

        let angle: f32 = quaternion_real_acos + quaternion_real_acos;
        let scale: f32 = 1.0 - (quaternion.to_real() * quaternion.to_real());

        let mut axis: Vector3 = quaternion.to_pure() * scale.sqrt().recip();

        if std::intrinsics::unlikely(scale < f32::EPSILON) {
            axis = Vector3::RIGHT;
        }

        Vector4::set(axis.x(), axis.y(), axis.z(), angle)
    }

    #[inline]
    pub fn to_angle_axis_mag_vec3(quaternion: Quaternion) -> Vector3 {
        let axis_rot_angle: Vector4 = to_angle_axis_vec4(quaternion);

        axis_rot_angle.trunc_vec3() * axis_rot_angle.w()
    }

    #[inline]
    pub fn from_angle_axis_quat(normalized_axis: Vector3, angle_radian: f32) -> Quaternion {
        let half_angle: f32 = angle_radian * 0.5f32;

        let quaternion_pure: Vector3 = normalized_axis * half_angle.sin();

        let quaternion_real: f32 = half_angle.cos();

        Quaternion::from_additive_form(quaternion_real, quaternion_pure)
    }

    #[inline]
    pub fn from_angle_axis_mag_quat(axis_mag: Vector3) -> Quaternion {
        let angle: f32 = length(axis_mag.value);

        let axis: Vector3 = axis_mag * angle.recip();

        from_angle_axis_quat(axis, angle)
    }

    #[inline]
    pub fn from_euler_quat(euler_radians: Vector3, euler_order: EulerOrder) -> Quaternion {
        let half_euler_rad: Vector3 = euler_radians * 0.5f32;

        let cos_half_euler_rad: Vector3 = Vector3 {
            value: cos(half_euler_rad.value),
        };

        let sin_half_euler_rad: Vector3 = Vector3 {
            value: sin(half_euler_rad.value),
        };

        let s_half_c: f32 = sin_half_euler_rad.x() * cos_half_euler_rad.y();
        let c_half_s: f32 = cos_half_euler_rad.x() * sin_half_euler_rad.y();
        let c_half_c: f32 = cos_half_euler_rad.x() * cos_half_euler_rad.y();
        let s_half_s: f32 = sin_half_euler_rad.x() * sin_half_euler_rad.y();

        let rhs_quaternion_equation: Quaternion = Quaternion::set(
            s_half_c * cos_half_euler_rad.z(),
            c_half_s * cos_half_euler_rad.z(),
            c_half_c * sin_half_euler_rad.z(),
            c_half_c * cos_half_euler_rad.z(),
        );

        let lhs_quaternion_equation: Quaternion = Quaternion::set(
            c_half_s * sin_half_euler_rad.z(),
            s_half_c * sin_half_euler_rad.z(),
            s_half_s * cos_half_euler_rad.z(),
            s_half_s * sin_half_euler_rad.z(),
        );

        Quaternion{ value: mul_add(lhs_quaternion_equation.value, euler_order.0.value, rhs_quaternion_equation.value) }
    }

    pub fn from_transformation_matrix_quat(transformation_matrix: Matrix4x4) -> Quaternion {
        let column_x_normalized: Vector3 = Vector3 { value: normalize(transformation_matrix.column_x.trunc_vec3().value) };
        let column_y_normalized: Vector3 = Vector3 { value: normalize(transformation_matrix.column_y.trunc_vec3().value) };
        let column_z_normalized: Vector3 = Vector3 { value: normalize(transformation_matrix.column_z.trunc_vec3().value) };

        let rotation_matrix: Matrix3x3 = Matrix3x3::set(column_x_normalized, column_y_normalized, column_z_normalized);

        from_rotation_matrix_quat(rotation_matrix)
    }

    pub fn from_rotation_matrix_quat(rotation_matrix: Matrix3x3) -> Quaternion {
        let mut quaternion_unscaled: std::simd::f32x4 = std::simd::f32x4::from_array([0.0; 4]);

        let mut _s0: f32 = 0.0;
        let mut _s1: f32 = 0.0;
        let mut _s2: f32 = 0.0;

        let mut _k0: usize = 0;
        let mut _k1: usize = 0;
        let mut _k2: usize = 0;
        let mut _k3: usize = 0;

        let diagonal_col = Vector3::set(rotation_matrix.column_x.x(), rotation_matrix.column_y.y(), rotation_matrix.column_z.z());

        if component_sum(diagonal_col.value) > 0.0 {
            _k0 = 3;
            _k1 = 2;
            _k2 = 1;
            _k3 = 0;

            _s0 = 1.0;
            _s1 = 1.0;
            _s2 = 1.0;
        } else if diagonal_col.x() > diagonal_col.y() && diagonal_col.x() > diagonal_col.z() {
            _k0 = 0;
            _k1 = 1;
            _k2 = 2;
            _k3 = 3;

            _s0 = 1.0;
            _s1 = -1.0;
            _s2 = -1.0;
        } else if diagonal_col.y() > diagonal_col.z() {
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

        let s_vector : Vector3 = Vector3::set(_s0, _s1, _s2);

        let t: f32 = component_sum((s_vector * diagonal_col).value) + 1.0;

        let s2_yx: f32 = s_vector.z() * rotation_matrix.column_y.x();
        let s1_xz: f32 = s_vector.y() * rotation_matrix.column_x.z();
        let s0_zy: f32 = s_vector.x() * rotation_matrix.column_z.y();

        quaternion_unscaled[_k1] = rotation_matrix.column_x.y() - s2_yx;
        quaternion_unscaled[_k2] = rotation_matrix.column_z.x() - s1_xz;
        quaternion_unscaled[_k3] = rotation_matrix.column_y.z() - s0_zy;
        quaternion_unscaled[_k0] = t;

        let scalar: f32 = t.sqrt().recip() * 0.5f32;

        Quaternion {
            value: quaternion_unscaled,
        } * scalar
    }

    #[inline]
    pub fn conjugate_quat(quaternion: Quaternion) -> Quaternion {
        Quaternion::set(-quaternion.i(), -quaternion.j(), -quaternion.k(), quaternion.w())
    }

    #[inline]
    pub fn inverse_quat(quaternion: Quaternion) -> Quaternion {
        Quaternion {
            value: conjugate_quat(quaternion).value
                * rcp(Vector4::broadcast(length_squared(quaternion.value)).value),
        }
    }

    #[inline]
    pub fn forward_vec3(quaternion: Quaternion) -> Vector3 {
        Vector3::FORWARD * quaternion
    }

    #[inline]
    pub fn right_vec3(quaternion: Quaternion) -> Vector3 {
        Vector3::RIGHT * quaternion
    }

    #[inline]
    pub fn up_vec3(quaternion: Quaternion) -> Vector3 {
        Vector3::UP * quaternion
    }

    #[inline]
    pub fn normalized_lerp_quat(
        start_quaternion: Quaternion,
        target_quaternion: Quaternion,
        delta: f32,
    ) -> Quaternion {
        let target_quaternion: std::simd::f32x4 = select(
            target_quaternion.value,
            -target_quaternion.value,
            Bool4::broadcast(dot(start_quaternion.value, target_quaternion.value).ge(&0.0)).value,
        );

        let linear_interpolated_quaternion: std::simd::f32x4 = lerp(
            start_quaternion.value,
            target_quaternion,
            Vector4::broadcast(delta).value,
        );

        Quaternion {
            value: normalize(linear_interpolated_quaternion),
        }
    }

    #[inline]
    pub fn slerp_quat(
        start_quaternion: Quaternion,
        target_quaternion: Quaternion,
        delta: f32,
    ) -> Quaternion {
        let normalized_dot: f32 = dot(start_quaternion.value, target_quaternion.value);

        let angle: f32 = normalized_dot.acos();

        let rcp_sin_angle: f32 = angle.sin().recip();

        let delta_difference: f32 = 1.0 - delta;

        let delta_mul_angle_sin: f32 = (delta * angle).sin();
        let delta_diff_mul_angle_sin: f32 = (delta_difference * angle).sin();

        let weight_start: f32 = delta_diff_mul_angle_sin * rcp_sin_angle;
        let weight_target: f32 = delta_mul_angle_sin * rcp_sin_angle;

        (start_quaternion * weight_start) + (target_quaternion * weight_target)
    }

    pub fn log_quat(quaternion: Quaternion) -> Quaternion {
        let quaternion_real: f32 = quaternion.to_real();
        let quaternion_pure: Vector3 = quaternion.to_pure();

        let quaternion_length: f32 = length(quaternion.value);

        let real_norm: f32 = (quaternion_real * quaternion_length.recip()).clamp(-1.0, 1.0);
        let arc_cos_real_norm: f32 = real_norm.acos();

        let pure_quaternion: Vector3 =
            quaternion_pure * length(quaternion_pure.value).recip() * arc_cos_real_norm;
        let real_quaternion: f32 = 0.5f32 * quaternion_length.log2();

        Quaternion::from_additive_form(real_quaternion, pure_quaternion)
    }

    pub fn exp_quat(quaternion: Quaternion) -> Quaternion {
        let quaternion_real: f32 = quaternion.to_real();
        let quaternion_pure: Vector3 = quaternion.to_pure();

        let quaternion_pure_length: f32 = length(quaternion_pure.value);

        let (sin_pure_len, cos_pure_len) = quaternion_pure_length.sin_cos();

        let pure_exp_quaternion: Vector3 =
            quaternion_pure * quaternion_pure_length.recip() * sin_pure_len;

        let real_exp_quaternion: f32 = cos_pure_len;

        Quaternion::from_additive_form(real_exp_quaternion, pure_exp_quaternion)
            * quaternion_real.exp()
    }
}


impl QuaternionSwizzles for Quaternion {
    #[inline]
    fn iiii(self) -> Self {
        Quaternion::broadcast(self.i())
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
        Quaternion::broadcast(self.j())
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
        let kiik = std::simd::simd_swizzle!(self.value, [2, 0, 0, 2]);

        Quaternion { value: kiik }
    }

    #[inline]
    fn kiiw(self) -> Self {
        let kiiw = std::simd::simd_swizzle!(self.value, [2, 0, 0, 3]);

        Quaternion { value: kiiw }
    }

    #[inline]
    fn kiji(self) -> Self {
        let kiji = std::simd::simd_swizzle!(self.value, [2, 0, 1, 0]);

        Quaternion { value: kiji }
    }

    #[inline]
    fn kijj(self) -> Self {
        let kijj = std::simd::simd_swizzle!(self.value, [2, 0, 1, 1]);

        Quaternion { value: kijj }
    }

    #[inline]
    fn kijk(self) -> Self {
        let kijk = std::simd::simd_swizzle!(self.value, [2, 0, 1, 2]);

        Quaternion { value: kijk }
    }

    #[inline]
    fn kijw(self) -> Self {
        let kijw = std::simd::simd_swizzle!(self.value, [2, 0, 1, 3]);

        Quaternion { value: kijw }
    }

    #[inline]
    fn kiki(self) -> Self {
        let kiki = std::simd::simd_swizzle!(self.value, [2, 0, 2, 0]);

        Quaternion { value: kiki }
    }

    #[inline]
    fn kikj(self) -> Self {
        let kikj = std::simd::simd_swizzle!(self.value, [2, 0, 2, 1]);

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
        let kjik = std::simd::simd_swizzle!(self.value, [2, 1, 0, 2]);

        Quaternion { value: kjik }
    }

    #[inline]
    fn kjiw(self) -> Self {
        let kjiw = std::simd::simd_swizzle!(self.value, [2, 1, 0, 3]);

        Quaternion { value: kjiw }
    }

    #[inline]
    fn kjji(self) -> Self {
        let kjji = std::simd::simd_swizzle!(self.value, [2, 1, 1, 0]);

        Quaternion { value: kjji }
    }

    #[inline]
    fn kjjj(self) -> Self {
        let kjjj = std::simd::simd_swizzle!(self.value, [2, 1, 1, 1]);

        Quaternion { value: kjjj }
    }

    #[inline]
    fn kjjk(self) -> Self {
        let kjjk = std::simd::simd_swizzle!(self.value, [2, 1, 1, 2]);

        Quaternion { value: kjjk }
    }

    #[inline]
    fn kjjw(self) -> Self {
        let kjjw = std::simd::simd_swizzle!(self.value, [2, 1, 1, 3]);

        Quaternion { value: kjjw }
    }

    #[inline]
    fn kjki(self) -> Self {
        let kjki = std::simd::simd_swizzle!(self.value, [2, 1, 2, 0]);

        Quaternion { value: kjki }
    }

    #[inline]
    fn kjkj(self) -> Self {
        let kjkj = std::simd::simd_swizzle!(self.value, [2, 1, 2, 1]);

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
        let kjwj = std::simd::simd_swizzle!(self.value, [2, 1, 3, 1]);

        Quaternion { value: kjwj }
    }

    #[inline]
    fn kjwk(self) -> Self {
        let kjwk = std::simd::simd_swizzle!(self.value, [2, 1, 3, 2]);

        Quaternion { value: kjwk }
    }

    #[inline]
    fn kjww(self) -> Self {
        let kjww = std::simd::simd_swizzle!(self.value, [2, 1, 3, 3]);

        Quaternion { value: kjww }
    }

    #[inline]
    fn kkii(self) -> Self {
        let kkii = std::simd::simd_swizzle!(self.value, [2, 2, 0, 0]);

        Quaternion { value: kkii }
    }

    #[inline]
    fn kkij(self) -> Self {
        let kkij = std::simd::simd_swizzle!(self.value, [2, 2, 0, 1]);

        Quaternion { value: kkij }
    }

    #[inline]
    fn kkik(self) -> Self {
        let kkik = std::simd::simd_swizzle!(self.value, [2, 2, 0, 2]);

        Quaternion { value: kkik }
    }

    #[inline]
    fn kkiw(self) -> Self {
        let kkiw = std::simd::simd_swizzle!(self.value, [2, 2, 0, 3]);

        Quaternion { value: kkiw }
    }

    #[inline]
    fn kkji(self) -> Self {
        let kkji = std::simd::simd_swizzle!(self.value, [2, 2, 1, 0]);

        Quaternion { value: kkji }
    }

    #[inline]
    fn kkjj(self) -> Self {
        let kkjj = std::simd::simd_swizzle!(self.value, [2, 2, 1, 1]);

        Quaternion { value: kkjj }
    }

    #[inline]
    fn kkjk(self) -> Self {
        let kkjk = std::simd::simd_swizzle!(self.value, [2, 2, 1, 2]);

        Quaternion { value: kkjk }
    }

    #[inline]
    fn kkjw(self) -> Self {
        let kkjw = std::simd::simd_swizzle!(self.value, [2, 2, 1, 3]);

        Quaternion { value: kkjw }
    }

    #[inline]
    fn kkki(self) -> Self {
        let kkki = std::simd::simd_swizzle!(self.value, [2, 2, 2, 0]);

        Quaternion { value: kkki }
    }

    #[inline]
    fn kkkj(self) -> Self {
        let kkkj = std::simd::simd_swizzle!(self.value, [2, 2, 2, 1]);

        Quaternion { value: kkkj }
    }

    #[inline]
    fn kkkk(self) -> Self {
        Quaternion::broadcast(self.k())
    }

    #[inline]
    fn kkkw(self) -> Self {
        let kkkw = std::simd::simd_swizzle!(self.value, [2, 2, 2, 3]);

        Quaternion { value: kkkw }
    }

    #[inline]
    fn kkwi(self) -> Self {
        let kkwi = std::simd::simd_swizzle!(self.value, [2, 2, 3, 0]);

        Quaternion { value: kkwi }
    }

    #[inline]
    fn kkwj(self) -> Self {
        let kkwj = std::simd::simd_swizzle!(self.value, [2, 2, 3, 1]);

        Quaternion { value: kkwj }
    }

    #[inline]
    fn kkwk(self) -> Self {
        let kkwk = std::simd::simd_swizzle!(self.value, [2, 2, 3, 2]);

        Quaternion { value: kkwk }
    }

    #[inline]
    fn kkww(self) -> Self {
        let kkww = std::simd::simd_swizzle!(self.value, [2, 2, 3, 3]);

        Quaternion { value: kkww }
    }

    #[inline]
    fn kwii(self) -> Self {
        let kwii = std::simd::simd_swizzle!(self.value, [2, 3, 0, 0]);

        Quaternion { value: kwii }
    }

    #[inline]
    fn kwij(self) -> Self {
        let kwij = std::simd::simd_swizzle!(self.value, [2, 3, 0, 1]);

        Quaternion { value: kwij }
    }

    #[inline]
    fn kwik(self) -> Self {
        let kwik = std::simd::simd_swizzle!(self.value, [2, 3, 0, 2]);

        Quaternion { value: kwik }
    }

    #[inline]
    fn kwiw(self) -> Self {
        let kwiw = std::simd::simd_swizzle!(self.value, [2, 3, 0, 3]);

        Quaternion { value: kwiw }
    }

    #[inline]
    fn kwji(self) -> Self {
        let kwji = std::simd::simd_swizzle!(self.value, [2, 3, 1, 0]);

        Quaternion { value: kwji }
    }

    #[inline]
    fn kwjj(self) -> Self {
        let kwjj = std::simd::simd_swizzle!(self.value, [2, 3, 1, 1]);

        Quaternion { value: kwjj }
    }

    #[inline]
    fn kwjk(self) -> Self {
        let kwjk = std::simd::simd_swizzle!(self.value, [2, 3, 1, 2]);

        Quaternion { value: kwjk }
    }

    #[inline]
    fn kwjw(self) -> Self {
        let kwjw = std::simd::simd_swizzle!(self.value, [2, 3, 1, 3]);

        Quaternion { value: kwjw }
    }

    #[inline]
    fn kwki(self) -> Self {
        let kwki = std::simd::simd_swizzle!(self.value, [2, 3, 2, 0]);

        Quaternion { value: kwki }
    }

    #[inline]
    fn kwkj(self) -> Self {
        let kwkj = std::simd::simd_swizzle!(self.value, [2, 3, 2, 1]);

        Quaternion { value: kwkj }
    }

    #[inline]
    fn kwkk(self) -> Self {
        let kwkk = std::simd::simd_swizzle!(self.value, [2, 3, 2, 2]);

        Quaternion { value: kwkk }
    }

    #[inline]
    fn kwkw(self) -> Self {
        let kwkw = std::simd::simd_swizzle!(self.value, [2, 3, 2, 3]);

        Quaternion { value: kwkw }
    }

    #[inline]
    fn kwwi(self) -> Self {
        let kwwi = std::simd::simd_swizzle!(self.value, [2, 3, 3, 0]);

        Quaternion { value: kwwi }
    }

    #[inline]
    fn kwwj(self) -> Self {
        let kwwj = std::simd::simd_swizzle!(self.value, [2, 3, 3, 1]);

        Quaternion { value: kwwj }
    }

    #[inline]
    fn kwwk(self) -> Self {
        let kwwk = std::simd::simd_swizzle!(self.value, [2, 3, 3, 2]);

        Quaternion { value: kwwk }
    }

    #[inline]
    fn kwww(self) -> Self {
        let kwww = std::simd::simd_swizzle!(self.value, [2, 3, 3, 3]);

        Quaternion { value: kwww }
    }

    #[inline]
    fn wiii(self) -> Self {
        let wiii = std::simd::simd_swizzle!(self.value, [3, 0, 0, 0]);

        Quaternion { value: wiii }
    }

    #[inline]
    fn wiij(self) -> Self {
        let wiij = std::simd::simd_swizzle!(self.value, [3, 0, 0, 1]);

        Quaternion { value: wiij }
    }

    #[inline]
    fn wiik(self) -> Self {
        let wiik = std::simd::simd_swizzle!(self.value, [3, 0, 0, 2]);

        Quaternion { value: wiik }
    }

    #[inline]
    fn wiiw(self) -> Self {
        let wiiw = std::simd::simd_swizzle!(self.value, [3, 0, 0, 3]);

        Quaternion { value: wiiw }
    }

    #[inline]
    fn wiji(self) -> Self {
        let wiji = std::simd::simd_swizzle!(self.value, [3, 0, 1, 0]);

        Quaternion { value: wiji }
    }

    #[inline]
    fn wijj(self) -> Self {
        let wijj = std::simd::simd_swizzle!(self.value, [3, 0, 1, 1]);

        Quaternion { value: wijj }
    }

    #[inline]
    fn wijk(self) -> Self {
        let wijk = std::simd::simd_swizzle!(self.value, [3, 0, 1, 2]);

        Quaternion { value: wijk }
    }

    #[inline]
    fn wijw(self) -> Self {
        let wijw = std::simd::simd_swizzle!(self.value, [3, 0, 1, 3]);

        Quaternion { value: wijw }
    }

    #[inline]
    fn wiki(self) -> Self {
        let wiki = std::simd::simd_swizzle!(self.value, [3, 0, 2, 0]);

        Quaternion { value: wiki }
    }

    #[inline]
    fn wikj(self) -> Self {
        let wikj = std::simd::simd_swizzle!(self.value, [3, 0, 2, 1]);

        Quaternion { value: wikj }
    }

    #[inline]
    fn wikk(self) -> Self {
        let wikk = std::simd::simd_swizzle!(self.value, [3, 0, 2, 2]);

        Quaternion { value: wikk }
    }

    #[inline]
    fn wikw(self) -> Self {
        let wikw = std::simd::simd_swizzle!(self.value, [3, 0, 2, 3]);

        Quaternion { value: wikw }
    }

    #[inline]
    fn wiwi(self) -> Self {
        let wiwi = std::simd::simd_swizzle!(self.value, [3, 0, 3, 0]);

        Quaternion { value: wiwi }
    }

    #[inline]
    fn wiwj(self) -> Self {
        let wiwj = std::simd::simd_swizzle!(self.value, [3, 0, 3, 1]);

        Quaternion { value: wiwj }
    }

    #[inline]
    fn wiwk(self) -> Self {
        let wiwk = std::simd::simd_swizzle!(self.value, [3, 0, 3, 2]);

        Quaternion { value: wiwk }
    }

    #[inline]
    fn wiww(self) -> Self {
        let wiww = std::simd::simd_swizzle!(self.value, [3, 0, 3, 3]);

        Quaternion { value: wiww }
    }

    #[inline]
    fn wjii(self) -> Self {
        let wjii = std::simd::simd_swizzle!(self.value, [3, 1, 0, 0]);

        Quaternion { value: wjii }
    }

    #[inline]
    fn wjij(self) -> Self {
        let wjij = std::simd::simd_swizzle!(self.value, [3, 1, 0, 1]);

        Quaternion { value: wjij }
    }

    #[inline]
    fn wjik(self) -> Self {
        let wjik = std::simd::simd_swizzle!(self.value, [3, 1, 0, 2]);

        Quaternion { value: wjik }
    }

    #[inline]
    fn wjiw(self) -> Self {
        let wjiw = std::simd::simd_swizzle!(self.value, [3, 1, 0, 3]);

        Quaternion { value: wjiw }
    }

    #[inline]
    fn wjji(self) -> Self {
        let wjji = std::simd::simd_swizzle!(self.value, [3, 1, 1, 0]);

        Quaternion { value: wjji }
    }

    #[inline]
    fn wjjj(self) -> Self {
        let wjjj = std::simd::simd_swizzle!(self.value, [3, 1, 1, 1]);

        Quaternion { value: wjjj }
    }

    #[inline]
    fn wjjk(self) -> Self {
        let wjjk = std::simd::simd_swizzle!(self.value, [3, 1, 1, 2]);

        Quaternion { value: wjjk }
    }

    #[inline]
    fn wjjw(self) -> Self {
        let wjjw = std::simd::simd_swizzle!(self.value, [3, 1, 1, 3]);

        Quaternion { value: wjjw }
    }

    #[inline]
    fn wjki(self) -> Self {
        let wjki = std::simd::simd_swizzle!(self.value, [3, 1, 2, 0]);

        Quaternion { value: wjki }
    }

    #[inline]
    fn wjkj(self) -> Self {
        let wjkj = std::simd::simd_swizzle!(self.value, [3, 1, 2, 1]);

        Quaternion { value: wjkj }
    }

    #[inline]
    fn wjkk(self) -> Self {
        let wjkk = std::simd::simd_swizzle!(self.value, [3, 1, 2, 2]);

        Quaternion { value: wjkk }
    }

    #[inline]
    fn wjkw(self) -> Self {
        let wjkw = std::simd::simd_swizzle!(self.value, [3, 1, 2, 3]);

        Quaternion { value: wjkw }
    }

    #[inline]
    fn wjwi(self) -> Self {
        let wjwi = std::simd::simd_swizzle!(self.value, [3, 1, 3, 0]);

        Quaternion { value: wjwi }
    }

    #[inline]
    fn wjwj(self) -> Self {
        let wjwj = std::simd::simd_swizzle!(self.value, [3, 1, 3, 1]);

        Quaternion { value: wjwj }
    }

    #[inline]
    fn wjwk(self) -> Self {
        let wjwk = std::simd::simd_swizzle!(self.value, [3, 1, 3, 2]);

        Quaternion { value: wjwk }
    }

    #[inline]
    fn wjww(self) -> Self {
        let wjww = std::simd::simd_swizzle!(self.value, [3, 1, 3, 3]);

        Quaternion { value: wjww }
    }

    #[inline]
    fn wkii(self) -> Self {
        let wkii = std::simd::simd_swizzle!(self.value, [3, 2, 0, 0]);

        Quaternion { value: wkii }
    }

    #[inline]
    fn wkij(self) -> Self {
        let wkij = std::simd::simd_swizzle!(self.value, [3, 2, 0, 1]);

        Quaternion { value: wkij }
    }

    #[inline]
    fn wkik(self) -> Self {
        let wkik = std::simd::simd_swizzle!(self.value, [3, 2, 0, 2]);

        Quaternion { value: wkik }
    }

    #[inline]
    fn wkiw(self) -> Self {
        let wkiw = std::simd::simd_swizzle!(self.value, [3, 2, 0, 3]);

        Quaternion { value: wkiw }
    }

    #[inline]
    fn wkji(self) -> Self {
        let wkji = std::simd::simd_swizzle!(self.value, [3, 2, 1, 0]);

        Quaternion { value: wkji }
    }

    #[inline]
    fn wkjj(self) -> Self {
        let wkjj = std::simd::simd_swizzle!(self.value, [3, 2, 1, 1]);

        Quaternion { value: wkjj }
    }

    #[inline]
    fn wkjk(self) -> Self {
        let wkjk = std::simd::simd_swizzle!(self.value, [3, 2, 1, 2]);

        Quaternion { value: wkjk }
    }

    #[inline]
    fn wkjw(self) -> Self {
        let wkjw = std::simd::simd_swizzle!(self.value, [3, 2, 1, 3]);

        Quaternion { value: wkjw }
    }

    #[inline]
    fn wkki(self) -> Self {
        let wkki = std::simd::simd_swizzle!(self.value, [3, 2, 2, 0]);

        Quaternion { value: wkki }
    }

    #[inline]
    fn wkkj(self) -> Self {
        let wkkj = std::simd::simd_swizzle!(self.value, [3, 2, 2, 1]);

        Quaternion { value: wkkj }
    }

    #[inline]
    fn wkkk(self) -> Self {
        let wkkk = std::simd::simd_swizzle!(self.value, [3, 2, 2, 2]);

        Quaternion { value: wkkk }
    }

    #[inline]
    fn wkkw(self) -> Self {
        let wkkw = std::simd::simd_swizzle!(self.value, [3, 2, 2, 3]);

        Quaternion { value: wkkw }
    }

    #[inline]
    fn wkwi(self) -> Self {
        let wkwi = std::simd::simd_swizzle!(self.value, [3, 2, 3, 0]);

        Quaternion { value: wkwi }
    }

    #[inline]
    fn wkwj(self) -> Self {
        let wkwj = std::simd::simd_swizzle!(self.value, [3, 2, 3, 1]);

        Quaternion { value: wkwj }
    }

    #[inline]
    fn wkwk(self) -> Self {
        let wkwk = std::simd::simd_swizzle!(self.value, [3, 2, 3, 2]);

        Quaternion { value: wkwk }
    }

    #[inline]
    fn wkww(self) -> Self {
        let wkww = std::simd::simd_swizzle!(self.value, [3, 2, 3, 3]);

        Quaternion { value: wkww }
    }

    #[inline]
    fn wwii(self) -> Self {
        let wwii = std::simd::simd_swizzle!(self.value, [3, 3, 0, 0]);

        Quaternion { value: wwii }
    }

    #[inline]
    fn wwij(self) -> Self {
        let wwij = std::simd::simd_swizzle!(self.value, [3, 3, 0, 1]);

        Quaternion { value: wwij }
    }

    #[inline]
    fn wwik(self) -> Self {
        let wwik = std::simd::simd_swizzle!(self.value, [3, 3, 0, 2]);

        Quaternion { value: wwik }
    }

    #[inline]
    fn wwiw(self) -> Self {
        let wwiw = std::simd::simd_swizzle!(self.value, [3, 3, 0, 3]);

        Quaternion { value: wwiw }
    }

    #[inline]
    fn wwji(self) -> Self {
        let wwji = std::simd::simd_swizzle!(self.value, [3, 3, 1, 0]);

        Quaternion { value: wwji }
    }

    #[inline]
    fn wwjj(self) -> Self {
        let wwjj = std::simd::simd_swizzle!(self.value, [3, 3, 1, 1]);

        Quaternion { value: wwjj }
    }

    #[inline]
    fn wwjk(self) -> Self {
        let wwjk = std::simd::simd_swizzle!(self.value, [3, 3, 1, 2]);

        Quaternion { value: wwjk }
    }

    #[inline]
    fn wwjw(self) -> Self {
        let wwjw = std::simd::simd_swizzle!(self.value, [3, 3, 1, 3]);

        Quaternion { value: wwjw }
    }

    #[inline]
    fn wwki(self) -> Self {
        let wwki = std::simd::simd_swizzle!(self.value, [3, 3, 2, 0]);

        Quaternion { value: wwki }
    }

    #[inline]
    fn wwkj(self) -> Self {
        let wwkj = std::simd::simd_swizzle!(self.value, [3, 3, 2, 1]);

        Quaternion { value: wwkj }
    }

    #[inline]
    fn wwkk(self) -> Self {
        let wwkk = std::simd::simd_swizzle!(self.value, [3, 3, 2, 2]);

        Quaternion { value: wwkk }
    }

    #[inline]
    fn wwkw(self) -> Self {
        let wwkw = std::simd::simd_swizzle!(self.value, [3, 3, 2, 3]);

        Quaternion { value: wwkw }
    }

    #[inline]
    fn wwwi(self) -> Self {
        let wwwi = std::simd::simd_swizzle!(self.value, [3, 3, 3, 0]);

        Quaternion { value: wwwi }
    }

    #[inline]
    fn wwwj(self) -> Self {
        let wwwj = std::simd::simd_swizzle!(self.value, [3, 3, 3, 1]);

        Quaternion { value: wwwj }
    }

    #[inline]
    fn wwwk(self) -> Self {
        let wwwk = std::simd::simd_swizzle!(self.value, [3, 3, 3, 2]);

        Quaternion { value: wwwk }
    }

    #[inline]
    fn wwww(self) -> Self {
        Quaternion::broadcast(self.w())
    }
}