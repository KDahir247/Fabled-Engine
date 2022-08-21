use crate::{LoadFlags, ObjError};
use fabled_render::mesh::{Indices, Mesh, Vertex};


use rayon::prelude::*;
// pub struct ObjLoader {
//     flags: u8,
// }
//
// impl Default for ObjLoader {
//     fn default() -> Self {
//         Self {
//             flags: LoadFlags::default().bits(),
//         }
//     }
// }
//
// impl ObjLoader {
//     pub fn new(load_option: LoadFlags) -> Self {
//         Self {
//             flags: load_option.bits(),
//         }
//     }
//
//     pub fn load<P: AsRef<std::path::Path>>(
//         &self,
//         obj_path: P,
//         chunk_size: Option<usize>,
//     ) -> Result<(Model, Vec<tobj::Material>), ObjError> {
//         let obj_path = obj_path.as_ref();
//
//         let file = std::fs::File::open(obj_path)?;
//
//         let mut obj_file_buffer = std::io::BufReader::new(file);
//
//         let load_flags =
// LoadFlags::from_bits(self.flags).unwrap_or_default();
//
//         let (models, mtl) =
//             tobj::load_obj_buf(&mut obj_file_buffer, &load_flags.into(),
// |mtl_path| {                 let full_path = if let Some(parent) =
// obj_path.parent() {                     parent.join(mtl_path)
//                 } else {
//                     mtl_path.to_owned()
//                 };
//
//                 tobj::load_mtl(full_path)
//             })
//             .map_err(ObjError::ObjError)?;
//
//
//         let chunk_size = chunk_size.unwrap_or(4).min(models.len());
//
//         let chunk_model_data = models.chunks_exact(chunk_size);
//
//         let chunk_model_size = chunk_model_data.len();
//
//         let chunk_remainder = chunk_model_data.remainder();
//
//         let mesh_len =
//             chunk_model_size * chunk_size + chunk_remainder.len() +
// (chunk_model_size >> 1);
//
//         let mut meshes = Vec::with_capacity(mesh_len);
//
//         for model_chunk in chunk_model_data {
//             for model in model_chunk {
//                 let mesh = self.calculate_obj_internal(model);
//
//                 meshes.push(mesh);
//             }
//         }
//
//         for remaining_model in chunk_remainder {
//             let mesh = self.calculate_obj_internal(remaining_model);
//
//             meshes.push(mesh);
//         }
//
//
//         let a = mtl.unwrap_or_default();
//
//         Ok((Model { meshes }, a))
//     }
//
//     fn calculate_obj_internal(&self, model: &tobj::Model) -> Mesh {
//         let vertex_len = model.mesh.positions.len() / 3;
//
//         let mut vertices = vec![Vertex::default(); vertex_len];
//
//
//         vertices
//             .par_iter_mut()
//             .enumerate()
//             .for_each(|(index, vertex)| {
//                 let index_3 = index * 3;
//                 let index_2 = index * 2;
//
//                 assert!(model.mesh.positions.len() > index_3 + 1);
//                 assert!(model.mesh.normals.len() > index_3 + 1);
//                 assert!(model.mesh.texcoords.len() > index_2);
//
//                 vertex.position = [
//                     model.mesh.positions[index_3],
//                     model.mesh.positions[index_3 + 1],
//                     model.mesh.positions[index_3 + 2],
//                     0.0,
//                 ];
//
//                 vertex.normal = [
//                     model.mesh.normals[index_3],
//                     model.mesh.normals[index_3 + 1],
//                     model.mesh.normals[index_3 + 2],
//                     0.0,
//                 ];
//
//                 vertex.tex_coord = [
//                     model.mesh.texcoords[index_2],
//                     model.mesh.texcoords[index_2 + 1],
//                 ];
//             });
//
//
//         let indices = model.mesh.indices.to_vec();
//
//         let material_id = model.mesh.material_id.unwrap_or_default();
//
//         Mesh {
//             vertices,
//             indices: Indices::U32(indices),
//             // material_id,
//         }
//     }
// }
//
// #[cfg(test)]
// mod obj_loader_test {
//     use crate::common::obj_common::load_test_obj;
//     use crate::ObjLoader;
//
//     #[test]
//     fn construction() {
//         let obj = ObjLoader::default();
//
//         let mut obj_dir = load_test_obj("obj");
//
//         if let Some(path) = obj_dir.pop() {
//             let model = obj.load(path, Some(5));
//             assert!(model.is_ok());
//         } else {
//             panic!("Could not find obj file in obj/test");
//         }
//     }
//
//     #[test]
//     fn load_obj() {
//         let obj = ObjLoader::default();
//
//         let mut obj_dir = load_test_obj("obj");
//
//         if let Some(path) = obj_dir.pop() {
//             let girl = obj.load(path.clone(), Some(3)).unwrap();
//
//             let file = std::fs::File::open(path).unwrap();
//
//             let mut obj_file_buffer = std::io::BufReader::new(file);
//
//
//             let (models, _) = tobj::load_obj_buf(
//                 &mut obj_file_buffer,
//                 &tobj::LoadOptions::default(),
//                 |_mtl_path| Err(tobj::LoadError::GenericFailure),
//             )
//             .unwrap();
//
//             assert_eq!(models.len(), girl.0.meshes.len());
//         } else {
//             panic!("Could not find obj file in obj/test");
//         }
//     }
// }
