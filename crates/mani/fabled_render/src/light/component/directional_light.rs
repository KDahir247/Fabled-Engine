use fabled_component::{All, Component};
use std::fmt::{Display, Formatter};

// For direction light we will set the illuminance directly
// since calculating the illuminance (Luminance flux / spherical radius *
// spherical radius) and the radius extent closely to infinite

// Directional light must have a illuminance, rotation.
// Optional Parameters: Color (treated as tint), Temperature, Shadow Parameters.

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct DirectionalLight {
    pub illuminance: f32,
}


impl Default for DirectionalLight {
    fn default() -> Self {
        Self {
            illuminance: 20000.0,
        }
    }
}


impl Display for DirectionalLight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DirectionLight(illuminance(lux) : {})", self.illuminance)
    }
}

impl DirectionalLight {
    pub fn new(lux: f32) -> Self {
        Self { illuminance: lux }
    }
}

// we need to track all (modification, deletion and creation)
impl Component for DirectionalLight {
    type Tracking = All;
}
