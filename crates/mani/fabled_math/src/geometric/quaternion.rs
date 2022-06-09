use crate::math::{cos, sin};
use std::fmt::{Display, Formatter};
use std::ops::{Mul, MulAssign};

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


impl Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}


impl MulAssign for Quaternion {
    fn mul_assign(&mut self, rhs: Self) {
        todo!()
    }
}


pub fn rotate_x(angle_radians: f32) -> Quaternion {
    const W: std::simd::f32x4 = std::simd::f32x4::from_array([0.0, 0.0, 0.0, 1.0]);
    const X: std::simd::f32x4 = std::simd::f32x4::from_array([1.0, 0.0, 0.0, 0.0]);
    let half_angle: std::simd::f32x4 = std::simd::f32x4::splat(angle_radians * 0.5f32);

    let quat_w = cos(half_angle) * W;
    let quat_x = sin(half_angle) * X;

    Quaternion {
        value: quat_x + quat_w,
    }
}

pub fn rotate_y(angle_radians: f32) -> Quaternion {
    const W: std::simd::f32x4 = std::simd::f32x4::from_array([0.0, 0.0, 0.0, 1.0]);
    const Y: std::simd::f32x4 = std::simd::f32x4::from_array([0.0, 1.0, 0.0, 0.0]);
    let half_angle: std::simd::f32x4 = std::simd::f32x4::splat(angle_radians * 0.5f32);

    let quat_w = cos(half_angle) * W;
    let quat_y = sin(half_angle) * Y;

    Quaternion {
        value: quat_y * quat_w,
    }
}

pub fn rotate_z(angle_radians: f32) -> Quaternion {
    const W: std::simd::f32x4 = std::simd::f32x4::from_array([0.0, 0.0, 0.0, 1.0]);
    const Z: std::simd::f32x4 = std::simd::f32x4::from_array([0.0, 0.0, 1.0, 0.0]);
    let half_angle: std::simd::f32x4 = std::simd::f32x4::splat(angle_radians * 0.5f32);

    let quat_w = cos(half_angle) * W;
    let quat_z = sin(half_angle) * Z;


    Quaternion {
        value: quat_z * quat_w,
    }
}

#[cfg(test)]
mod quaternion_test {
    use crate::rotate_x;

    #[test]
    fn rotation_x_quaternion_test() {
        let quat_x = rotate_x(45.0f32.to_radians());


        println!("{}", quat_x);

        panic!()
    }
}
