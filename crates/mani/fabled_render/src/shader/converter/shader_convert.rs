use crate::shader::converter::*;
use crate::shader::validation_rule::ValidationLayer;

pub fn convert_shader(
    conversion: ShaderConvertOption,
    shader_module: &naga::Module,
) -> anyhow::Result<ShaderConvertResult> {
    let shader_info = shader_module.validate(naga::valid::ValidationFlags::all())?;

    let conversion_res = match conversion {
        ShaderConvertOption::Wgsl => {
            let wgsl_buffer = naga::back::wgsl::write_string(shader_module, &shader_info)?;

            ShaderConvertResult::Wgsl(wgsl_buffer)
        }
        ShaderConvertOption::Spv { option } => {
            let mut spv_option = naga::back::spv::Options::default();

            if let SpvOptions::Custom {
                maj_min,
                //capabilities,
            } = option
            {
                //spv_option.capabilities = capabilities;
                spv_option.lang_version = maj_min;
            };

            let spv = naga::back::spv::write_vec(shader_module, &shader_info, &spv_option)?;
            let bytes = spv
                .iter()
                .fold(Vec::with_capacity(spv.len() * 4), |mut acc, index| {
                    acc.extend_from_slice(&index.to_le_bytes());
                    acc
                });

            ShaderConvertResult::Spv(bytes)
        }
        ShaderConvertOption::Glsl { version } => {
            match version {
                Version::Desktop(desk_prep) => {
                    assert!(
                        desk_prep <= *naga::back::glsl::SUPPORTED_CORE_VERSIONS.last().unwrap(),
                        "Desktop preprocessor version is not supported. Supported versions are : [330, 400, 410, 420, 430, 440, 450]"
                    );
                }
                Version::Embedded(embed_prep) => {
                    assert!(
                        embed_prep <= *naga::back::glsl::SUPPORTED_ES_VERSIONS.last().unwrap(),
                        "Embedded preprocessor version is not supported. Supported versions are : [300, 310, 320]"
                    );
                }
            };

            let mut glsl_buffer = String::new();

            for entry in shader_module.entry_points.iter() {
                let glsl_option = naga::back::glsl::Options {
                    version: version.into(),
                    shader_stage: entry.stage,
                    entry_point: entry.name.to_owned(),
                };

                let mut writer = naga::back::glsl::Writer::new(
                    &mut glsl_buffer,
                    shader_module,
                    &shader_info,
                    &glsl_option,
                )?;

                writer.write()?;
            }

            ShaderConvertResult::Glsl(glsl_buffer)
        }
    };

    Ok(conversion_res)
}

// --------------------- TEST ---------------------

#[cfg(test)]
mod converter_test {
    use crate::init_shader_test_env;
    use crate::shader::converter::*;
    use crate::shader::parser::*;

    //unwrapping None fails the test since it throws an panic.
    #[test]
    fn convert_wgsl_to() {
        init_shader_test_env();

        let wgsl_parse = ShaderParser::parse(std::env::var("WGSL_FILE").unwrap()).unwrap();

        let glsl_desktop_representation = convert_shader(
            ShaderConvertOption::Glsl {
                version: Version::Desktop(430),
            },
            &wgsl_parse.module,
        )
        .unwrap();

        if let ShaderConvertResult::Glsl(data) = glsl_desktop_representation {
            println!("GLSL Desktop INTERPRETATION\n {}", data);
        } else {
            panic!(
                "Glsl Core decode failed and return another data type other than glsl\n{:?}",
                glsl_desktop_representation
            );
        }

        let glsl_core_representation = convert_shader(
            ShaderConvertOption::Glsl {
                version: Version::Embedded(320),
            },
            &wgsl_parse.module,
        )
        .unwrap();

        if let ShaderConvertResult::Glsl(data) = &glsl_core_representation {
            println!("GLSL Embedded INTERPRETATION\n {}", data);
        } else {
            panic!(
                "Glsl ES decode failed and return another data type other than glsl\n{:?}",
                glsl_core_representation
            );
        }

        let spv_representation = convert_shader(
            ShaderConvertOption::Spv {
                option: SpvOptions::Default,
            },
            &wgsl_parse.module,
        )
        .unwrap();

        if let ShaderConvertResult::Spv(ref v) = spv_representation {
            println!("SPV INTERPRETATION\n {:?}", v);
        } else {
            panic!(
                "SPV decode failed and return another data type other than spv\n{:?}",
                spv_representation
            )
        }
    }

