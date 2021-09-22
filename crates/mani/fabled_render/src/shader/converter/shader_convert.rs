use crate::shader::converter::*;
use crate::shader::parser::*;
use crate::shader::ShaderError;

pub fn convert_shader<P: AsRef<std::path::Path>>(
    conversion: ShaderConvertOption,
    path: P,
) -> Result<ShaderConvertResult, ShaderError> {
    let (module, module_info) = parse_shader(path, None)?;

    let conversion_res = match conversion {
        ShaderConvertOption::Wgsl => {
            let wgsl_buffer = naga::back::wgsl::write_string(&module, &module_info)
                .map_err(ShaderError::WGSLConvertError)?;

            ShaderConvertResult::Wgsl(wgsl_buffer)
        }

        ShaderConvertOption::Spv { option } => {
            let mut spv_option = naga::back::spv::Options::default();

            // capabilities,
            if let SpvOptions::Custom { maj_min } = option {
                // spv_option.capabilities = capabilities;
                spv_option.lang_version = maj_min;
            };

            let spv = naga::back::spv::write_vec(&module, &module_info, &spv_option)
                .map_err(ShaderError::SPVConvertError)?;

            let bytes = spv
                .iter()
                .fold(Vec::with_capacity(spv.len() * 4), |mut acc, index| {
                    acc.extend_from_slice(&index.to_le_bytes());
                    acc
                });

            ShaderConvertResult::Spv(bytes)
        }
        ShaderConvertOption::Glsl { version } => {
            let mut glsl_buffer = String::new();

            for entry in module.entry_points.iter() {
                let glsl_option = naga::back::glsl::Options {
                    version: version.into(),
                    shader_stage: entry.stage,
                    entry_point: entry.name.to_owned(),
                };

                // Handle invalid version for glsl shader.
                // Desktop preprocessor version : [330, 400, 410, 420, 430, 440, 450]
                // Embedded preprocessor version : [300, 310, 320]
                let mut writer = naga::back::glsl::Writer::new(
                    &mut glsl_buffer,
                    &module,
                    &module_info,
                    &glsl_option,
                )
                .map_err(ShaderError::GLSLConvertError)?;

                // if module is invalid, which may happen if the file extension is not know it
                // will return an empty module. This will handle hard error.
                writer.write().map_err(ShaderError::GLSLConvertError)?;
            }

            ShaderConvertResult::Glsl(glsl_buffer)
        }
    };

    Ok(conversion_res)
}

// --------------------- TEST ---------------------

#[cfg(test)]
mod converter_test {
    use crate::shader::converter::*;
    use crate::shader::init_shader_test_env;

    // unwrapping None fails the test since it throws an panic.
    #[test]
    fn convert_wgsl_to() {
        init_shader_test_env();

        let env_wgsl = std::env::var("WGSL_FILE").unwrap();
        let wgsl_path = std::path::Path::new(&env_wgsl);

        let glsl_desktop_representation = convert_shader(
            ShaderConvertOption::Glsl {
                version: Version::Desktop(430),
            },
            wgsl_path,
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
            wgsl_path,
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
            wgsl_path,
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

        let env_spv = std::env::var("SPV_FILE").unwrap();
        let spv_path = std::path::Path::new(&env_spv);

        let glsl_desktop_representation = convert_shader(
            ShaderConvertOption::Glsl {
                version: Version::Desktop(420),
            },
            spv_path,
        )
        .unwrap();

        // GLSL

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
            spv_path,
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

        // WGSL

        let wgsl_representation = convert_shader(ShaderConvertOption::Wgsl, spv_path).unwrap();

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

        let env_glsl_vert = std::env::var("VERT_FILE").unwrap();
        let env_glsl_frag = std::env::var("FRAG_FILE").unwrap();
        let env_glsl_comp = std::env::var("COMP_FILE").unwrap();
        let glsl_vert_path = std::path::Path::new(&env_glsl_vert);
        let glsl_frag_path = std::path::Path::new(&env_glsl_frag);
        let glsl_comp_path = std::path::Path::new(&env_glsl_comp);

        let wgsl_vertex_representation =
            convert_shader(ShaderConvertOption::Wgsl, &glsl_vert_path).unwrap();

        let wgsl_fragment_representation =
            convert_shader(ShaderConvertOption::Wgsl, &glsl_frag_path).unwrap();

        let wgsl_compute_representation =
            convert_shader(ShaderConvertOption::Wgsl, &glsl_comp_path).unwrap();

        // WGSL

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

        let spir_v_vertex_representation = convert_shader(
            ShaderConvertOption::Spv {
                option: SpvOptions::Default,
            },
            glsl_vert_path,
        )
        .unwrap();
        let spir_v_fragment_representation = convert_shader(
            ShaderConvertOption::Spv {
                option: SpvOptions::Default,
            },
            glsl_frag_path,
        )
        .unwrap();
        let spir_v_compute_representation = convert_shader(
            ShaderConvertOption::Spv {
                option: SpvOptions::Default,
            },
            glsl_comp_path,
        )
        .unwrap();

        // SPIR-V

        if let ShaderConvertResult::Spv(v) = spir_v_vertex_representation {
            println!("SPV VERTEX INTERPRETATION\n {:?}", v);
        } else {
            panic!(
                "SPV decode failed and return another data type other than spv\n{:?}",
                spir_v_vertex_representation
            )
        }

        if let ShaderConvertResult::Spv(v) = spir_v_fragment_representation {
            println!("SPV FRAGMENT INTERPRETATION\n {:?}", v);
        } else {
            panic!(
                "SPV decode failed and return another data type other than spv\n{:?}",
                spir_v_fragment_representation
            )
        }

        if let ShaderConvertResult::Spv(v) = spir_v_compute_representation {
            println!("SPV COMPUTE INTERPRETATION\n {:?}", v);
        } else {
            panic!(
                "SPV decode failed and return another data type other than spv\n{:?}",
                spir_v_compute_representation
            )
        }
    }

    #[test]
    #[should_panic]
    fn invalid_convert() {
        let env_wgsl = std::env::var("WGSL_FILE").unwrap();
        let wgsl_path = std::path::Path::new(&env_wgsl);

        convert_shader(
            ShaderConvertOption::Glsl {
                // Supported es glsl versions [300, 310, 320]
                version: Version::Embedded(330),
            },
            wgsl_path,
        )
        .unwrap();

        convert_shader(
            ShaderConvertOption::Glsl {
                // Supported core glsl versions [330, 400, 410,
                // 420, 430, 440, 450]
                version: Version::Desktop(460),
            },
            wgsl_path,
        )
        .unwrap();
    }
}
