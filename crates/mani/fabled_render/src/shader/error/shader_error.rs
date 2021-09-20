use crate::shader::ValidationError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShaderError {
    #[error("Invalid file extension for shader or missing file extension all together")]
    InvalidFileExtension,

    #[error("Glsl shader failed in the parsing phase.\nMessage: {}", .0.kind)]
    GlslParserError(naga::front::glsl::ParseError),

    #[error("Wgsl shader failed in the parsing phase.\nMessage: {}", .0)]
    WGSLParseError(naga::front::wgsl::ParseError),

    #[error("Spir-v shader failed in the parsing phase.\nMessage: {}", .0)]
    SPVParseError(naga::front::spv::Error),

    #[error("Error occurred when validating shader file\nMessage{}", .0)]
    ValidationError(ValidationError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
