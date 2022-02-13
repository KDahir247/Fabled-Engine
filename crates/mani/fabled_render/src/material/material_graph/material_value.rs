use crate::material::{MaterialPrimitiveType, MaterialValueType};
use bytemuck::{Pod, Zeroable};
use fabled_core::prime::container::wrapper::Wrapper;
use naga::{ScalarKind, TypeInner, VectorSize};
use serde::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialValue {
    pub value_ty: MaterialValueType,
    pub value_bytes: Box<[u8]>,
}

impl MaterialValue {
    pub fn new<N: Pod + Zeroable>(value_ty: MaterialValueType, value: N) -> Self {
        Self {
            value_ty,
            value_bytes: Wrapper::new(value).get_bytes().into_boxed_slice(),
        }
    }

    pub fn get_primitive_ty(&self) -> Option<MaterialPrimitiveType> {
        self.value_ty.into()
    }
}


impl From<&naga::TypeInner> for MaterialValue {
    fn from(ty: &TypeInner) -> Self {
        match ty {
            TypeInner::Scalar { kind, .. } => match kind {
                ScalarKind::Sint => MaterialValue::new(MaterialValueType::SignedInt, 0_u32),
                ScalarKind::Uint => MaterialValue::new(MaterialValueType::UnsignedInt, 0_i32),
                ScalarKind::Float => MaterialValue::new(MaterialValueType::Float, 0.0_f32),
                ScalarKind::Bool => MaterialValue::new(MaterialValueType::None, 0_u32),
            },

            TypeInner::Vector { size, kind, .. } => match kind {
                ScalarKind::Sint => match size {
                    VectorSize::Bi => MaterialValue::new(MaterialValueType::Vec2Signed, [0_i32; 2]),
                    VectorSize::Tri => {
                        MaterialValue::new(MaterialValueType::Vec3Signed, [0_i32; 3])
                    }
                    VectorSize::Quad => {
                        MaterialValue::new(MaterialValueType::Vec4Signed, [0_i32; 4])
                    }
                },
                ScalarKind::Uint => match size {
                    VectorSize::Bi => {
                        MaterialValue::new(MaterialValueType::Vec2Unsigned, [0_u32; 2])
                    }
                    VectorSize::Tri => {
                        MaterialValue::new(MaterialValueType::Vec3Unsigned, [0_u32; 3])
                    }
                    VectorSize::Quad => {
                        MaterialValue::new(MaterialValueType::Vec4Unsigned, [0_u32; 4])
                    }
                },
                ScalarKind::Float => match size {
                    VectorSize::Bi => MaterialValue::new(MaterialValueType::Vec2Float, [0_f32; 2]),
                    VectorSize::Tri => MaterialValue::new(MaterialValueType::Vec3Float, [0_f32; 3]),
                    VectorSize::Quad => {
                        MaterialValue::new(MaterialValueType::Vec4Float, [0_f32; 4])
                    }
                },
                _ => MaterialValue::new(MaterialValueType::None, 0_u32),
            },
            TypeInner::Matrix { columns, rows, .. } => {
                match rows {
                    VectorSize::Bi => match columns {
                        VectorSize::Bi => {
                            MaterialValue::new(MaterialValueType::Mat2Float, [0.0f32; 4])
                        }
                        _ => MaterialValue::new(MaterialValueType::None, 0_u32),
                    },
                    VectorSize::Tri => match columns {
                        VectorSize::Tri => {
                            MaterialValue::new(MaterialValueType::Mat3Float, [0.0f32; 9])
                        }
                        _ => MaterialValue::new(MaterialValueType::None, 0_u32),
                    },
                    VectorSize::Quad => match columns {
                        VectorSize::Quad => {
                            MaterialValue::new(MaterialValueType::Mat4Float, [0.0f32; 16])
                        }
                        _ => MaterialValue::new(MaterialValueType::None, 0_u32),
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
                MaterialValue::new(MaterialValueType::Sampler, *comparison as u32)
            }
            _ => MaterialValue::new(MaterialValueType::None, 0_u32),
        }
    }
}
