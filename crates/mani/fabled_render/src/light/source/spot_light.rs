use crate::light::{
    ev_to_candela, lux_to_candela, spot_light_candela_to_lumen, LightAppearance, LightUnit,
};

// Intensity is Luminance Power (Luminance flux) in lumen
pub struct SpotLight {
    pub appearance: LightAppearance,
    pub intensity: f32,
    pub range: f32,
    pub inner_cone: f32,
    pub outer_cone: f32,
    pub distance_m: f32,
}

impl Default for SpotLight {
    fn default() -> Self {
        Self {
            intensity: 40000.0,
            range: 10.0,
            inner_cone: 0.0,
            outer_cone: 45.0f32.to_radians(),
            appearance: LightAppearance::default(),
            distance_m: 10.0,
        }
    }
}

impl SpotLight {
    pub fn new(
        intensity: f32,
        unit_type: LightUnit,
        range: f32,
        inner: f32,
        outer: f32,
        appearance: LightAppearance,
        distance_m: f32, // Used for conversion between lux to candela.
    ) -> Self {
        // the inner angle can't be greater then the outer angle.
        let inner_safe = inner.min(outer - 0.01f32);

        // Convert it to lumen (luminance power (luminance flux))
        let mut unit_value = intensity;

        match unit_type {
            LightUnit::Candela => {
                unit_value = spot_light_candela_to_lumen(unit_value, outer);
            }
            LightUnit::Lux => {
                let luminance_intensity = lux_to_candela(unit_value, distance_m);
                unit_value = spot_light_candela_to_lumen(luminance_intensity, outer);
            }
            LightUnit::EV100 {
                iso,
                calibration_constant,
            } => {
                let luminance_intensity = ev_to_candela(unit_value, iso, calibration_constant);
                unit_value = spot_light_candela_to_lumen(luminance_intensity, outer);
            }
            _ => {} // Already lumen (luminance power (luminance flux))
        }

        Self {
            intensity: unit_value,
            range,
            inner_cone: inner_safe,
            outer_cone: outer,
            appearance,
            distance_m,
        }
    }
}
