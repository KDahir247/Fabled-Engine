use crate::shader::ValidationError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShaderError {
    #[error("Invalid file extension for shader or missing file extension all together")]
    InvalidFileExtension,

    //------- Front End -----------
    #[error("Glsl shader failed in the parsing phase.\nMessage: {}", .0.kind)]
    GLSLParserError(naga::front::glsl::ParseError),

    #[error("Wgsl shader failed in the parsing phase.\nMessage: {}", .0)]
    WGSLParseError(naga::front::wgsl::ParseError),

    #[error("Spir-v shader failed in the parsing phase.\nMessage: {}", .0)]
    SPVParseError(naga::front::spv::Error),

    // --------- Back End ----------
    #[error("shader failed to be converted to glsl shader\nMessage: {} ", .0)]
    GLSLConvertError(naga::back::glsl::Error),

    #[error("shader failed to be converted to wgsl shader\n{}", .0)]
    WGSLConvertError(naga::back::wgsl::Error),

    #[error("shader failed to be converted to spirv shader\nMessage{}", .0)]
    SPVConvertError(naga::back::spv::Error),

    #[error("Error occurred when validating shader file\nMessage{}", .0)]
    ValidationError(ValidationError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
