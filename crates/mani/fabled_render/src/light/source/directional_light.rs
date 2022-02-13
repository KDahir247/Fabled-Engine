// For direction light we will set the illuminance directly
// since calculating the illuminance (Luminance flux / spherical radius *
// spherical radius) and the radius extent closely to infinite

pub struct DirectionalLight {
    pub illuminance: f32,
}


impl Default for DirectionalLight {
    fn default() -> Self {
        Self {
            illuminance: 130000.0,
        }
    }
}


impl DirectionalLight {
    pub fn new(lux: f32) -> Self {
        Self {
            illuminance: lux,
        }
    }
}
