pub struct PointLight {
    position: [f32; 4],
    pub intensity: f32,
    pub radius: f32,
}

impl PointLight {
    // flux represents Luminous flux of the light.
    // Luminous flux is how much light a light source emits.
    pub fn new(flux: f32, radius: f32) -> Self {
        // We need the luminous intensity to determine how light is traveling in a
        // certain direction.
        let luminance_intensity = flux / (4.0 * std::f32::consts::PI);
        Self {
            position: [0.0; 4],
            intensity: luminance_intensity,
            radius,
        }
    }

    pub fn illuminance_interior(&self) -> f32 {
        self.intensity / self.radius.powf(2.0)
    }
}
