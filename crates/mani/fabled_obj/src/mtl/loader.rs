use fabled_render::material::MaterialParameter;
use fabled_render::texture::Texture;

use std::ops::DerefMut;

use crate::{MaterialMetadata, ObjError};
use rayon::prelude::*;

#[derive(Default)]
pub struct MtlLoader;

impl MtlLoader {
    pub fn load<P: AsRef<std::path::Path>>(
        &self,
        mtl_path: P,
    ) -> Result<MaterialMetadata, ObjError> {
        let file = std::fs::File::open(mtl_path).unwrap();

        let mut mtl_file_buffer = std::io::BufReader::new(file);

        let load_result = tobj::load_mtl_buf(&mut mtl_file_buffer);

        let mtl_detail = load_result.unwrap();

        let obj_mtl = mtl_detail.0;

        let mtl_collection = std::sync::Arc::new(parking_lot::Mutex::new(MaterialMetadata {
            materials: Vec::new(),
        }));

        let material_collection = mtl_collection.clone();

        obj_mtl.par_iter().for_each(|material: &tobj::Material| {
            // todo we need a clear separation between pbr material and standard material.

            // todo need to find a way to store strings to hold the texture path.

            let mut mtl_guard = material_collection.lock();

            let texture_maps = [
                Some(&material.diffuse_texture),
                Some(&material.ambient_texture),
                Some(&material.specular_texture),
                Some(&material.normal_texture),
                Some(&material.shininess_texture),
                Some(&material.dissolve_texture),
                None,
                None,
            ];

            let texture_params = [
                MaterialParameter::Color(material.diffuse),
                MaterialParameter::Color(material.ambient),
                MaterialParameter::Color(material.specular),
                MaterialParameter::None,
                MaterialParameter::Scalar(material.shininess),
                MaterialParameter::Scalar(material.dissolve),
                MaterialParameter::Scalar(material.optical_density),
                MaterialParameter::Scalar(material.illumination_model.unwrap_or_default() as f32),
            ];


            for (&texture_dir, factor) in texture_maps.iter().zip(texture_params) {
                let texture_dir = texture_dir.map(|str| std::borrow::Cow::from(str.to_owned()));

                let texture = Texture {
                    texture: texture_dir,
                    texture_option: Default::default(),
                    texture_blending: Default::default(),
                };

                println!("{:?} {:?}", texture.texture, factor);

                mtl_guard.materials.push((texture, factor));
            }
            println!("\n");
        });

        let mut material_guard = mtl_collection.lock();

        let materials = std::mem::take(material_guard.deref_mut());

        Ok(materials)
    }
}


#[cfg(test)]
mod mtl_loader_test {
    use crate::MtlLoader;

    #[test]
    fn load_mtl() {
        let mtl = MtlLoader::default();
        let materials = mtl
            .load("D:/Study//Fabled Engine/example/just_a_girl/untitled.mtl")
            .unwrap();
    }
}
