use crate::material::MaterialTargetFormat;
use naga::{ScalarKind, TypeInner, VectorSize};
use serde::*;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum EmptyTarget {
    None,
}

impl From<EmptyTarget> for MaterialTargetFormat {
    fn from(_: EmptyTarget) -> Self {
        MaterialTargetFormat::Undefined
    }
}

//todo future support for array and struct types.
#[repr(C)]
#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum MaterialTarget {
    None,
    //Aligned
    Uint(u32),
    Sint(i32),
    Float(f32),
    Bool(bool),

    Vec2UInt([u32; 2]),
    Vec2SInt([i32; 2]),
    Vec2Float([f32; 2]),
    Vec2Boolean([bool; 2]),

    Vec4UInt([u32; 4]),
    Vec4SInt([i32; 4]),
    Vec4Float([f32; 4]),
    Vec4Boolean([bool; 4]),

    //Column Major Matrix
    Matrix2x2([f32; 4]),
    Matrix4x4([f32; 16]),

    Sampler(bool), // 1 byte

                   //UnAligned
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
            MaterialTarget::Bool(_) => MaterialTargetFormat::Boolean,
            MaterialTarget::Vec2UInt(_) => MaterialTargetFormat::Vector2UnsignedInt,
            MaterialTarget::Vec2SInt(_) => MaterialTargetFormat::Vector2SignedInt,
            MaterialTarget::Vec2Float(_) => MaterialTargetFormat::Vector2Float,
            MaterialTarget::Vec2Boolean(_) => MaterialTargetFormat::Vector2Boolean,
            MaterialTarget::Vec4UInt(_) => MaterialTargetFormat::Vector4UnsignedInt,
            MaterialTarget::Vec4SInt(_) => MaterialTargetFormat::Vector4SignedInt,
            MaterialTarget::Vec4Float(_) => MaterialTargetFormat::Vector4Float,
            MaterialTarget::Vec4Boolean(_) => MaterialTargetFormat::Vector4Boolean,
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
                ScalarKind::Sint => MaterialTarget::Sint(0),
                ScalarKind::Uint => MaterialTarget::Uint(0),
                ScalarKind::Float => MaterialTarget::Float(0.0),
                ScalarKind::Bool => MaterialTarget::Bool(false),
            },
            TypeInner::Vector { size, kind, .. } => match size {
                VectorSize::Bi => match kind {
                    ScalarKind::Sint => MaterialTarget::Vec2SInt([0i32; 2]),
                    ScalarKind::Uint => MaterialTarget::Vec2UInt([0u32; 2]),
                    ScalarKind::Float => MaterialTarget::Vec2Float([0.0f32; 2]),
                    ScalarKind::Bool => MaterialTarget::Vec2Boolean([false; 2]),
                },
                VectorSize::Tri => match kind {
                    //Aligned data only. Any Tri-Vector will result to a Tri-Vector that is extend by 1 to a Quad-Vector.
                    ScalarKind::Sint => MaterialTarget::Vec4SInt([0i32; 4]),
                    ScalarKind::Uint => MaterialTarget::Vec4UInt([0u32; 4]),
                    ScalarKind::Float => MaterialTarget::Vec4Float([0.0f32; 4]),
                    ScalarKind::Bool => MaterialTarget::Vec4Boolean([false; 4]),
                },
                VectorSize::Quad => match kind {
                    ScalarKind::Sint => MaterialTarget::Vec4SInt([0i32; 4]),
                    ScalarKind::Uint => MaterialTarget::Vec4UInt([0u32; 4]),
                    ScalarKind::Float => MaterialTarget::Vec4Float([0.0f32; 4]),
                    ScalarKind::Bool => MaterialTarget::Vec4Boolean([false; 4]),
                },
            },
            TypeInner::Matrix { columns, rows, .. } => match columns {
                VectorSize::Bi => match rows {
                    VectorSize::Bi => MaterialTarget::Matrix2x2([0.0f32; 4]),
                    _ => MaterialTarget::None,
                },
                VectorSize::Tri => match rows {
                    //Aligned data only. Any Tri-Matrix will result to a 4x4 Matrix where the last row and column are zero-ed out
                    VectorSize::Tri => MaterialTarget::Matrix4x4([0.0f32; 16]),
                    _ => MaterialTarget::None,
                },
                VectorSize::Quad => match rows {
                    VectorSize::Quad => MaterialTarget::Matrix4x4([0.0f32; 16]),
                    _ => MaterialTarget::None,
                },
            },
            //TypeInner::Image { .. } => MaterialTarget::Texture(&"".to_string().into_bytes()),
            TypeInner::Sampler { comparison } => MaterialTarget::Sampler(*comparison),
            _ => MaterialTarget::None,
        }
    }
}
