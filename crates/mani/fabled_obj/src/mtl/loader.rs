use fabled_render::material::StandardMaterial;
use rayon::prelude::*;

const UNKNOWN_PARAM_SUPPORT: [&str; 3] = ["disp", "decal", "refl"];

#[derive(Default)]
pub struct MtlLoader;

impl MtlLoader {
    pub fn load<P: AsRef<std::path::Path>>(&self, mtl_path: P) {
        let file = std::fs::File::open(mtl_path).unwrap();

        let mut mtl_file_buffer = std::io::BufReader::new(file);

        let load_result = tobj::load_mtl_buf(&mut mtl_file_buffer);

        let mtl_detail = load_result.unwrap();

        let obj_mtl = mtl_detail.0;

        let model_materials = obj_mtl
            .par_iter()
            .map(|material: &tobj::Material| {
                for a in UNKNOWN_PARAM_SUPPORT {
                    let result = material.unknown_param.get(a);

                    if let Some(result) = result {
                        println!("{}", result);
                    }
                }

                StandardMaterial {
                    ambient_color: material.ambient,
                    diffuse_color: material.diffuse,
                    specular_color: material.specular,
                    factor: [
                        material.shininess,
                        material.optical_density,
                        material.dissolve,
                        material.illumination_model.unwrap_or_default() as f32,
                    ],
                }
            })
            .collect::<Vec<StandardMaterial>>();

        println!("{:#?}", model_materials);
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
