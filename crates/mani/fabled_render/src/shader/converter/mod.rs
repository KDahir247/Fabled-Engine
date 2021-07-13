mod shader_convert;
pub use shader_convert::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Version {
    ///`core` glsl
    Desktop(u16),
    //`es` glsl
    Embedded(u16),
}

impl From<Version> for naga::back::glsl::Version {
    fn from(version: Version) -> Self {
        match version {
            Version::Desktop(desk_prep) => naga::back::glsl::Version::Desktop(desk_prep),
            Version::Embedded(embed_prep) => naga::back::glsl::Version::Embedded(embed_prep),
        }
    }
}

#[derive(Debug)]
#[repr(align(4))]
pub enum SpvOptions {
    Default,
    Custom {
        maj_min: (u8, u8),
        //capabilities: Option<naga::FastHashSet<naga::back::spv::Capability>>,
    },
}

#[derive(Debug)]
pub enum ShaderConvertOption {
    Wgsl,
    Spv { option: SpvOptions },
    Glsl { version: Version },
}

#[derive(Debug, PartialEq)]
pub enum ShaderConvertResult {
    Wgsl(String),
    Spv(Vec<u8>),
    Glsl(String),
}

#[cfg(test)]
mod data_alignment_test {

    use crate::shader::converter::*;

    #[test]
    fn data_alignment() {
        //version
        let version_data = std::mem::size_of::<Version>();
        assert_eq!(version_data & (version_data - 1), 0);

        //result
        let shader_result_data = std::mem::size_of::<ShaderConvertResult>();
        assert_eq!(shader_result_data & (shader_result_data - 1), 0);

        //spv option
        let spv_option_data = std::mem::size_of::<SpvOptions>();
        assert_eq!(spv_option_data & (spv_option_data - 1), 0);

        //option
        let shader_option_data = std::mem::size_of::<ShaderConvertOption>();
        assert_eq!(shader_option_data & (shader_option_data - 1), 0);
    }
}

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
