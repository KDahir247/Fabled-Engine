use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum MaterialAttributes {
    Empty = -1000,
    Scalar = 0,
    Vector = 1,
    Matrix = 2,
    Image = 3,
    Sampler = 4,
    Array = 5,
}
