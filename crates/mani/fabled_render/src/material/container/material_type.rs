use crate::material::{PBRStandardMaterial, StandardMaterial};

#[derive(Copy, Clone, Debug)]
pub enum MaterialType {
    Standard(StandardMaterial),
    PBR(PBRStandardMaterial),
}
