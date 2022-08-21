// use crate::mesh::Indices::U32;
// use crate::mesh::{Mesh, Vertex};
// use fabled_math::math::{cos, cross, normalize, reverse, sin};
// use fabled_math::{Vector3, Vector4};
// use smallvec::ToSmallVec;
//
// // todo don't use push use memcopy set the vec to the specified length.
//
// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct Cone<const TESS: usize> {
//     pub apex_position: [f32; 3],
//     pub radius: f32,
//     pub height: f32,
// }
//
// impl Default for Cone<16> {
//     fn default() -> Self {
//         Self::new(5.0, 2., [0., 2., 0.])
//     }
// }
//
// impl<const TESS: usize> Cone<TESS> {
//     pub fn new(radius: f32, height: f32, apex_position: [f32; 3]) ->
// Cone<TESS> {         // We can't technically have a cone with less than three
// tessellation slice for         // the base. right?
//         Self {
//             radius,
//             height,
//             apex_position,
//         }
//     }
// }
//
// impl<const TESS: usize> From<Cone<TESS>> for Mesh
// where
//     [(); TESS * 6]:,
//     [(); TESS + 3]:,
// {
//     fn from(cone: Cone<TESS>) -> Self {
//         let Cone {
//             radius,
//             height,
//             apex_position,
//         } = cone;
//
//         const DEFAULT_VERTEX: Vertex = Vertex::init();
//
//         let mut indices = [0_u32; TESS * 6];
//
//         const DEFAULT_VERTICES: Vertex = Vertex::init();
//
//         let apex_position: Vector3 = Vector3::from_array(apex_position);
//
//         let apex_position_norm = Vector3 {
//             value: normalize(apex_position.value),
//         };
//
//         let center = apex_position + (-apex_position_norm * height);
//
//         let forward_dir = Vector3 {
//             value: cross(Vector3::RIGHT.value, apex_position_norm.value),
//         };
//
//         let rcp_tesselation_slice = 1.0 / TESS as f32;
//
//         let angle_inc = std::f32::consts::TAU * rcp_tesselation_slice;
//
//         let apex_pos: Vertex = Vertex {
//             position: apex_position.to_primitive(),
//             tex_coord: [0.0, 1.0],
//             normal: [0.0, 1.0, 0.0, 0.0],
//             tangent: [-1.0, 0.0, 0.0, 1.0],
//             bi_tangent: [0.0, 0.0, 1.0, 1.0],
//         };
//
//         let base_center_pos: Vertex = Vertex {
//             position: center.to_primitive(),
//             tex_coord: [0.0, -1.0],
//             normal: [0.0, -1.0, 0.0, 0.0],
//             tangent: [-1.0, 0.0, 0.0, 1.0],
//             bi_tangent: [0.0, 0.0, -1.0, 1.0],
//         };
//
//         let mut vertices = [DEFAULT_VERTICES; TESS + 3];
//
//         vertices[..2].copy_from_slice(&[apex_pos, base_center_pos]);
//
//
//         // todo we can unroll this. handle remainder case
//         let chunk = vertices[2..].chunks_exact_mut(4);
//
//         chunk.enumerate().for_each(|(index, vertices_chunk)| {
//             let index_f32 = index as f32;
//
//             let side_vec4 = Vector4::from_array([
//                 index_f32 * 4.0,
//                 index_f32 * 4.0 + 1.0,
//                 index_f32 * 4.0 + 2.0,
//                 index_f32 * 4.0 + 3.0,
//             ]);
//
//             let angle_inc_vec4 = Vector4::splat(angle_inc);
//
//             let [rad_sin_0, rad_sin_1, rad_sin_2, rad_sin_3] = Vector4 {
//                 value: sin((angle_inc_vec4 * side_vec4).value),
//             }
//             .to_primitive();
//
//             let [rad_cos_0, rad_cos_1, rad_cos_2, rad_cos_3] = Vector4 {
//                 value: cos((angle_inc_vec4 * side_vec4).value),
//             }
//             .to_primitive();
//
//             let [vertex_pos_3, vertex_pos_2, vertex_pos_1, vertex_pos_0] = [
//                 center + (Vector3::RIGHT * rad_cos_3 + forward_dir *
// rad_sin_3) * radius,                 center + (Vector3::RIGHT * rad_cos_2 +
// forward_dir * rad_sin_2) * radius,                 center + (Vector3::RIGHT *
// rad_cos_1 + forward_dir * rad_sin_1) * radius,                 center +
// (Vector3::RIGHT * rad_cos_0 + forward_dir * rad_sin_0) * radius,             
// ];
//
//             let [slant_height_3, slant_height_0, slant_height_1,
// slant_height_2] = [                 Vector3::UP - vertex_pos_3,
//                 Vector3::UP - vertex_pos_0,
//                 Vector3::UP - vertex_pos_1,
//                 Vector3::UP - vertex_pos_2,
//             ];
//
//             let vertex_direction_0 = Vector3 {
//                 value: normalize(vertex_pos_0.value),
//             };
//             let vertex_direction_1 = Vector3 {
//                 value: normalize(vertex_pos_1.value),
//             };
//             let vertex_direction_2 = Vector3 {
//                 value: normalize(vertex_pos_2.value),
//             };
//             let vertex_direction_3 = Vector3 {
//                 value: normalize(vertex_pos_3.value),
//             };
//
//             let tangent_0 = Vector3::set(-vertex_direction_0[2], 0.0,
// vertex_direction_0[0]);             let tangent_1 =
// Vector3::set(-vertex_direction_1[2], 0.0, vertex_direction_1[0]);            
// let tangent_2 = Vector3::set(-vertex_direction_2[2], 0.0,
// vertex_direction_2[0]);             let tangent_3 =
// Vector3::set(-vertex_direction_3[2], 0.0, vertex_direction_3[0]);
//
//             let normal_0 = Vector3 {
//                 value: normalize(cross(slant_height_0.value,
// tangent_0.value)),             };
//
//             let normal_1 = Vector3 {
//                 value: normalize(cross(slant_height_1.value,
// tangent_1.value)),             };
//
//             let normal_2 = Vector3 {
//                 value: normalize(cross(slant_height_2.value,
// tangent_2.value)),             };
//
//             let normal_3 = Vector3 {
//                 value: normalize(cross(slant_height_3.value,
// tangent_3.value)),             };
//
//             let d = Vertex {
//                 tangent: [0.0; 4],
//                 bi_tangent: [0.0; 4],
//                 position: vertex_pos_3.to_primitive(),
//                 normal: normal_3.to_primitive(),
//                 tex_coord: [side_vec4[3] * rcp_tesselation_slice, 0.0],
//             };
//
//             let a = Vertex {
//                 tangent: [0.0; 4],
//                 bi_tangent: [0.0; 4],
//                 position: vertex_pos_0.to_primitive(),
//                 normal: normal_0.to_primitive(),
//                 tex_coord: [side_vec4[0] * rcp_tesselation_slice, 0.0],
//             };
//
//             let b = Vertex {
//                 tangent: [0.0; 4],
//                 bi_tangent: [0.0; 4],
//                 position: vertex_pos_1.to_primitive(),
//                 normal: normal_1.to_primitive(),
//                 tex_coord: [side_vec4[1] * rcp_tesselation_slice, 0.0],
//             };
//             let c = Vertex {
//                 tangent: [0.0; 4],
//                 bi_tangent: [0.0; 4],
//                 position: vertex_pos_2.to_primitive(),
//                 normal: normal_2.to_primitive(),
//                 tex_coord: [side_vec4[2] * rcp_tesselation_slice, 0.0],
//             };
//
//
//             vertices_chunk.copy_from_slice(&[a, b, c, d]);
//         });
//
//
//         // indices
//         for face_index in 2..TESS + 2 {
//             let face_indices_u32 = face_index as u32;
//
//             let face_indices = [
//                 0,
//                 face_indices_u32 + 1,
//                 face_indices_u32,
//                 1,
//                 face_indices_u32,
//                 face_indices_u32 + 1,
//             ];
//
//             let start_index = (face_index - 2) * 6;
//             let end_index = (face_index - 2) * 6 + 6;
//             indices[start_index..end_index].copy_from_slice(&face_indices);
//         }
//
//         Mesh {
//             vertices: vertices.to_vec(),
//             indices: U32(indices.to_smallvec()),
//         }
//     }
// }
//
// #[cfg(test)]
// mod test {
//     use crate::mesh::primitive::cone::Cone;
//     use crate::mesh::Mesh;
//
//     #[test]
//     fn test() {
//         let cone: Cone<64> = Cone::new(1.0, 2., [0.0, 1.0, 0.0]);
//         let cone_model: Mesh = cone.into();
//         for vertex in &cone_model.vertices {
//             println!(
//                 "new Vector3({}f, {}f, {}f),",
//                 vertex.normal[0], vertex.normal[1], vertex.normal[2]
//             );
//         }
//         println!("{:?}", cone_model.indices);
//     }
// }
