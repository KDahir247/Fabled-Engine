use crate::light::{
    ev_to_candela, lux_to_candela, point_light_candela_to_lumen, LightAppearance, LightUnit,
};

// Intensity is Luminance Power (Luminance flux) in lumen
pub struct PointLight {
    pub appearance: LightAppearance,
    pub intensity: f32,
    pub radius: f32,
    pub range: f32,
    pub distance_m: f32,
}

impl Default for PointLight {
    fn default() -> Self {
        Self {
            intensity: 40000.0,
            radius: 10.0,
            range: 10.0,
            appearance: LightAppearance::default(),
            distance_m: 10.0,
        }
    }
}

impl PointLight {
    pub fn new(
        intensity: f32,
        unit_type: LightUnit,
        radius: f32,
        range: f32,
        appearance: LightAppearance,
        distance_m: f32,
    ) -> Self {
        let mut unit_value = intensity;

        // Convert it to lumen (luminance power (luminance flux))
        match unit_type {
            LightUnit::Candela => {
                unit_value = point_light_candela_to_lumen(unit_value);
            }
            LightUnit::Lux => {
                let luminance_intensity = lux_to_candela(unit_value, distance_m);
                unit_value = point_light_candela_to_lumen(luminance_intensity);
            }
            LightUnit::EV100 {
                iso,
                calibration_constant,
            } => {
                let luminance_intensity = ev_to_candela(unit_value, iso, calibration_constant);
                unit_value = point_light_candela_to_lumen(luminance_intensity);
            }
            LightUnit::Lumen => {} // Already lumen (luminance power (luminance flux))
        }

        Self {
            intensity: unit_value,
            radius,
            range,
            appearance,
            distance_m,
        }
    }

    pub fn illuminance_interior(&self) -> f32 {
        self.intensity / self.radius.powf(2.0)
    }
}
