use crate::LoadFlags;
use fabled_render::mesh::{Mesh, Model, Vertex};
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

    pub fn load<P: AsRef<std::path::Path>>(&self, obj_path: P, chunk_size: usize) {
        let file = std::fs::File::open(obj_path).unwrap();
        let file_metadata = file.metadata().unwrap();

        let mut obj_file_buffer =
            std::io::BufReader::with_capacity(file_metadata.len() as usize, file);

        let load_flags = LoadFlags::from_bits(self.flags).unwrap_or_default();

        let load_result =
            tobj::load_obj_buf(&mut obj_file_buffer, &load_flags.into(), |mtl_path| {
                // We don't want to load mtl from this script.
                println!("{}", mtl_path.display());
                Err(tobj::LoadError::GenericFailure)
            });

        let obj_detail = load_result.unwrap();

        let chunk_model_data = obj_detail.0.par_chunks_exact(chunk_size);
        let chunk_remainder = chunk_model_data.remainder();

        chunk_model_data.into_par_iter().for_each(|model_chunk| {
            assert!(model_chunk.len().eq(&chunk_size));

            let mut end_base_collection = Vec::with_capacity(chunk_size);

            for chunk_index in 0..chunk_size {
                end_base_collection.push(model_chunk[chunk_index].mesh.positions.len() / 3);
            }

            let mut end_bases = &end_base_collection[..chunk_size];
            let mut chunk_index = 0;

            while !end_bases.is_empty() {
                let range_bases = 0..end_bases[0];

                let vertex: Vec<Vertex> = range_bases
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
                            tangent: [0.0; 4],
                            bi_tangent: [0.0; 4],
                            position: vertex,
                            tex_coord,
                            normal,
                        }
                    })
                    .collect();


                println!("chunk calculated");
                end_bases = &end_bases[1..];
                chunk_index += 1;
            }

            println!("chunks finished calculating");
        });


        // chunk remainder here.
        for remaining_model in chunk_remainder {
            println!("remain data that need to be calculated");
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
            2,
        );
    }
}
