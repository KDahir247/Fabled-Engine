use crate::material::{EmptyNode, MaterialNode, MaterialTarget};
use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum Attributes {
    Scalar(Vec<MaterialNode>),
    Vector(Vec<MaterialNode>),
    Matrix(Vec<MaterialNode>),
    Pointer(Vec<EmptyNode>),
    ValuePointer(Vec<EmptyNode>),
    Array(Vec<EmptyNode>),
    Struct(Vec<EmptyNode>),
    Image(Vec<MaterialNode>),
    Sampler(Vec<MaterialNode>),
}
