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
