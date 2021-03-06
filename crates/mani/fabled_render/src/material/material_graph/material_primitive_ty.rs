use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
pub enum MaterialPrimitiveType {
    Scalar = 0,
    Vector = 1,
    Matrix = 2,
    Image = 3,
    Sampler = 4,
    Array = 5,
    Struct = 6,
}
