mod material_parser;
mod shader_parser;

pub use material_parser::*;
pub use shader_parser::*;

#[cfg(test)]
mod data_alignment_test {
    use crate::shader::parser::*;

    #[test]
    fn data_alignment() {
        let shader_parser = std::mem::size_of::<ShaderParser>();
        assert_eq!(shader_parser & (shader_parser - 1), 0);

        let material_parser = std::mem::size_of::<MaterialParser>();
        println!("material_parser is {} bytes", material_parser);
    }
}

#[cfg(test)]
mod shader_test {
    use crate::init_shader_test_env;
    use crate::shader::parser::*;

    #[test]
    fn wgsl_parse() {
        init_shader_test_env();

        let wgsl_parser = ShaderParser::parse(std::env::var("WGSL_FILE").unwrap()).unwrap();
        assert_eq!(wgsl_parser.module.functions.len(), 0);
        assert_eq!(wgsl_parser.module.global_variables.len(), 2);
        assert_eq!(wgsl_parser.module.types.len(), 7);
        assert_eq!(wgsl_parser.module.constants.len(), 7);
        assert_eq!(wgsl_parser.module.entry_points.len(), 2);
    }

    #[test]
    fn spv_parse() {
        init_shader_test_env();

        let spv_parser = ShaderParser::parse(std::env::var("SPV_FILE").unwrap()).unwrap();
        assert_eq!(spv_parser.module.functions.len(), 2);
        assert_eq!(spv_parser.module.global_variables.len(), 7);
        assert_eq!(spv_parser.module.types.len(), 57);
        assert_eq!(spv_parser.module.constants.len(), 39);
        assert_eq!(spv_parser.module.entry_points.len(), 1);
    }

    #[test]
    fn glsl_parse() {
        init_shader_test_env();

        let vert_parser = ShaderParser::parse(std::env::var("VERT_FILE").unwrap()).unwrap();

        assert_eq!(vert_parser.module.functions.len(), 1);
        assert_eq!(vert_parser.module.global_variables.len(), 8);
        assert_eq!(vert_parser.module.types.len(), 8);
        assert_eq!(vert_parser.module.constants.len(), 12);
        assert_eq!(vert_parser.module.entry_points.len(), 1);

        let frag_parser = ShaderParser::parse(std::env::var("FRAG_FILE").unwrap()).unwrap();

        assert_eq!(frag_parser.module.functions.len(), 1);
        assert_eq!(frag_parser.module.global_variables.len(), 3);
        assert_eq!(frag_parser.module.types.len(), 4);
        assert_eq!(frag_parser.module.constants.len(), 4);
        assert_eq!(frag_parser.module.entry_points.len(), 1);

        let comp_parser = ShaderParser::parse(std::env::var("COMP_FILE").unwrap()).unwrap();

        assert_eq!(comp_parser.module.functions.len(), 2);
        assert_eq!(comp_parser.module.global_variables.len(), 2);
        assert_eq!(comp_parser.module.types.len(), 4);
        assert_eq!(comp_parser.module.constants.len(), 11);
        assert_eq!(comp_parser.module.entry_points.len(), 1);
    }

    use crate::shader::converter::*;

    #[test]
    fn encode() {
        init_shader_test_env();

        let target = std::path::Path::new(".\\src\\shader\\shader\\glsl\\test\\encode_test.glsl");

        let wgsl_parser = ShaderParser::parse(std::env::var("WGSL_FILE").unwrap()).unwrap();

        let conversion_res = convert_shader(
            ShaderConvertOption::Glsl {
                version: Version::Desktop(420),
            },
            &wgsl_parser.module,
        )
        .unwrap();

        if let ShaderConvertResult::Glsl(data) = conversion_res {
            match ShaderParser::encode(data, target) {
                Ok(_) => {}
                Err(err) => panic!("shader encode failed : error {}", err),
            }
        }
    }

    #[test]
    fn decode() {
        init_shader_test_env();

        let target = std::path::Path::new(".\\src\\shader\\shader\\glsl\\test\\encode_test.glsl");

        //we want something to read from
        encode();

        let decode_shader = ShaderParser::decode(target);

        if let Ok(data) = decode_shader {
            println!("{}", data);
        } else {
            panic!("Failed to decode shader: file {}", target.display())
        }

        if target.exists() {
            match std::fs::remove_file(target) {
                Ok(_) => {}
                Err(err) => panic!("Failed to remove file {}", err),
            }
        }
    }
}

#[cfg(test)]
mod material_test {

    use crate::init_shader_test_env;
    use crate::shader::parser::*;

    #[test]
    fn display_material() {
        init_shader_test_env();

        let wgsl_path = std::env::var("WGSL_FILE").unwrap();
        let wgsl_path = std::path::Path::new(wgsl_path.as_str());

        let mut material_wgsl_parser = MaterialParser::new("Wgsl Material");
        let wgsl_tree = material_wgsl_parser
            .create_material_hierarchy(wgsl_path)
            .unwrap();

        println!("WGSL TREE:\n{}\n\n", wgsl_tree);

        let spv_path = std::env::var("SPV_FILE").unwrap();
        let spv_path = std::path::Path::new(spv_path.as_str());

        let mut material_spv_parser = MaterialParser::new("SPV Material");

        let spv_tree = material_spv_parser
            .create_material_hierarchy(spv_path)
            .unwrap();
        println!("SPV TREE:\n{}\n\n", spv_tree);

        let vertex_path = std::env::var("VERT_FILE").unwrap();
        let vertex_path = std::path::Path::new(vertex_path.as_str());

        let mut material_vert_parser = MaterialParser::new("GLSL Vertex Material");
        let vertex_tree = material_vert_parser
            .create_material_hierarchy(vertex_path)
            .unwrap();

        println!("GLSL VERTEX TREE:\n{}\n\n", vertex_tree);

        let frag_path = std::env::var("FRAG_FILE").unwrap();
        let frag_path = std::path::Path::new(frag_path.as_str());

        let mut material_frag_parser = MaterialParser::new("GLSL Fragment Material");

        let fragment_tree = material_frag_parser
            .create_material_hierarchy(frag_path)
            .unwrap();

        println!("GLSL FRAGMENT TREE:\n{}\n\n", fragment_tree);

        let compute_path = std::env::var("COMP_FILE").unwrap();
        let compute_path = std::path::Path::new(compute_path.as_str());

        let mut material_comp_parser = MaterialParser::new("GLSL Compute Material");

        let compute_tree = material_comp_parser
            .create_material_hierarchy(compute_path)
            .unwrap();

        println!("GLSL COMPUTE TREE:\n{}\n\n", compute_tree);
    }
}
