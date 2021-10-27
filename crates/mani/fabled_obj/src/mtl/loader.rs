use fabled_render::material::MaterialParameter;
use fabled_render::texture::Texture;

use std::ops::DerefMut;

use crate::{MaterialMetadata, ObjError, TextureTarget, UNKNOWN_PARAM_PBR_SUPPORT};
use rayon::prelude::*;

#[derive(Default)]
pub struct MtlLoader;

impl MtlLoader {
    pub fn load<P: AsRef<std::path::Path>>(
        &self,
        mtl_path: P,
        target: TextureTarget,
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

        // todo remove magic numbers
        let mtl_collection = std::sync::Arc::new(parking_lot::Mutex::new(MaterialMetadata {
            materials: Vec::with_capacity(mtl_detail.len() * 8 + 3),
        }));

        let material_collection = mtl_collection.clone();

        mtl_detail.par_iter().for_each(|material: &tobj::Material| {
            // todo we need a clear separation between pbr material and standard material.

            let (texture_maps, texture_params) = Self::retrieve_standard_materials(material);

            for (&texture_dir, factor) in texture_maps.iter().zip(texture_params) {
                // todo if the map is a empty string or None.
                let full_texture_dir = if let Some(tex_dir) = texture_dir {
                    let str_dir = parent_dir.join(tex_dir).to_str().unwrap().to_string();

                    let result = Some(str_dir);

                    result.map(std::borrow::Cow::from)
                } else {
                    None
                };

                let texture = Texture {
                    texture: full_texture_dir,
                    texture_option: Default::default(),
                    texture_blending: Default::default(),
                };

                let mut mtl_guard = material_collection.lock();

                mtl_guard.materials.push((texture, factor));
            }
        });

        let mut material_guard = mtl_collection.lock();

        let materials = std::mem::take(material_guard.deref_mut());

        Ok(materials)
    }

    // todo can I simplify this? and clean up the code (2 array) this will be a part
    // of pbr as well.
    fn retrieve_standard_materials(
        material: &tobj::Material,
    ) -> ([Option<&String>; 8], [MaterialParameter; 8]) {
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

        (texture_maps, texture_params)
    }

    fn retrieve_pbr_materials(material: &tobj::Material) {
        for pbr_support in UNKNOWN_PARAM_PBR_SUPPORT {
            let pbr_param = material.unknown_param.get(pbr_support);

            if let Some(pbr_param) = pbr_param {}
        }
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
                "D:/Study//Fabled Engine/example/just_a_girl/untitled.mtl",
                TextureTarget::Standard,
            )
            .unwrap();

        println!("{:#?}", materials.materials);
    }
}
