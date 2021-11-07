use crate::{LoadFlags, ModelMetadata, ObjError};

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
    ) -> Result<ModelMetadata, ObjError> {
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
            let meshes = meshes.clone();

            for model in model_chunk {
                let mesh = self.calculate_obj_internal(model);

                let mut mesh_guard = meshes.lock();

                mesh_guard.push(mesh);
            }
        });

        let mut mesh_guard = meshes.lock();

        for remaining_model in chunk_remainder {
            let mesh = self.calculate_obj_internal(remaining_model);

            mesh_guard.push(mesh);
        }

        let meshes = std::mem::take(mesh_guard.deref_mut());

        Ok(ModelMetadata {
            mtl_path: std::path::PathBuf::from(obj_path.as_ref().parent().unwrap()),
            model: Model { meshes },
        })
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

                let normal = model
                    .mesh
                    .normals
                    .get((index * 3)..(index * 3 + 3))
                    .unwrap_or(&[0.0; 3]);


                let tex_coord = model
                    .mesh
                    .texcoords
                    .get((index * 2)..(index * 2 + 2))
                    .unwrap_or(&[0.0; 2]);

                Vertex {
                    position: vertex,
                    tex_coord: [tex_coord[0], tex_coord[1]],
                    normal: [normal[0], normal[1], normal[2]],
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
        let model = obj
            .load(
                "D:/Study//Fabled Engine/example/just_a_girl/untitled.obj",
                3,
            )
            .unwrap();

        println!("{}", model.model.meshes.len());
    }
}
