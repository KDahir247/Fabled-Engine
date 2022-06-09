use crate::{Matrix3x3, Vector3};

#[derive(Default)]
pub struct Affine3 {
    pub translation: Vector3,
    pub matrix3: Matrix3x3,
}


// impl Affine3 {
//     // euler is in radians
//     #[rustfmt::skip]
//     pub fn new(translation: Vector3, euler: Vector3) -> Self {
//
//         const HALF : std::simd::Simd<f32, 4> =  std::simd::Simd::splat(2.0);
//         let intermediate_step: std::simd::Simd<f32, 4> = euler.value * HALF;
//
//         // [euler_x * 0.5, euler_y * 0.5, euler_z  * 0.5]
//         let array_representation : [f32; 4] = intermediate_step.to_array();
//
//         // we need to get sin and cos and then do a swizzle (3(0),3(0),sin,
// cos);
//
//         let x = glam::Quat::from_rotation_x(euler[0]);
//         let y = glam::Quat::from_rotation_y(euler[1]);
//         let z = glam::Quat::from_rotation_z(euler[2]);
//
//         let quaternion = x * y * z;
//
//         let x2 = quaternion.x * quaternion.x;
//         let y2 = quaternion.y * quaternion.y;
//         let z2 = quaternion.z * quaternion.z;
//         let xy = quaternion.x * quaternion.y;
//         let xz = quaternion.x * quaternion.z;
//         let yz = quaternion.y * quaternion.z;
//         let wx = quaternion.w * quaternion.x;
//         let wy = quaternion.w * quaternion.y;
//         let wz = quaternion.w * quaternion.z;
//
//        let rotation_matrix =  [
//             1.0 - 2.0 * (y2 + z2), 2.0 * (xy + wz), 2.0 * (xz - wy),//col 0
//             2.0 * (xy - wz), 1.0 - 2.0 * (x2 + z2), 2.0 * (yz + wx),//col 1
//             2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (x2 + y2) //col 2
//         ];
//
//
//
//         Self{
//             translation,
//             matrix3: rotation_matrix
//         }
//     }
// }
//
//
// #[cfg(test)]
// mod affine3d_test {
//     use crate::Affine3;
//
//     #[test]
//     fn affine_creation_test() {
//         let affine3d = Affine3::new([0.0; 3], [13.5, 45.0, 90.0]);
//     }
// }
