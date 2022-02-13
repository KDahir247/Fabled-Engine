use crate::material::MaterialPrimitiveType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
pub enum MaterialValueType {
    None,

    UnsignedInt,
    SignedInt,
    Float,

    Vec2Unsigned,
    Vec2Signed,
    Vec2Float,

    Vec3Unsigned,
    Vec3Signed,
    Vec3Float,

    Vec4Unsigned,
    Vec4Signed,
    Vec4Float,

    Mat2Float,
    Mat3Float,
    Mat4Float,

    Sampler,
}


impl From<MaterialValueType> for Option<MaterialPrimitiveType> {
    fn from(target: MaterialValueType) -> Self {
        match target {
            MaterialValueType::None => None,

            MaterialValueType::UnsignedInt
            | MaterialValueType::SignedInt
            | MaterialValueType::Float => Some(MaterialPrimitiveType::Scalar),

            MaterialValueType::Vec2Unsigned
            | MaterialValueType::Vec2Signed
            | MaterialValueType::Vec2Float
            | MaterialValueType::Vec3Unsigned
            | MaterialValueType::Vec3Signed
            | MaterialValueType::Vec3Float
            | MaterialValueType::Vec4Unsigned
            | MaterialValueType::Vec4Signed
            | MaterialValueType::Vec4Float => Some(MaterialPrimitiveType::Vector),

            MaterialValueType::Mat2Float
            | MaterialValueType::Mat3Float
            | MaterialValueType::Mat4Float => Some(MaterialPrimitiveType::Matrix),

            MaterialValueType::Sampler => Some(MaterialPrimitiveType::Sampler),
        }
    }
}
