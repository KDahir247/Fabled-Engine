use crate::material::MaterialTargetFormat;

use fabled_core::prime::container::wrapper::Wrapper;

use naga::{ScalarKind, TypeInner, VectorSize};

use serde::*;

use fabled_core::prime::container::primitive::Primitive;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum EmptyTarget {
    None,
}

impl From<EmptyTarget> for MaterialTargetFormat {
    fn from(_: EmptyTarget) -> Self {
        MaterialTargetFormat::Undefined
    }
}

#[rustfmt::skip]
//this will extend primitives to support sampler and texture.
#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum MaterialTarget {
    None,
    //Aligned
    Uint(Wrapper<u32>),
    Sint(Wrapper<i32>),
    Float(Wrapper<f32>),

    Vec2UInt(Wrapper<[u32; 2]>),
    Vec2SInt(Wrapper<[i32; 2]>),
    Vec2Float(Wrapper<[f32; 2]>),

    Vec4UInt(Wrapper<[u32; 4]>),
    Vec4SInt(Wrapper<[i32; 4]>),
    Vec4Float(Wrapper<[f32; 4]>),

    //Column Major Matrix
    Matrix2x2(Wrapper<[f32; 4]>),
    Matrix4x4(Wrapper<[f32; 16]>),

    Sampler(Wrapper<u8>), // 1 byte

    //UnAligned
    // Texture should store an Option of Tiling and Offset and a way to reference the texture.
    //todo replay with a POD (Plain old data type) that is aligned to the power of two. rather than a string.
    //todo got to find a better identifier for this type.
    //Texture(&'static [u8]), //), // 24 bytes
}

impl From<MaterialTarget> for MaterialTargetFormat {
    fn from(target: MaterialTarget) -> Self {
        match target {
            MaterialTarget::None => MaterialTargetFormat::Undefined,
            MaterialTarget::Uint(_) => MaterialTargetFormat::UnsignedInt,
            MaterialTarget::Sint(_) => MaterialTargetFormat::SignedInt,
            MaterialTarget::Float(_) => MaterialTargetFormat::Float,
            MaterialTarget::Vec2UInt(_) => MaterialTargetFormat::Vector2UnsignedInt,
            MaterialTarget::Vec2SInt(_) => MaterialTargetFormat::Vector2SignedInt,
            MaterialTarget::Vec2Float(_) => MaterialTargetFormat::Vector2Float,
            MaterialTarget::Vec4UInt(_) => MaterialTargetFormat::Vector4UnsignedInt,
            MaterialTarget::Vec4SInt(_) => MaterialTargetFormat::Vector4SignedInt,
            MaterialTarget::Vec4Float(_) => MaterialTargetFormat::Vector4Float,
            MaterialTarget::Matrix2x2(_) => MaterialTargetFormat::Matrix2x2Float,
            MaterialTarget::Matrix4x4(_) => MaterialTargetFormat::Matrix4x4Float,
            MaterialTarget::Sampler(_) => MaterialTargetFormat::Sampler,
            //MaterialTarget::Texture(_) => MaterialTargetFormat::Texture,
        }
    }
}

impl From<&naga::TypeInner> for MaterialTarget {
    fn from(type_var: &naga::TypeInner) -> Self {
        match type_var {
            TypeInner::Scalar { kind, .. } => match kind {
                ScalarKind::Sint => MaterialTarget::Sint(Wrapper::new(0)),
                ScalarKind::Uint => MaterialTarget::Uint(Wrapper::new(0)),
                ScalarKind::Float => MaterialTarget::Float(Wrapper::new(0.0)),
                _ => MaterialTarget::None,
            },
            TypeInner::Vector { size, kind, .. } => match size {
                VectorSize::Bi => match kind {
                    ScalarKind::Sint => MaterialTarget::Vec2SInt(Wrapper::new([0i32; 2])),
                    ScalarKind::Uint => MaterialTarget::Vec2UInt(Wrapper::new([0u32; 2])),
                    ScalarKind::Float => MaterialTarget::Vec2Float(Wrapper::new([0.0f32; 2])),
                    _ => MaterialTarget::None,
                },
                VectorSize::Tri => match kind {
                    //Aligned data only. Any Tri-Vector will result to a Tri-Vector that is extend by 1 to a Quad-Vector.
                    ScalarKind::Sint => MaterialTarget::Vec4SInt(Wrapper::new([0i32; 4])),
                    ScalarKind::Uint => MaterialTarget::Vec4UInt(Wrapper::new([0u32; 4])),
                    ScalarKind::Float => MaterialTarget::Vec4Float(Wrapper::new([0.0f32; 4])),
                    _ => MaterialTarget::None,
                },
                VectorSize::Quad => match kind {
                    ScalarKind::Sint => MaterialTarget::Vec4SInt(Wrapper::new([0i32; 4])),
                    ScalarKind::Uint => MaterialTarget::Vec4UInt(Wrapper::new([0u32; 4])),
                    ScalarKind::Float => MaterialTarget::Vec4Float(Wrapper::new([0.0f32; 4])),
                    _ => MaterialTarget::None,
                },
            },
            TypeInner::Matrix { columns, rows, .. } => match columns {
                VectorSize::Bi => match rows {
                    VectorSize::Bi => MaterialTarget::Matrix2x2(Wrapper::new([0.0f32; 4])),
                    _ => MaterialTarget::None,
                },
                VectorSize::Tri => match rows {
                    //Aligned data only. Any Tri-Matrix will result to a 4x4 Matrix where the last row and column are zero-ed out
                    VectorSize::Tri => MaterialTarget::Matrix4x4(Wrapper::new([0.0f32; 16])),
                    _ => MaterialTarget::None,
                },
                VectorSize::Quad => match rows {
                    VectorSize::Quad => MaterialTarget::Matrix4x4(Wrapper::new([0.0f32; 16])),
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
