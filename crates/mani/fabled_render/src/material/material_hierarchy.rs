use naga::{ScalarKind, TypeInner, VectorSize};
use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialTree {
    pub material_name: String,
    pub shader: String,
    pub attributes: [MaterialLeaf; 9],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialLeaf {
    pub type_name: String,
    pub types: Vec<MaterialNode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialNode {
    pub value_name: String,
    pub value_type: naga::TypeInner,
    pub value_group: Option<u32>,
    pub value_binding: Option<u32>,
    pub value: MaterialTarget,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MaterialTarget {
    None,

    //Aligned
    Uint(u32),
    Sint(i32),
    Float(f32),
    Bool(bool),

    //Aligned
    Vec2SInt([i32; 2]),
    Vec2UInt([u32; 2]),
    Vec2Float([f32; 2]),
    Vec2Boolean([bool; 2]),

    //UnAligned
    Vec3SInt([i32; 3]),
    Vec3UInt([u32; 3]),
    Vec3Float([f32; 3]),
    Vec3Boolean([bool; 3]),

    //Aligned
    Vec4SInt([i32; 4]),
    Vec4UInt([u32; 4]),
    Vec4Float([f32; 4]),
    Vec4Boolean([bool; 4]),

    //Column Major Matrix
    //Aligned
    Matrix2x2([f32; 4]),
    Matrix4x4([f32; 16]),

    //UnAligned
    Matrix3x3([f32; 9]),
    Texture(String),

    //Aligned
    Sampler(bool),
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
                    ScalarKind::Sint => MaterialTarget::Vec3SInt([0i32; 3]),
                    ScalarKind::Uint => MaterialTarget::Vec3UInt([0u32; 3]),
                    ScalarKind::Float => MaterialTarget::Vec3Float([0.0f32; 3]),
                    ScalarKind::Bool => MaterialTarget::Vec3Boolean([false; 3]),
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
                    VectorSize::Tri => MaterialTarget::Matrix3x3([0.0f32; 9]),
                    _ => MaterialTarget::None,
                },
                VectorSize::Quad => match rows {
                    VectorSize::Quad => MaterialTarget::Matrix4x4([0.0f32; 16]),
                    _ => MaterialTarget::None,
                },
            },
            TypeInner::Image { .. } => MaterialTarget::Texture("".to_string()),
            TypeInner::Sampler { comparison } => MaterialTarget::Sampler(*comparison),
            _ => MaterialTarget::None,
        }
    }
}

impl MaterialTarget {
    pub fn index(target: &naga::TypeInner) -> usize {
        match target {
            TypeInner::Scalar { .. } => 0,
            TypeInner::Vector { .. } => 1,
            TypeInner::Matrix { .. } => 2,
            TypeInner::Pointer { .. } => 3,
            TypeInner::ValuePointer { .. } => 4,
            TypeInner::Array { .. } => 5,
            TypeInner::Struct { .. } => 6,
            TypeInner::Image { .. } => 7,
            TypeInner::Sampler { .. } => 8,
        }
    }
}
