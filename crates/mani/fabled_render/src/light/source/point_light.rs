pub struct PointLight {
    pub intensity: f32,
    pub radius: f32,
    pub range: f32,
    pub color: [f32; 3],
}

impl Default for PointLight {
    fn default() -> Self {
        Self {
            intensity: 10.0,
            radius: 10.0,
            range: 10.0,
            color: [1.0; 3],
        }
    }
}

impl PointLight {
    // flux represents Luminous flux of the light.
    // Luminous flux is how much light a light source emits.
    pub fn new(flux: f32, radius: f32, range: f32, color: [f32; 3]) -> Self {
        // We need the luminous intensity to determine how light is traveling in a
        // certain direction.
        let luminance_intensity = flux / (4.0 * std::f32::consts::PI);
        Self {
            intensity: luminance_intensity,
            radius,
            range,
            color,
        }
    }

    pub fn illuminance_interior(&self) -> f32 {
        self.intensity / self.radius.powf(2.0)
    }
}
