use fabled_render::material::MaterialParameter;
use fabled_render::texture::Texture;

use std::ops::DerefMut;

use crate::{MaterialMetadata, ObjError, TextureTarget};
use rayon::prelude::*;

const STANDARD_MATERIAL_COUNT: usize = 8;

#[derive(Default)]
pub struct MtlLoader;

impl MtlLoader {
    pub fn load<P: AsRef<std::path::Path>>(
        &self,
        mtl_path: P,
        target: TextureTarget,
        chunk_size: usize,
    ) -> Result<MaterialMetadata, ObjError> {
        let parent_dir = mtl_path.as_ref().parent().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Parent path is not a valid directory",
            )
        })?;

        let file = std::fs::File::open(mtl_path.as_ref())?;

        let mut mtl_file_buffer = std::io::BufReader::new(file);

        let (mtl_detail, _) =
            tobj::load_mtl_buf(&mut mtl_file_buffer).map_err(ObjError::ObjError)?;

        let chunk_size = chunk_size.min(mtl_detail.len());

        let material_chunk = mtl_detail.par_chunks_exact(chunk_size);

        let chunk_remainder = material_chunk.remainder();

        let mtl_len = chunk_size * STANDARD_MATERIAL_COUNT + chunk_size;

        let mtl_collection = std::sync::Arc::new(parking_lot::Mutex::new(MaterialMetadata {
            materials: Vec::with_capacity(mtl_len),
        }));

        material_chunk.into_par_iter().for_each(|mtl_chunk| {
            let mtl_collection = mtl_collection.clone();

            for mtl in mtl_chunk {
                let mut mtl = Self::calculate_mtl_internal(parent_dir, mtl);
                let mut mtl_guard = mtl_collection.lock();

                mtl_guard.materials.append(&mut mtl);
            }
        });

        for remaining_mtl in chunk_remainder {
            let mut mtl = Self::calculate_mtl_internal(parent_dir, remaining_mtl);
            let mut mtl_guard = mtl_collection.lock();
            mtl_guard.materials.append(&mut mtl);
        }

        let mut material_guard = mtl_collection.lock();

        let materials = std::mem::take(material_guard.deref_mut());

        Ok(materials)
    }

    fn calculate_mtl_internal(
        parent_dir: &std::path::Path,
        mtl: &tobj::Material,
    ) -> Vec<(Texture<'static>, MaterialParameter)> {
        let (tex_dir, mtl_param) = Self::retrieve_standard_materials(mtl);

        let len = std::cmp::min(tex_dir.len(), mtl_param.len());

        let mut mtl_container = Vec::with_capacity(len);

        let (mut textures, mut params) = (&tex_dir[..len], &mtl_param[..len]);

        while textures.len() >= 4 {
            for index in 0..4 {
                let full_tex_dir = textures[index].map(|tex_path| {
                    let full_tex_dir = parent_dir.join(tex_path).to_str().unwrap().to_string();
                    std::borrow::Cow::from(full_tex_dir)
                });

                let target_texture = Texture {
                    texture: full_tex_dir,
                    texture_option: Default::default(),
                    texture_blending: Default::default(),
                };

                let target_param = params[index];


                mtl_container.push((target_texture, target_param));
            }

            textures = &textures[4..];
            params = &params[4..];
        }

        for (texture, param) in textures.iter().zip(params) {
            let full_tex_dir = texture.map(|tex_path| {
                let full_tex_dir = parent_dir.join(tex_path).to_str().unwrap().to_string();
                std::borrow::Cow::from(full_tex_dir)
            });

            let texture = Texture {
                texture: full_tex_dir,
                texture_option: Default::default(),
                texture_blending: Default::default(),
            };

            mtl_container.push((texture, *param));
        }

        mtl_container
    }

    // todo can I simplify this? and clean up the code (2 array) this will be a part
    // of pbr as well.
    fn retrieve_standard_materials(
        material: &tobj::Material,
    ) -> (Vec<Option<&String>>, Vec<MaterialParameter>) {
        let texture_maps = vec![
            Some(&material.diffuse_texture),
            Some(&material.ambient_texture),
            Some(&material.specular_texture),
            Some(&material.normal_texture),
            Some(&material.shininess_texture),
            Some(&material.dissolve_texture),
            None,
            None,
        ];

        let texture_params = vec![
            MaterialParameter::Color(material.diffuse),
            MaterialParameter::Color(material.ambient),
            MaterialParameter::Color(material.specular),
            MaterialParameter::None,
            MaterialParameter::Scalar(material.shininess),
            MaterialParameter::Scalar(material.dissolve),
            MaterialParameter::Scalar(material.optical_density),
            MaterialParameter::Scalar(material.illumination_model.unwrap_or_default() as f32),
        ];

        (texture_maps, texture_params)
    }
}


#[cfg(test)]
mod mtl_loader_test {
    use crate::{MtlLoader, TextureTarget};

    #[test]
    fn load_mtl() {
        let mtl_loader = MtlLoader::default();

        let materials = mtl_loader
            .load(
                "D://Study//Fabled Engine//example//just_a_girl//untitled.mtl",
                TextureTarget::Standard,
                3,
            )
            .unwrap();

        println!("{:#?}", materials.materials);
    }
}
