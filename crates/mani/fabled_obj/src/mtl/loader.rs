use fabled_render::material::MaterialParameter;
use fabled_render::texture::Texture;

use std::ops::DerefMut;

use crate::{
    parse_floatn, MaterialMetadata, ObjError, TextureTarget, UNKNOWN_PARAM_PBR_SUPPORT,
    UNKNOWN_PARAM_SUPPORT,
};
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
                let mut mtl = Self::calculate_mtl_internal(parent_dir, mtl, target);

                let mut mtl_guard = mtl_collection.lock();

                mtl_guard.materials.append(&mut mtl);
            }
        });

        let mut mtl_guard = mtl_collection.lock();

        for remaining_mtl in chunk_remainder {
            let mut mtl = Self::calculate_mtl_internal(parent_dir, remaining_mtl, target);

            mtl_guard.materials.append(&mut mtl);
        }

        let materials = std::mem::take(mtl_guard.deref_mut());

        Ok(materials)
    }

    fn calculate_mtl_internal(
        parent_dir: &std::path::Path,
        mtl: &tobj::Material,
        target: TextureTarget,
    ) -> Vec<(Texture<'static>, MaterialParameter)> {
        let (mut tex_dir, mut mtl_param) = Self::retrieve_standard_materials(mtl);

        if target == TextureTarget::PBR {
            let (mut pbr_tex_dir, mut pbr_mtl_param) = Self::retrieve_pbr_materials(mtl);

            tex_dir.append(&mut pbr_tex_dir);

            mtl_param.append(&mut pbr_mtl_param);
        }

        let len = std::cmp::min(tex_dir.len(), mtl_param.len());

        let mut mtl_container = Vec::with_capacity(len);

        let (mut tex_chunk, tex_remainder) = tex_dir.split_at(len);

        let (mut param_chunk, param_remainder) = mtl_param.split_at(len);

        while tex_chunk.len() >= 4 {
            for index in 0..4 {
                let texture = tex_chunk[index];

                let param = param_chunk[index];

                let texture = Self::create_texture(parent_dir, texture);

                mtl_container.push((texture, param));
            }

            tex_chunk = &tex_chunk[4..];
            param_chunk = &param_chunk[4..];
        }

        for (&texture, param) in tex_chunk.iter().zip(param_chunk) {
            let texture = Self::create_texture(parent_dir, texture);

            mtl_container.push((texture, *param));
        }

        for &tex_dir in tex_remainder {
            let texture = Self::create_texture(parent_dir, tex_dir);

            mtl_container.push((texture, MaterialParameter::None));
        }

        for &mtl_param in param_remainder {
            mtl_container.push((Texture::default(), mtl_param));
        }

        mtl_container
    }

    fn create_texture<'a, P: AsRef<std::path::Path>>(parent_dir: P, tex_dir: &str) -> Texture<'a> {
        let parent = parent_dir.as_ref();
        let valid_dir = (!tex_dir.is_empty()).then(|| {
            let full_tex_dir = parent.join(tex_dir).to_str().unwrap().to_string();

            std::borrow::Cow::from(full_tex_dir)
        });

        Texture {
            texture: valid_dir,
            texture_option: Default::default(),
            texture_blending: Default::default(),
        }
    }

    fn retrieve_standard_materials(
        material: &tobj::Material,
    ) -> (Vec<&String>, Vec<MaterialParameter>) {
        let mut texture_maps = vec![
            &material.diffuse_texture,
            &material.ambient_texture,
            &material.specular_texture,
            &material.normal_texture,
            &material.shininess_texture,
            &material.dissolve_texture,
        ];

        let mut texture_params = vec![
            MaterialParameter::Color(material.diffuse),
            MaterialParameter::Color(material.ambient),
            MaterialParameter::Color(material.specular),
            MaterialParameter::Scalar(1.0), // normal factor
            MaterialParameter::Scalar(material.shininess),
            MaterialParameter::Scalar(material.dissolve),
        ];

        for param in UNKNOWN_PARAM_SUPPORT {
            if let Some(valid_tex_path) = material.unknown_param.get(param) {
                texture_maps.push(valid_tex_path);

                texture_params.push(MaterialParameter::None);
            }
        }

        (texture_maps, texture_params)
    }

    fn retrieve_pbr_materials(material: &tobj::Material) -> (Vec<&String>, Vec<MaterialParameter>) {
        let mut pbr_texture_maps = Vec::new();

        let mut pbr_texture_params = Vec::new();

        let mut val = Vec::new();

        for pbr_param in UNKNOWN_PARAM_PBR_SUPPORT {
            if let Some(valid_pbr_param) = material.unknown_param.get(pbr_param) {
                let mut words = valid_pbr_param[..].split_whitespace();

                let count = words.clone().count();
                if parse_floatn(&mut words, &mut val, count) {
                    match count {
                        1 => pbr_texture_params.push(MaterialParameter::Scalar(val.pop().unwrap())),
                        3 => {
                            let z = val.pop().unwrap();

                            let y = val.pop().unwrap();

                            let x = val.pop().unwrap();

                            pbr_texture_params.push(MaterialParameter::Color([x, y, z]))
                        }
                        _ => {}
                    }
                } else {
                    pbr_texture_maps.push(valid_pbr_param);
                }
            }
        }

        (pbr_texture_maps, pbr_texture_params)
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
                TextureTarget::PBR,
                3,
            )
            .unwrap();

        println!("{:#?}", materials.materials);
    }
}
