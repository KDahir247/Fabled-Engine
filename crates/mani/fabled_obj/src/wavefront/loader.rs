use crate::{LoadFlags, ModelMetadata, ObjError};

use fabled_render::mesh::{Indices, Mesh, Model, Vertex};

use rayon::prelude::*;

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
        let obj_path = obj_path.as_ref();

        let file = std::fs::File::open(obj_path)?;

        let mut obj_file_buffer = std::io::BufReader::new(file);

        let load_flags = LoadFlags::from_bits(self.flags).unwrap_or_default();

        let (models, _) =
            tobj::load_obj_buf(&mut obj_file_buffer, &load_flags.into(), |_mtl_path| {
                Err(tobj::LoadError::GenericFailure)
            })
            .map_err(ObjError::ObjError)?;

        let chunk_size = chunk_size.min(models.len());

        let chunk_model_data = models.chunks_exact(chunk_size);

        let chunk_model_size = chunk_model_data.len();

        let chunk_remainder = chunk_model_data.remainder();

        let mesh_len =
            chunk_model_size * chunk_size + chunk_remainder.len() + (chunk_model_size >> 1);

        let mut meshes = Vec::with_capacity(mesh_len);

        for model_chunk in chunk_model_data {
            for model in model_chunk {
                let mesh = self.calculate_obj_internal(model);

                meshes.push(mesh);
            }
        }

        for remaining_model in chunk_remainder {
            let mesh = self.calculate_obj_internal(remaining_model);

            meshes.push(mesh);
        }

        Ok(ModelMetadata {
            mtl_path: std::path::PathBuf::from(obj_path.parent().unwrap()),
            model: Model { meshes },
        })
    }

    fn calculate_obj_internal(&self, model: &tobj::Model) -> Mesh {
        let vertices = (0..model.mesh.positions.len() / 3)
            .into_par_iter()
            .map(|index| {
                let vertex = [
                    model.mesh.positions[index * 3],
                    model.mesh.positions[index * 3 + 1],
                    model.mesh.positions[index * 3 + 2],
                ];

                let normal: &[f32] = model
                    .mesh
                    .normals
                    .get((index * 3)..(index * 3 + 3))
                    .unwrap_or(&[0.0; 3]);

                let tex_coord: &[f32] = model
                    .mesh
                    .texcoords
                    .get((index * 2)..(index * 2 + 2))
                    .unwrap_or(&[0.0; 3]);

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
    use crate::common::obj_common::load_test_obj;
    use crate::ObjLoader;

    #[test]
    fn construction() {
        let obj = ObjLoader::default();

        let mut obj_dir = load_test_obj("obj");

        if let Some(path) = obj_dir.pop() {
            let model = obj.load(path, 3);
            assert!(model.is_ok());
        } else {
            panic!("Could not find obj file in obj/test");
        }
    }

    #[test]
    fn load_obj() {
        let obj = ObjLoader::default();

        let mut obj_dir = load_test_obj("obj");

        if let Some(path) = obj_dir.pop() {
            let girl = obj.load(path.clone(), 3).unwrap();

            let file = std::fs::File::open(path).unwrap();

            let mut obj_file_buffer = std::io::BufReader::new(file);


            let (models, _) = tobj::load_obj_buf(
                &mut obj_file_buffer,
                &tobj::LoadOptions::default(),
                |_mtl_path| Err(tobj::LoadError::GenericFailure),
            )
            .unwrap();

            assert_eq!(models.len(), girl.model.meshes.len());
        } else {
            panic!("Could not find obj file in obj/test");
        }
    }
}
