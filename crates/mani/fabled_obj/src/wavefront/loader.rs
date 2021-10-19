use crate::LoadFlags;

use fabled_render::mesh::{Indices, Mesh, Model, Vertex};

use rayon::prelude::*;
use std::ops::DerefMut;

pub struct ObjLoader {
    flags: u8,
}

impl Default for ObjLoader {
    fn default() -> Self {
        Self {
            flags: LoadFlags::default().bits(),
        }
    }
}

impl ObjLoader {
    pub fn new(load_option: LoadFlags) -> Self {
        Self {
            flags: load_option.bits(),
        }
    }

    pub fn load<P: AsRef<std::path::Path>>(&self, obj_path: P, chunk_size: usize) -> Model {
        let file = std::fs::File::open(obj_path.as_ref()).unwrap();
        let file_metadata = file.metadata().unwrap();

        let mut obj_file_buffer =
            std::io::BufReader::with_capacity(file_metadata.len() as usize, file);

        let load_flags = LoadFlags::from_bits(self.flags).unwrap_or_default();

        let load_result =
            tobj::load_obj_buf(&mut obj_file_buffer, &load_flags.into(), |mtl_path| {
                let full_path = if let Some(parent) = obj_path.as_ref().parent() {
                    parent.join(mtl_path)
                } else {
                    mtl_path.to_owned()
                };

                // We don't want to load mtl from this script.
                println!("{:?}", full_path.display());

                Err(tobj::LoadError::GenericFailure)
            });

        let obj_detail = load_result.unwrap();

        let chunk_model_data = obj_detail.0.par_chunks_exact(chunk_size);

        let chunk_remainder = chunk_model_data.remainder();

        let mesh_len = chunk_model_data.len() * chunk_size + chunk_remainder.len();

        let meshes = std::sync::Arc::new(parking_lot::Mutex::new(Vec::with_capacity(mesh_len)));

        chunk_model_data.into_par_iter().for_each(|model_chunk| {
            assert!(model_chunk.len().eq(&chunk_size));

            let mut end_base_collection = Vec::with_capacity(chunk_size);

            for model in model_chunk {
                end_base_collection.push(model.mesh.positions.len() / 3);
            }

            let mut end_bases = &end_base_collection[..chunk_size];

            let mut chunk_index = 0;

            while !end_bases.is_empty() {
                let range_bases = 0..end_bases[0];

                let vertices: Vec<Vertex> = range_bases
                    .into_iter()
                    .map(|index| {
                        let model = &model_chunk[chunk_index];

                        let vertex: [f32; 3] = [
                            model.mesh.positions[index * 3],
                            model.mesh.positions[index * 3 + 1],
                            model.mesh.positions[index * 3 + 2],
                        ];

                        let normal: [f32; 3] = if model.mesh.normals.is_empty() {
                            [0.0, 0.0, 0.0]
                        } else {
                            [
                                model.mesh.normals[index * 3],
                                model.mesh.normals[index * 3 + 1],
                                model.mesh.normals[index * 3 + 2],
                            ]
                        };

                        let tex_coord: [f32; 2] = if model.mesh.texcoords.is_empty() {
                            [0.0, 0.0]
                        } else {
                            [
                                model.mesh.texcoords[index * 2],
                                model.mesh.texcoords[index * 2 + 1],
                            ]
                        };

                        Vertex {
                            position: vertex,
                            tex_coord,
                            normal,
                            ..Default::default()
                        }
                    })
                    .collect();


                let material_id = model_chunk[chunk_index]
                    .mesh
                    .material_id
                    .unwrap_or_default() as u32;

                let indices = model_chunk[chunk_index].mesh.indices.to_owned();

                let mesh = Mesh {
                    vertices,
                    indices: Indices::U32(indices),
                    material_id,
                };

                let mut mesh_guard = meshes.lock();

                mesh_guard.push(mesh);

                println!("chunk calculated");

                end_bases = &end_bases[1..];

                chunk_index += 1;
            }
        });

        // remaining_chunk calculated in the main thread.
        for (index, remaining_model) in chunk_remainder.iter().enumerate() {
            let vertex = [
                remaining_model.mesh.positions[index * 3],
                remaining_model.mesh.positions[index * 3 + 1],
                remaining_model.mesh.positions[index * 3 + 2],
            ];

            let normal = if remaining_model.mesh.normals.is_empty() {
                [0.0, 0.0, 0.0]
            } else {
                [
                    remaining_model.mesh.normals[index * 3],
                    remaining_model.mesh.normals[index * 3 + 1],
                    remaining_model.mesh.normals[index * 3 + 2],
                ]
            };

            let tex_coord = if remaining_model.mesh.texcoords.is_empty() {
                [0.0, 0.0]
            } else {
                [
                    remaining_model.mesh.texcoords[index * 2],
                    remaining_model.mesh.texcoords[index * 2 + 1],
                ]
            };

            let material_id = remaining_model.mesh.material_id.unwrap_or_default() as u32;

            let remaining_vertex = Vertex {
                position: vertex,
                tex_coord,
                normal,
                ..Default::default()
            };

            let remaining_indices = remaining_model.mesh.indices.to_owned();

            let remaining_mesh = Mesh {
                vertices: vec![remaining_vertex],
                indices: Indices::U32(remaining_indices),
                material_id,
            };

            let mut mesh_guard = meshes.lock();

            println!("remaining chunk calculated");

            mesh_guard.push(remaining_mesh);
        }

        let mut mesh_guard = meshes.lock();

        let meshes = std::mem::take(mesh_guard.deref_mut());

        Model { meshes }
    }
}


#[cfg(test)]
mod obj_loader_test {
    use crate::ObjLoader;

    #[test]
    fn load_obj() {
        let obj = ObjLoader::default();
        obj.load(
            "D:/Study//Fabled Engine/example/just_a_girl/untitled.obj",
            3,
        );
    }
}
