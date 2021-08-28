use anyhow::Context;
use std::io::{Read, Write};

use crate::shader::validation_rule::ValidationLayer;
use crate::shader::ParseOption;

const AUTO_GEN_MESSAGE : &str = "//This is auto generated code. Do not modify code! Modification can break interpreted solution.\n//Modify shader code if you know what you're doing.\n\n";

pub fn parse_shader<P: AsRef<std::path::Path>>(
    source: P,
    parse_option: Option<ParseOption>,
) -> anyhow::Result<naga::Module> {
    let file = source.as_ref();

    let file_ext = file
        .extension()
        .context("Input has no extension? or file name?")?
        .to_str()
        .unwrap();

    let module = match file_ext {
        "wgsl" => {
            let input = std::fs::read_to_string(file)?;
            let result = naga::front::wgsl::parse_str(&input);
            result?
        }
        "spv" => {
            // when we create filesystem module a more polished implementation will be substituted
            // currently this will be sufficient, since it prevent TOCTTOU (Time-of-check to time-of-use) attack
            let mut render_module = env!("CARGO_MANIFEST_DIR").to_string();
            render_module.push_str("\\shader\\shader_dump");
            std::fs::create_dir_all(render_module.clone()).unwrap();

            let mut options = naga::front::spv::Options {
                flow_graph_dump_prefix: Some(std::path::PathBuf::from(render_module)),
                ..Default::default()
            };

            if let Some(ParseOption::SPV {
                adjust_coordinate_space,
                strict_capabilities,
            }) = parse_option
            {
                options.adjust_coordinate_space = adjust_coordinate_space;
                options.strict_capabilities = strict_capabilities;
            }

            let input = std::fs::read(file)?;
            let result = naga::front::spv::parse_u8_slice(&input, &options);
            result?
        }

        stage @ "vert" | stage @ "frag" | stage @ "comp" => {
            let input = std::fs::read_to_string(file)?;
            let mut entry_points = naga::FastHashMap::default();

            let target = match stage {
                    "vert" => naga::ShaderStage::Vertex,
                    "frag" => naga::ShaderStage::Fragment,
                    "comp" => naga::ShaderStage::Compute,
                    _ => panic!("expect error correct glsl extension (vert, frag, comp) has been re-evaluated as incorrect?"),
                };

            let mut main_func_entry = "main".to_string();

            if let Some(ParseOption::Glsl { entry_point }) = parse_option {
                main_func_entry = entry_point;
            };

            entry_points.insert(main_func_entry, target);
            naga::front::glsl::parse_str(
                &input,
                &naga::front::glsl::Options {
                    entry_points,
                    defines: Default::default(),
                },
            )?
        }
        _ => naga::Module::default(),
    };

    //validate shader module. Specifying a harsh validation on the shader.
    module.validate(naga::valid::ValidationFlags::all())?;

    Ok(module)
}

pub fn encode_shader<P: AsRef<std::path::Path>>(
    shader_data: String,
    target: P,
) -> anyhow::Result<()> {
    let file = std::fs::File::create(target).expect("Unable to create ron_material file");
    let mut buffer_writer = std::io::BufWriter::new(file);
    let mut auto_gen = AUTO_GEN_MESSAGE.to_string();
    auto_gen.push_str(shader_data.as_str());
    buffer_writer
        .write_all(auto_gen.as_bytes())
        .expect("Unable to write to write to target ron_material file");
    buffer_writer.flush()?;
    Ok(())
}

pub fn decode_shader<P: AsRef<std::path::Path>>(target: P) -> anyhow::Result<String> {
    let mut data = String::new();

    {
        let file = std::fs::File::open(target).expect("Unable to open ron_material file");
        let mut buffer_reader = std::io::BufReader::new(file);
        buffer_reader
            .read_to_string(&mut data)
            .expect("Unable to read string");
    }

    Ok(data)
}

// -------------- Test ---------------

#[cfg(test)]
mod shader_test {
    use crate::shader::converter::*;
    use crate::shader::init_shader_test_env;
    use crate::shader::parser::*;

    #[test]
    fn wgsl_parse() {
        init_shader_test_env();

        let wgsl_shader_path = std::env::var("WGSL_FILE").unwrap();
        let wgsl_module = parse_shader(wgsl_shader_path, None).unwrap();

        assert_eq!(wgsl_module.functions.len(), 4);
        assert_eq!(wgsl_module.global_variables.len(), 1);
        assert_eq!(wgsl_module.types.len(), 10);
        assert_eq!(wgsl_module.constants.len(), 22);
        assert_eq!(wgsl_module.entry_points.len(), 2);
    }

    #[test]
    fn spv_parse() {
        init_shader_test_env();

        let spv_shader_path = std::env::var("SPV_FILE").unwrap();
        let spv_module = parse_shader(spv_shader_path, None).unwrap();

        assert_eq!(spv_module.functions.len(), 2);
        assert_eq!(spv_module.global_variables.len(), 7);
        assert_eq!(spv_module.types.len(), 57);
        assert_eq!(spv_module.constants.len(), 39);
        assert_eq!(spv_module.entry_points.len(), 1);
    }

    #[test]
    fn glsl_parse() {
        init_shader_test_env();

        let glsl_vert_shader_path = std::env::var("VERT_FILE").unwrap();
        let vert_module = parse_shader(glsl_vert_shader_path, None).unwrap();

        assert_eq!(vert_module.functions.len(), 1);
        assert_eq!(vert_module.global_variables.len(), 8);
        assert_eq!(vert_module.types.len(), 8);
        assert_eq!(vert_module.constants.len(), 12);
        assert_eq!(vert_module.entry_points.len(), 1);

        let glsl_frag_shader_path = std::env::var("FRAG_FILE").unwrap();
        let frag_module = parse_shader(glsl_frag_shader_path, None).unwrap();

        assert_eq!(frag_module.functions.len(), 1);
        assert_eq!(frag_module.global_variables.len(), 3);
        assert_eq!(frag_module.types.len(), 4);
        assert_eq!(frag_module.constants.len(), 4);
        assert_eq!(frag_module.entry_points.len(), 1);

        let glsl_comp_shader_path = std::env::var("COMP_FILE").unwrap();
        let comp_module = parse_shader(glsl_comp_shader_path, None).unwrap();

        assert_eq!(comp_module.functions.len(), 2);
        assert_eq!(comp_module.global_variables.len(), 2);
        assert_eq!(comp_module.types.len(), 4);
        assert_eq!(comp_module.constants.len(), 11);
        assert_eq!(comp_module.entry_points.len(), 1);
    }

    #[test]
    fn encode_test() {
        init_shader_test_env();

        let source = std::env::var("WGSL_FILE").unwrap();
        let target = std::path::Path::new(".\\src\\shader\\shader\\glsl\\test\\encode_test.glsl");

        let conversion_res = convert_shader(
            ShaderConvertOption::Glsl {
                version: Version::Desktop(420),
            },
            source,
        )
        .unwrap();

        if let ShaderConvertResult::Glsl(data) = conversion_res {
            match encode_shader(data, target) {
                Ok(_) => {}
                Err(err) => panic!("shader encode failed : error {}", err),
            }
        }
    }

    #[test]
    fn decode_test() {
        init_shader_test_env();

        let target = std::path::Path::new(".\\src\\shader\\shader\\glsl\\test\\encode_test.glsl");

        //we want something to read from
        encode_test();

        let decode_shader = decode_shader(target);

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
