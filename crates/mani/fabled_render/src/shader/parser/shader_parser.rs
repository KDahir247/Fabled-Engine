use crate::shader::validation_rule::ValidationLayer;
use crate::shader::{ParseOption, ShaderError};
use std::io::{Read, Write};

const AUTO_GEN_MESSAGE : &str = "//This is auto generated code. Do not modify code! Modification can break interpreted solution.\n//Modify shader code only if you know what you're doing.\n\n";

pub fn parse_shader<P: AsRef<std::path::Path>>(
    source: P,
    parse_option: Option<ParseOption>,
) -> Result<(naga::Module, naga::valid::ModuleInfo), ShaderError> {
    let path = source.as_ref();
    let mut file = std::fs::File::open(path)?;

    let content_capacity = file.metadata()?.len() as usize;
    let mut file_content_buf = String::with_capacity(content_capacity);

    // We can guarantee that unwrap or returning a default OStr will be a "" if we
    // convert it to a &str, so we will bypass the check.
    let file_ext = unsafe {
        path.extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_unchecked()
    };

    let module = match file_ext {
        "wgsl" => {
            file.read_to_string(&mut file_content_buf);

            naga::front::wgsl::parse_str(&file_content_buf).map_err(ShaderError::WGSLParseError)?
        }
        "spv" => {
            let mut file_byte_buf = vec![0; content_capacity];

            // when we create filesystem module a more polished implementation will be
            // substituted currently this will be sufficient, since it prevent
            // TOCTTOU (Time-of-check to time-of-use) attack
            let shader_meta_path =
                std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("\\shader\\shader_dump");

            std::fs::create_dir_all(shader_meta_path.as_path())?;

            let mut options = naga::front::spv::Options {
                flow_graph_dump_prefix: Some(shader_meta_path),
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

            file.read_exact(&mut file_byte_buf)?;

            naga::front::spv::parse_u8_slice(&file_byte_buf, &options)
                .map_err(ShaderError::SPVParseError)?
        }

        stage @ "vert" | stage @ "frag" | stage @ "comp" => unsafe {
            file.read_to_string(&mut file_content_buf);

            let mut entry_points = naga::FastHashMap::default();

            let mut main_func_entry = "main".to_string();

            if let Some(ParseOption::Glsl { entry_point }) = parse_option {
                main_func_entry = entry_point;
            };

            let target = match stage {
                "vert" => naga::ShaderStage::Vertex,
                "frag" => naga::ShaderStage::Fragment,
                "comp" => naga::ShaderStage::Compute,
                _ => std::hint::unreachable_unchecked(),
            };

            entry_points.insert(main_func_entry, target);

            naga::front::glsl::parse_str(
                &file_content_buf,
                &naga::front::glsl::Options {
                    entry_points,
                    defines: Default::default(),
                },
            )
            .map_err(ShaderError::GLSLParserError)?
        },
        _ => naga::Module::default(),
    };

    // validate shader module. Specifying a harsh validation on the shader.
    let module_info = module
        .validate(naga::valid::ValidationFlags::all())
        .map_err(ShaderError::ValidationError)?;

    Ok((module, module_info))
}

pub fn encode_shader<P: AsRef<std::path::Path>>(
    shader_data: String,
    target: P,
) -> Result<(), ShaderError> {
    let mut file = std::fs::File::create(target)?;

    let buffer_size = AUTO_GEN_MESSAGE.len() + shader_data.len();
    let mut buffer = vec![0; buffer_size];

    let (header, data) = buffer.split_at_mut(AUTO_GEN_MESSAGE.len());
    header.copy_from_slice(AUTO_GEN_MESSAGE.as_bytes());
    data.copy_from_slice(shader_data.as_bytes());

    file.write_all(&buffer)?;

    Ok(())
}

pub fn decode_shader<P: AsRef<std::path::Path>>(target: P) -> Result<String, ShaderError> {
    let mut file = std::fs::File::open(target)?;

    let length = file.metadata()?.len() as usize;
    let mut buffer = vec![0; length];
    file.read_exact(&mut buffer)?;

    let data = String::from_utf8(buffer).unwrap_or_default();

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
        let wgsl_module = parse_shader(wgsl_shader_path, None).unwrap().0;

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
        let (spv_module, _) = parse_shader(spv_shader_path, None).unwrap();

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
        let (vert_module, _) = parse_shader(glsl_vert_shader_path, None).unwrap();

        assert_eq!(vert_module.functions.len(), 1);
        assert_eq!(vert_module.global_variables.len(), 8);
        assert_eq!(vert_module.types.len(), 8);
        assert_eq!(vert_module.constants.len(), 12);
        assert_eq!(vert_module.entry_points.len(), 1);

        let glsl_frag_shader_path = std::env::var("FRAG_FILE").unwrap();
        let (frag_module, _) = parse_shader(glsl_frag_shader_path, None).unwrap();

        assert_eq!(frag_module.functions.len(), 1);
        assert_eq!(frag_module.global_variables.len(), 3);
        assert_eq!(frag_module.types.len(), 4);
        assert_eq!(frag_module.constants.len(), 4);
        assert_eq!(frag_module.entry_points.len(), 1);

        let glsl_comp_shader_path = std::env::var("COMP_FILE").unwrap();
        let (comp_module, _) = parse_shader(glsl_comp_shader_path, None).unwrap();

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

        // we want something to read from
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
