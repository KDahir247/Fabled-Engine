// For direction light we will set the illuminance directly
// since calculating the illuminance (Luminance flux / spherical radius *
// spherical radius) and the radius extent closely to infinite

pub struct DirectionalLight {
    pub illuminance: f32,
    pub color: [f32; 3],
}


impl Default for DirectionalLight {
    fn default() -> Self {
        unimplemented!()
    }
}


impl DirectionalLight {
    pub fn new(lux: f32, color: [f32; 3]) -> Self {
        Self {
            illuminance: lux,
            color,
        }
    }
}
