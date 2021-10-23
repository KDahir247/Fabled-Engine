use fabled_render::material::StandardMaterial;
use rayon::prelude::*;

const UNKNOWN_PARAM_SUPPORT: [&str; 4] = ["map_Ns", "disp", "decal", "refl"];

#[derive(Default)]
pub struct MtlLoader;

impl MtlLoader {
    pub fn load<P: AsRef<std::path::Path>>(&self, mtl_path: P) {
        let file = std::fs::File::open(mtl_path).unwrap();

        let mut mtl_file_buffer = std::io::BufReader::new(file);

        let load_result = tobj::load_mtl_buf(&mut mtl_file_buffer);

        let mtl_detail = load_result.unwrap();

        let obj_mtl = mtl_detail.0;

        let a = obj_mtl
            .par_iter()
            .map(|material: &tobj::Material| {
                // material.unknown_param.


                println!("{:?}", material.name);
                material.unknown_param.get("");

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
            .collect::<Vec<_>>();

        println!("{:#?}", a);
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
