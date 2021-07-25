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