    #[test]
    fn convert_spv_to() {
        init_shader_test_env();

        let spv_parse = ShaderParser::parse(std::env::var("SPV_FILE").unwrap()).unwrap();

        let glsl_desktop_representation = convert_shader(
            ShaderConvertOption::Glsl {
                version: Version::Desktop(420),
            },
            &spv_parse.module,
        )
        .unwrap();

        //GLSL

        if let ShaderConvertResult::Glsl(data) = glsl_desktop_representation {
            println!("GLSL Desktop INTERPRETATION\n {}", data);
        } else {
            panic!(
                "Glsl Core decode failed and return another data type other than glsl\n{:?}",
                glsl_desktop_representation
            );
        }

        let glsl_embedded_representation = convert_shader(
            ShaderConvertOption::Glsl {
                version: Version::Embedded(320),
            },
            &spv_parse.module,
        )
        .unwrap();

        if let ShaderConvertResult::Glsl(data) = &glsl_embedded_representation {
            println!("GLSL Embedded INTERPRETATION\n {}", data);
        } else {
            panic!(
                "Glsl ES decode failed and return another data type other than glsl\n{:?}",
                glsl_embedded_representation
            );
        }

        //WGSL

        let wgsl_representation =
            convert_shader(ShaderConvertOption::Wgsl, &spv_parse.module).unwrap();

        if let ShaderConvertResult::Wgsl(data) = wgsl_representation {
            println!("WEB_GPU INTERPRETATION\n {}", data);
        } else {
            panic!(
                "Wgsl decode failed and return another data type other than wgsl\n{:?}",
                wgsl_representation
            )
        }
    }

    #[test]
    fn convert_glsl_to() {
        init_shader_test_env();

        let glsl_vert_parse = ShaderParser::parse(std::env::var("VERT_FILE").unwrap()).unwrap();
        let glsl_frag_parse = ShaderParser::parse(std::env::var("FRAG_FILE").unwrap()).unwrap();
        let glsl_comp_parse = ShaderParser::parse(std::env::var("COMP_FILE").unwrap()).unwrap();

        let wgsl_vertex_representation =
            convert_shader(ShaderConvertOption::Wgsl, &glsl_vert_parse.module).unwrap();

        let wgsl_fragment_representation =
            convert_shader(ShaderConvertOption::Wgsl, &glsl_frag_parse.module).unwrap();

        let wgsl_compute_representation =
            convert_shader(ShaderConvertOption::Wgsl, &glsl_comp_parse.module).unwrap();

        //WGSL

        if let ShaderConvertResult::Wgsl(data) = wgsl_vertex_representation {
            println!("WEB_GPU VERTEX INTERPRETATION\n {}", data);
        } else {
            panic!(
                "Wgsl decode failed and return another data type other than wgsl\n{:?}",
                wgsl_vertex_representation
            )
        }

        if let ShaderConvertResult::Wgsl(data) = wgsl_fragment_representation {
            println!("WEB_GPU FRAGMENT INTERPRETATION\n {}", data);
        } else {
            panic!(
                "Wgsl decode failed and return another data type other than wgsl\n{:?}",
                wgsl_fragment_representation
            )
        }

        if let ShaderConvertResult::Wgsl(data) = wgsl_compute_representation {
            println!("WEB_GPU COMPUTE INTERPRETATION\n {}", data);
        } else {
            panic!(
                "Wgsl decode failed and return another data type other than wgsl\n{:?}",
                wgsl_compute_representation
            )
        }

        let spirv_vertex_representation = convert_shader(
            ShaderConvertOption::Spv {
                option: SpvOptions::Default,
            },
            &glsl_vert_parse.module,
        )
        .unwrap();
        let spirv_fragment_representation = convert_shader(
            ShaderConvertOption::Spv {
                option: SpvOptions::Default,
            },
            &glsl_frag_parse.module,
        )
        .unwrap();
        let spirv_compute_representation = convert_shader(
            ShaderConvertOption::Spv {
                option: SpvOptions::Default,
            },
            &glsl_comp_parse.module,
        )
        .unwrap();

        //SPIR-V

        if let ShaderConvertResult::Spv(v) = spirv_vertex_representation {
            println!("SPV VERTEX INTERPRETATION\n {:?}", v);
        } else {
            panic!(
                "SPV decode failed and return another data type other than spv\n{:?}",
                spirv_vertex_representation
            )
        }

        if let ShaderConvertResult::Spv(v) = spirv_fragment_representation {
            println!("SPV FRAGMENT INTERPRETATION\n {:?}", v);
        } else {
            panic!(
                "SPV decode failed and return another data type other than spv\n{:?}",
                spirv_fragment_representation
            )
        }

        if let ShaderConvertResult::Spv(v) = spirv_compute_representation {
            println!("SPV COMPUTE INTERPRETATION\n {:?}", v);
        } else {
            panic!(
                "SPV decode failed and return another data type other than spv\n{:?}",
                spirv_compute_representation
            )
        }
    }

    #[test]
    #[should_panic]
    fn invalid_convert() {
        let wgsl_parse = ShaderParser::parse(std::env::var("WGSL_FILE").unwrap()).unwrap();

        convert_shader(
            ShaderConvertOption::Glsl {
                version: Version::Embedded(330), //Supported es glsl versions [300, 310, 320]
            },
            &wgsl_parse.module,
        )
        .unwrap();

        convert_shader(
            ShaderConvertOption::Glsl {
                version: Version::Desktop(460), // Supported core glsl versions [330, 400, 410, 420, 430, 440, 450]
            },
            &wgsl_parse.module,
        )
        .unwrap();
    }
}
