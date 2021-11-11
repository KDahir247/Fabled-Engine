use fabled_render::material::MaterialParameter;
use fabled_render::texture::Texture;

use std::ops::DerefMut;

use crate::{
    parse_float_n, IlluminationModel, Material, MaterialMetadata, ObjError, TextureTarget,
    UNKNOWN_PARAM_PBR_SUPPORT, UNKNOWN_PARAM_SUPPORT,
};
use rayon::prelude::*;

pub struct MtlLoader {
    target: TextureTarget,
}

impl Default for MtlLoader {
    fn default() -> Self {
        Self {
            target: TextureTarget::Standard,
        }
    }
}

impl MtlLoader {
    pub fn new(target: TextureTarget) -> Self {
        Self { target }
    }


    pub fn load<P: AsRef<std::path::Path>>(
        &self,
        mtl_path: P,
        chunk_size: usize,
    ) -> Result<Vec<MaterialMetadata>, ObjError> {
        let mtl_path = mtl_path.as_ref();

        let parent_dir = mtl_path.parent().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Parent path is not a valid directory",
            )
        })?;

        let file = std::fs::File::open(mtl_path)?;

        let mut mtl_file_buffer = std::io::BufReader::new(file);

        let (mtl_detail, _) =
            tobj::load_mtl_buf(&mut mtl_file_buffer).map_err(ObjError::ObjError)?;

        let mtl_size = mtl_detail.len();

        let chunk_size = chunk_size.min(mtl_size);

        let material_chunk = mtl_detail.par_chunks_exact(chunk_size);

        let chunk_remainder = material_chunk.remainder();

        let len = mtl_size + (mtl_size >> 1);

        let mtl_collection = std::sync::Arc::new(parking_lot::Mutex::new(Vec::with_capacity(len)));

        material_chunk.into_par_iter().for_each(|mtl_chunk| {
            let mtl_collection = mtl_collection.clone();

            for mtl in mtl_chunk {
                let mut mtl_guard = mtl_collection.lock();

                let (illum_model, materials) =
                    Self::calculate_mtl_internal(parent_dir, mtl, self.target);

                mtl_guard.push(MaterialMetadata {
                    illum_model,
                    materials,
                });
            }
        });

        let mut mtl_guard = mtl_collection.lock();

        for remaining_mtl in chunk_remainder {
            let (illum_model, materials) =
                Self::calculate_mtl_internal(parent_dir, remaining_mtl, self.target);

            mtl_guard.push(MaterialMetadata {
                illum_model,
                materials,
            });
        }

        let materials = std::mem::take(mtl_guard.deref_mut());

        Ok(materials)
    }

    fn calculate_mtl_internal<P: AsRef<std::path::Path>>(
        parent_dir: P,
        mtl: &tobj::Material,
        target: TextureTarget,
    ) -> (IlluminationModel, Vec<Material<'static>>) {
        let (mut tex_dir, mut mtl_param) = Self::retrieve_standard_materials(mtl);

        if target.eq(&TextureTarget::PBR) {
            let (mut pbr_tex_dir, mut pbr_mtl_param) = Self::retrieve_pbr_materials(mtl);

            tex_dir.append(&mut pbr_tex_dir);

            mtl_param.append(&mut pbr_mtl_param);
        }

        let len = std::cmp::min(tex_dir.len(), mtl_param.len());

        let len_offset = len >> 1;

        let mut mtl_container = Vec::with_capacity(len + len_offset);

        let (mut tex_chunk, tex_remainder) = tex_dir.split_at(len);

        let (mut param_chunk, param_remainder) = mtl_param.split_at(len);

        while tex_chunk.len() >= len_offset {
            for index in 0..len_offset {
                let texture = tex_chunk[index];

                let mtl_param = param_chunk[index];

                let texture = Self::create_texture(parent_dir.as_ref(), texture);

                mtl_container.push(Material { texture, mtl_param });
            }

            tex_chunk = &tex_chunk[len_offset..];
            param_chunk = &param_chunk[len_offset..];
        }

        for &tex_dir in tex_remainder {
            let texture = Self::create_texture(parent_dir.as_ref(), tex_dir);

            mtl_container.push(Material {
                texture,
                mtl_param: Default::default(),
            });
        }

        for &mtl_param in param_remainder {
            mtl_container.push(Material {
                texture: Default::default(),
                mtl_param,
            });
        }

        (mtl.illumination_model.into(), mtl_container)
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
            MaterialParameter::Scalar(1.0),
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
        let len = UNKNOWN_PARAM_SUPPORT.len() + (UNKNOWN_PARAM_SUPPORT.len() >> 1);

        let mut pbr_texture_maps = Vec::with_capacity(len);

        let mut pbr_texture_params = Vec::with_capacity(len);

        let mut val = Vec::with_capacity(4);

        for pbr_param in UNKNOWN_PARAM_PBR_SUPPORT {
            if let Some(valid_pbr_param) = material.unknown_param.get(pbr_param) {
                let mut words = valid_pbr_param[..].split_whitespace();

                let count = words.clone().count();
                if parse_float_n(&mut words, &mut val, count) {
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
    use crate::common::obj_common::load_test_obj;
    use crate::MtlLoader;

    #[test]
    fn construction() {
        let mtl_loader = MtlLoader::default();

        let mut mtl_dir = load_test_obj("mtl");

        if let Some(path) = mtl_dir.pop() {
            let materials = mtl_loader.load(path, 3);

            assert!(materials.is_ok());
        } else {
            panic!("Could not find material file in  obj/test");
        }
    }

    #[test]
    fn load_mtl() {
        let mtl_loader = MtlLoader::default();

        let mut mtl_dir = load_test_obj("mtl");

        if let Some(path) = mtl_dir.pop() {
            let materials = mtl_loader.load(path, 3).unwrap();

            assert_eq!(materials.len(), 3);

            println!("{:#?}", materials);
        } else {
            panic!("Could not find material file in  obj/test");
        }
    }
}
