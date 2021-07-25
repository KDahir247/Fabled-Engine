use crate::shader::validation_rule::ValidationLayer;
use anyhow::Context;
use std::io::{Read, Write};
pub const AUTO_GEN_MESSAGE : &str = "//This is auto generated code. Do not modify code! Modification can break interpreted solution.\n//Modify shader code if you know what you're doing.\n\n";

#[repr(align(128))]
pub struct ShaderParser {
    pub module: naga::Module,
}

impl ShaderParser {
    pub fn parse<P: AsRef<std::path::Path>>(source: P) -> anyhow::Result<Self> {
        let file = source.as_ref();

        let file_ext = file
            .extension()
            .context("Input has no extension? or file name?")?
            .to_str()
            .context("Failed to convert to &str due to utf8 validity failure. Format unicode is formatted other utf8")?;

        let module = match file_ext {
            "wgsl" => {
                let input = std::fs::read_to_string(file)?;
                let result = naga::front::wgsl::parse_str(&input);
                result?
            }
            "spv" => {
                let options = naga::front::spv::Options::default();
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

                entry_points.insert("main".to_string(), target);
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

        Ok(Self { module })
    }

    pub fn encode<P: AsRef<std::path::Path>>(shader_data: String, target: P) -> anyhow::Result<()> {
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

    pub fn decode<P: AsRef<std::path::Path>>(target: P) -> anyhow::Result<String> {
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
}

// -------------- Test ---------------

#[cfg(test)]
mod shader_test {
    use crate::init_shader_test_env;
    use crate::shader::parser::*;

    #[test]
    fn wgsl_parse() {
        init_shader_test_env();

        let wgsl_parser = ShaderParser::parse(std::env::var("WGSL_FILE").unwrap()).unwrap();
        assert_eq!(wgsl_parser.module.functions.len(), 4);
        assert_eq!(wgsl_parser.module.global_variables.len(), 1);
        assert_eq!(wgsl_parser.module.types.len(), 10);
        assert_eq!(wgsl_parser.module.constants.len(), 22);
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
