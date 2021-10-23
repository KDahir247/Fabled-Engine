use crate::{LoadFlags, ObjError};

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

    pub fn load<P: AsRef<std::path::Path>>(
        &self,
        obj_path: P,
        chunk_size: usize,
    ) -> Result<Model, ObjError> {
        let file = std::fs::File::open(obj_path.as_ref())?;

        let mut obj_file_buffer = std::io::BufReader::new(file);

        let load_flags = LoadFlags::from_bits(self.flags).unwrap_or_default();

        let obj_detail =
            tobj::load_obj_buf(&mut obj_file_buffer, &load_flags.into(), |_mtl_path| {
                Err(tobj::LoadError::GenericFailure)
            })
            .map_err(ObjError::ObjError)?;

        let models = obj_detail.0;

        let chunk_size = chunk_size.min(models.len());

        let chunk_model_data = models.par_chunks_exact(chunk_size);

        let chunk_remainder = chunk_model_data.remainder();

        let mesh_len = chunk_model_data.len() * chunk_size + chunk_remainder.len();

        let meshes = std::sync::Arc::new(parking_lot::Mutex::new(Vec::with_capacity(mesh_len)));

        chunk_model_data.into_par_iter().for_each(|model_chunk| {
            for model in model_chunk {
                let mesh = self.calculate_obj_internal(model);

                let mut mesh_guard = meshes.lock();

                mesh_guard.push(mesh);
            }
        });

        for remaining_model in chunk_remainder {
            let mesh = self.calculate_obj_internal(remaining_model);

            let mut mesh_guard = meshes.lock();

            mesh_guard.push(mesh);
        }

        let mut mesh_guard = meshes.lock();

        let meshes = std::mem::take(mesh_guard.deref_mut());

        Ok(Model { meshes })
    }

    fn calculate_obj_internal(&self, model: &tobj::Model) -> Mesh {
        let vertices = (0..model.mesh.positions.len() / 3)
            .into_iter()
            .map(|index| {
                let vertex = [
                    model.mesh.positions[index * 3],
                    model.mesh.positions[index * 3 + 1],
                    model.mesh.positions[index * 3 + 2],
                ];

                let normal = if model.mesh.normals.is_empty() {
                    [0.0, 0.0, 0.0]
                } else {
                    [
                        model.mesh.normals[index * 3],
                        model.mesh.normals[index * 3 + 1],
                        model.mesh.normals[index * 3 + 2],
                    ]
                };

                let tex_coord = if model.mesh.texcoords.is_empty() {
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
            .collect::<Vec<Vertex>>();

        let indices = model.mesh.indices.to_owned();

        let material_id = model.mesh.material_id.unwrap_or_default() as u32;

        Mesh {
            vertices,
            indices: Indices::U32(indices),
            material_id,
        }
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
        )
        .unwrap();
    }
}
