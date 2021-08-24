use serde::{Deserialize, Serialize};

//todo future support for array
#[derive(Debug, Serialize, Deserialize)]
pub enum Attributes {
    Scalar,
    Vector,
    Matrix,
    Image,
    Sampler,
    //Array,
    //Struct
}
