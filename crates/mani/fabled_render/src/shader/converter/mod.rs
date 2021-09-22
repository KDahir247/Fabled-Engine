mod shader_convert;

pub use shader_convert::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Version {
    /// `core` glsl
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(align(4))]
pub enum SpvOptions {
    Default,
    Custom {
        maj_min: (u8, u8),
        // capabilities: Option<naga::FastHashSet<naga::back::spv::Capability>>,
    },
}

impl Default for SpvOptions {
    fn default() -> Self {
        SpvOptions::Default
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
mod data_test {

    use crate::shader::converter::*;

    #[test]
    fn data_size() {
        // version
        let version_size = std::mem::size_of::<Version>();
        assert_eq!(version_size & (version_size - 1), 0);

        // result
        let shader_result_size = std::mem::size_of::<ShaderConvertResult>();
        assert_eq!(shader_result_size & (shader_result_size - 1), 0);

        // spv option
        let spv_option_size = std::mem::size_of::<SpvOptions>();
        assert_eq!(spv_option_size & (spv_option_size - 1), 0);

        // option
        let shader_option_size = std::mem::size_of::<ShaderConvertOption>();
        assert_eq!(shader_option_size & (shader_option_size - 1), 0);
    }

    #[test]
    fn data_alignment() {
        let version_alignment = std::mem::align_of::<Version>();
        assert_eq!(version_alignment & (version_alignment - 1), 0);

        let shader_result_alignment = std::mem::align_of::<ShaderConvertResult>();
        assert_eq!(shader_result_alignment & (shader_result_alignment - 1), 0);

        let spv_option_alignment = std::mem::align_of::<SpvOptions>();
        assert_eq!(spv_option_alignment & (spv_option_alignment - 1), 0);

        let shader_option_alignment = std::mem::align_of::<ShaderConvertOption>();
        assert_eq!(shader_option_alignment & (shader_option_alignment - 1), 0);
    }
}
