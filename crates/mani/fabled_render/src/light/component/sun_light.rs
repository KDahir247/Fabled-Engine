use crate::light::Source;
use fabled_component::{All, Component};
use std::fmt::{write, Display, Formatter};

// For direction light we will set the illuminance directly
// since calculating the illuminance (Luminance flux / spherical radius *
// spherical radius) and the radius extent closely to infinite

// Orthographic projection
// Directional light must have a illuminance, rotation.
// Optional Parameters: Color (treated as tint), Temperature, Shadow Parameters.

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct SunLight {
    pub illuminance: f32,
    pub angle_rad: f32,
}


impl Default for SunLight {
    fn default() -> Self {
        Self {
            illuminance: 20000.0,
            angle_rad: 0.00951,
        }
    }
}

impl SunLight {}

impl Source for SunLight {}

impl Component for SunLight {
    type Tracking = All;
}

impl Display for SunLight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "SunLight(\n\tIlluminance : {}\n\tangle : {}\n)",
            self.illuminance, self.angle_rad,
        )
    }
}
