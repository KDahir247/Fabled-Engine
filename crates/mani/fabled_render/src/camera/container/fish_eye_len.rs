use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq)]
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

impl Display for FishLens {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string_repr = match self {
            FishLens::Rectilinear => "Rectilinear",
            FishLens::Stereographic => "Stereographic",
            FishLens::Equidistant => "Equidistant",
            FishLens::EquisolidAngle => "EquisolidAngle",
            FishLens::Orthographic => "Orthographic",
        };

        f.write_str(string_repr)
    }
}
