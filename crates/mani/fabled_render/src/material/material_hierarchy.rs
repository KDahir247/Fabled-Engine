use naga::{ScalarKind, TypeInner, VectorSize};
use serde::*;

//todo data not aligned.

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
    Uint(u32),  //4 bytes
    Sint(i32),  //4 bytes
    Float(f32), //4 bytes
    Bool(bool), //1 byte

    Vec2SInt([i32; 2]),     //8 bytes
    Vec2UInt([u32; 2]),     //8 bytes
    Vec2Float([f32; 2]),    //8 bytes
    Vec2Boolean([bool; 2]), //2 bytes

    Vec4SInt([i32; 4]),     // 16 bytes
    Vec4UInt([u32; 4]),     // 16 bytes
    Vec4Float([f32; 4]),    // 16 bytes
    Vec4Boolean([bool; 4]), // 4 bytes

    //Column Major Matrix
    Matrix2x2([f32; 4]),  // 16 bytes
    Matrix4x4([f32; 16]), // 64 bytes

    Sampler(bool), // 1 byte
    //

    //UnAligned
    //todo replay with a POD (Plain old data type) that is aligned to the power of two. rather than a string.
    Texture(String), // 24 bytes
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

#[cfg(test)]
mod data_alignment_test {
    use crate::material::{MaterialLeaf, MaterialNode, MaterialTarget, MaterialTree};

    #[test]
    fn data_alignment() {
        let material_tree = std::mem::size_of::<MaterialTree>();
        println!("{}", material_tree);

        let material_leaf = std::mem::size_of::<MaterialLeaf>();
        println!("{}", material_leaf);

        let material_node = std::mem::size_of::<MaterialNode>();
        println!("{}", material_node);

        let material_target = std::mem::size_of::<MaterialTarget>();
        println!("{}", material_target);
    }
}
