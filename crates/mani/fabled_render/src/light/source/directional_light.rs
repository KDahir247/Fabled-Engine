// For direction light we will set the illuminance directly
// since calculating the illuminance (Luminance flux / spherical radius *
// spherical radius) and the radius extent closely to infinite

use crate::light::LightAppearance;

pub struct DirectionalLight {
    pub appearance: LightAppearance,
    pub illuminance: f32,
}


impl Default for DirectionalLight {
    fn default() -> Self {
        Self {
            illuminance: 130000.0,
            appearance: LightAppearance::default(),
        }
    }
}


impl DirectionalLight {
    pub fn new(lux: f32, appearance: LightAppearance) -> Self {
        Self {
            illuminance: lux,
            appearance,
        }
    }
}
