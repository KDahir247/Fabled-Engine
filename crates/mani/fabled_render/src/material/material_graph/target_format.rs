use crate::material::TypeFormat;
use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum MaterialTargetFormat {
    Undefined,
    UnsignedInt,
    SignedInt,
    Float,
    Boolean,

    Vector2UnsignedInt,
    Vector2SignedInt,
    Vector2Float,
    Vector2Boolean,

    Vector4UnsignedInt,
    Vector4SignedInt,
    Vector4Float,
    Vector4Boolean,

    Matrix2x2Float,
    Matrix4x4Float,

    Sampler,
    Texture,
}

impl From<MaterialTargetFormat> for TypeFormat {
    fn from(material_format: MaterialTargetFormat) -> Self {
        //Convert material variable type without data to variable primitive type
        match material_format {
            MaterialTargetFormat::Undefined
            | MaterialTargetFormat::Sampler
            | MaterialTargetFormat::Texture => TypeFormat::Undefined,

            MaterialTargetFormat::UnsignedInt
            | MaterialTargetFormat::Vector2UnsignedInt
            | MaterialTargetFormat::Vector4UnsignedInt => TypeFormat::Uint32,

            MaterialTargetFormat::SignedInt
            | MaterialTargetFormat::Vector2SignedInt
            | MaterialTargetFormat::Vector4SignedInt => TypeFormat::SInt32,

            MaterialTargetFormat::Float
            | MaterialTargetFormat::Vector2Float
            | MaterialTargetFormat::Vector4Float
            | MaterialTargetFormat::Matrix2x2Float
            | MaterialTargetFormat::Matrix4x4Float => TypeFormat::Float32,

            MaterialTargetFormat::Boolean
            | MaterialTargetFormat::Vector2Boolean
            | MaterialTargetFormat::Vector4Boolean => TypeFormat::Boolean,
        }
    }
}
