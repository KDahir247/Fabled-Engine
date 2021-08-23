use crate::material::MaterialKey;
use serde::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum Attributes {
    Scalar(Vec<MaterialKey>),
    Vector(Vec<MaterialKey>),
    Matrix(Vec<MaterialKey>),
    Image(Vec<MaterialKey>),
    Sampler(Vec<MaterialKey>),
}
