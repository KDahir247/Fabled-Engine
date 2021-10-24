use fabled_render::material::{MaterialType, StandardMaterial};
use fabled_render::texture::Texture;

use crate::MaterialMetadata;
use rayon::prelude::*;

#[derive(Default)]
pub struct MtlLoader;

impl MtlLoader {
    pub fn load<P: AsRef<std::path::Path>>(
        &self,
        mtl_path: P,
    ) -> Result<Vec<MaterialMetadata>, std::io::Error> {
        let file = std::fs::File::open(mtl_path).unwrap();

        let mut mtl_file_buffer = std::io::BufReader::new(file);

        let load_result = tobj::load_mtl_buf(&mut mtl_file_buffer);

        let mtl_detail = load_result.unwrap();

        let obj_mtl = mtl_detail.0;

        let material_detail = obj_mtl
            .par_iter()
            .map(|material: &tobj::Material| {
                // todo we need a clear separation between pbr material and standard material.

                // todo need to find a way to store strings to hold the texture path.

                let texture = Texture {
                    texture: std::borrow::Cow::from(material.diffuse_texture.to_owned()),
                    texture_option: Default::default(),
                    texture_blending: Default::default(),
                };

                let material = MaterialType::Standard(StandardMaterial {
                    diffuse_color: material.diffuse,
                    ambient_color: material.ambient,
                    specular_color: material.specular,
                    unknown_param: [0.0; 3],
                    factor: [
                        material.shininess,
                        material.optical_density,
                        material.dissolve,
                    ],
                });

                MaterialMetadata { texture, material }
            })
            .collect::<Vec<MaterialMetadata>>();


        // todo change material detail not to be a collection of type, but have a
        // collection internally that contains the material datas.
        Ok(material_detail)
    }
}


#[cfg(test)]
mod mtl_loader_test {
    use crate::MtlLoader;

    #[test]
    fn load_mtl() {
        let mtl = MtlLoader::default();
        mtl.load("D:/Study//Fabled Engine/example/just_a_girl/untitled.mtl");
    }
}
