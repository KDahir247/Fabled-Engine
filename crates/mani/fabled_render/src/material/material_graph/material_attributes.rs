use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
pub enum MaterialAttributes {
    Scalar = 0,
    Vector = 1,
    Matrix = 2,
    Image = 3,
    Sampler = 4,
    Array = 5,
}
