#[derive(Copy, Clone, Debug)]
pub enum FishLens {
    // Normal
    Rectilinear,
    // Fish eyes
    Stereographic,
    Equidistant,
    EquisolidAngle,
    Orthographic,
}


impl Default for FishLens {
    fn default() -> Self {
        Self::Rectilinear
    }
}
