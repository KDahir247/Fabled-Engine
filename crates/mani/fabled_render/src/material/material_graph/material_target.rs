use crate::material::MaterialAttributes;
use fabled_core::prime::container::wrapper::Wrapper;
use naga::{ScalarKind, TypeInner, VectorSize};
use serde::*;

//todo don't like how this is implemented. Should be re looked at later (MaterialTarget 80 bytes)
// should be 8 bytes or 24 bytes at max. The code can get big very fast if adding supported for 3x2,
// 2x3 etc... matrix and extra support for primitive.

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum EmptyTarget {
    None,
}

impl From<EmptyTarget> for MaterialAttributes {
    fn from(_: EmptyTarget) -> Self {
        MaterialAttributes::Empty
    }
}

#[rustfmt::skip]
//this will extend primitives to support sampler and texture.
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum MaterialTarget {
    None,

    UnsignedInt(Wrapper<u32>),
    SignedInt(Wrapper<i32>),
    Float(Wrapper<f32>),

    Vec2UnsignedInt(Wrapper<[u32; 2]>),
    Vec2SingedInt(Wrapper<[i32; 2]>),
    Vec2Float(Wrapper<[f32; 2]>),

    Vec4UnsignedInt(Wrapper<[u32; 4]>),
    Vec4SignedInt(Wrapper<[i32; 4]>),
    Vec4Float(Wrapper<[f32; 4]>),

    Matrix2x2Float(Wrapper<[f32; 4]>),
    Matrix4x4Float(Wrapper<[f32; 16]>),

    Sampler(Wrapper<u8>), // 1 byte

    //UnAligned
    // Texture should store an Option of Tiling and Offset, Turbulence, and a way to reference the texture.
    //todo replay with a POD (Plain old data type) that is aligned to the power of two. rather than a string.
    //todo got to find a better identifier for this type.
    //Texture(&'static [u8]), //), //24 bytes
}

impl From<MaterialTarget> for Option<MaterialAttributes> {
    fn from(target: MaterialTarget) -> Self {
        match target {
            MaterialTarget::None => None,
            MaterialTarget::UnsignedInt(_)
            | MaterialTarget::SignedInt(_)
            | MaterialTarget::Float(_) => Some(MaterialAttributes::Scalar),
            MaterialTarget::Vec2UnsignedInt(_)
            | MaterialTarget::Vec2SingedInt(_)
            | MaterialTarget::Vec2Float(_)
            | MaterialTarget::Vec4UnsignedInt(_)
            | MaterialTarget::Vec4SignedInt(_)
            | MaterialTarget::Vec4Float(_) => Some(MaterialAttributes::Vector),
            MaterialTarget::Matrix2x2Float(_) | MaterialTarget::Matrix4x4Float(_) => {
                Some(MaterialAttributes::Matrix)
            }
            MaterialTarget::Sampler(_) => Some(MaterialAttributes::Sampler),
            //MaterialTarget::Texture(_) => MaterialTargetFormat::Texture,
        }
    }
}

impl From<&naga::TypeInner> for MaterialTarget {
    fn from(type_var: &naga::TypeInner) -> Self {
        match type_var {
            TypeInner::Scalar { kind, .. } => match kind {
                ScalarKind::Sint => MaterialTarget::SignedInt(Wrapper::new(0)),
                ScalarKind::Uint => MaterialTarget::UnsignedInt(Wrapper::new(0)),
                ScalarKind::Float => MaterialTarget::Float(Wrapper::new(0.0)),
                _ => MaterialTarget::None,
            },
            TypeInner::Vector { size, kind, .. } => match size {
                VectorSize::Bi => match kind {
                    ScalarKind::Sint => MaterialTarget::Vec2SingedInt(Wrapper::new([0i32; 2])),
                    ScalarKind::Uint => MaterialTarget::Vec2UnsignedInt(Wrapper::new([0u32; 2])),
                    ScalarKind::Float => MaterialTarget::Vec2Float(Wrapper::new([0.0f32; 2])),
                    _ => MaterialTarget::None,
                },
                VectorSize::Tri => match kind {
                    //Aligned data only. Any Tri-Vector will result to a Tri-Vector that is extend by 1 to a Quad-Vector.
                    ScalarKind::Sint => MaterialTarget::Vec4SignedInt(Wrapper::new([0i32; 4])),
                    ScalarKind::Uint => MaterialTarget::Vec4UnsignedInt(Wrapper::new([0u32; 4])),
                    ScalarKind::Float => MaterialTarget::Vec4Float(Wrapper::new([0.0f32; 4])),
                    _ => MaterialTarget::None,
                },
                VectorSize::Quad => match kind {
                    ScalarKind::Sint => MaterialTarget::Vec4SignedInt(Wrapper::new([0i32; 4])),
                    ScalarKind::Uint => MaterialTarget::Vec4UnsignedInt(Wrapper::new([0u32; 4])),
                    ScalarKind::Float => MaterialTarget::Vec4Float(Wrapper::new([0.0f32; 4])),
                    _ => MaterialTarget::None,
                },
            },
            TypeInner::Matrix { columns, rows, .. } => match columns {
                VectorSize::Bi => match rows {
                    VectorSize::Bi => MaterialTarget::Matrix2x2Float(Wrapper::new([0.0f32; 4])),
                    _ => MaterialTarget::None,
                },
                VectorSize::Tri => match rows {
                    //Aligned data only. Any Tri-Matrix will result to a 4x4 Matrix where the last row and column are zero-ed out
                    VectorSize::Tri => MaterialTarget::Matrix4x4Float(Wrapper::new([0.0f32; 16])),
                    _ => MaterialTarget::None,
                },
                VectorSize::Quad => match rows {
                    VectorSize::Quad => MaterialTarget::Matrix4x4Float(Wrapper::new([0.0f32; 16])),
                    _ => MaterialTarget::None,
                },
            },
            //TypeInner::Image { .. } => MaterialTarget::Texture(&"".to_string().into_bytes()),
            TypeInner::Sampler { comparison } => {
                MaterialTarget::Sampler(Wrapper::new(*comparison as u8))
            }
            _ => MaterialTarget::None,
        }
    }
}
