use crate::material::{PBRStandardMaterial, StandardMaterial};

// todo remove we will go to the meta data serialize and deserialize.
#[derive(Copy, Clone, Debug)]
pub enum MaterialType {
    Standard(StandardMaterial),
    PBR(PBRStandardMaterial),
}
