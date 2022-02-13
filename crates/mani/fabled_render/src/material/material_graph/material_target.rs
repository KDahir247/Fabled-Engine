use crate::material::MaterialValueType;
use bytemuck::{Pod, Zeroable};
use fabled_core::prime::container::wrapper::Wrapper;
use naga::{ScalarKind, TypeInner, VectorSize};
use serde::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialTarget {
    pub ty: MaterialValueType,
    pub value_bytes: Box<[u8]>,
}

impl MaterialTarget {
    pub fn new<N: Pod + Zeroable>(ty: MaterialValueType, value: N) -> Self {
        Self {
            ty,
            value_bytes: Wrapper::new(value).get_bytes().into_boxed_slice(),
        }
    }
}


impl From<&naga::TypeInner> for MaterialTarget {
    fn from(ty: &TypeInner) -> Self {
        match ty {
            TypeInner::Scalar { kind, .. } => match kind {
                ScalarKind::Sint => MaterialTarget::new(MaterialValueType::SignedInt, 0_u32),
                ScalarKind::Uint => MaterialTarget::new(MaterialValueType::UnsignedInt, 0_i32),
                ScalarKind::Float => MaterialTarget::new(MaterialValueType::Float, 0.0_f32),
                ScalarKind::Bool => MaterialTarget::new(MaterialValueType::None, 0_u32),
            },

            TypeInner::Vector { size, kind, .. } => match kind {
                ScalarKind::Sint => match size {
                    VectorSize::Bi => {
                        MaterialTarget::new(MaterialValueType::Vec2Signed, [0_i32; 2])
                    }
                    VectorSize::Tri => {
                        MaterialTarget::new(MaterialValueType::Vec3Signed, [0_i32; 3])
                    }
                    VectorSize::Quad => {
                        MaterialTarget::new(MaterialValueType::Vec4Signed, [0_i32; 4])
                    }
                },
                ScalarKind::Uint => match size {
                    VectorSize::Bi => {
                        MaterialTarget::new(MaterialValueType::Vec2Unsigned, [0_u32; 2])
                    }
                    VectorSize::Tri => {
                        MaterialTarget::new(MaterialValueType::Vec3Unsigned, [0_u32; 3])
                    }
                    VectorSize::Quad => {
                        MaterialTarget::new(MaterialValueType::Vec4Unsigned, [0_u32; 4])
                    }
                },
                ScalarKind::Float => match size {
                    VectorSize::Bi => MaterialTarget::new(MaterialValueType::Vec2Float, [0_f32; 2]),
                    VectorSize::Tri => {
                        MaterialTarget::new(MaterialValueType::Vec3Float, [0_f32; 3])
                    }
                    VectorSize::Quad => {
                        MaterialTarget::new(MaterialValueType::Vec4Float, [0_f32; 4])
                    }
                },
                _ => MaterialTarget::new(MaterialValueType::None, 0_u32),
            },
            TypeInner::Matrix { columns, rows, .. } => {
                match rows {
                    VectorSize::Bi => match columns {
                        VectorSize::Bi => {
                            MaterialTarget::new(MaterialValueType::Mat2Float, [0.0f32; 4])
                        }
                        _ => MaterialTarget::new(MaterialValueType::None, 0_u32),
                    },
                    VectorSize::Tri => match columns {
                        VectorSize::Tri => {
                            MaterialTarget::new(MaterialValueType::Mat3Float, [0.0f32; 9])
                        }
                        _ => MaterialTarget::new(MaterialValueType::None, 0_u32),
                    },
                    VectorSize::Quad => match columns {
                        VectorSize::Quad => {
                            MaterialTarget::new(MaterialValueType::Mat4Float, [0.0f32; 16])
                        }
                        _ => MaterialTarget::new(MaterialValueType::None, 0_u32),
                    },
                }
                // todo
            }
            // TypeInner::Array { base, size, stride } => {}
            // TypeInner::Image {
            //     dim,
            //     arrayed,
            //     class,
            // } => {}
            TypeInner::Sampler { comparison } => {
                MaterialTarget::new(MaterialValueType::Sampler, *comparison as u32)
            }
            _ => MaterialTarget::new(MaterialValueType::None, 0_u32),
        }
    }
}
