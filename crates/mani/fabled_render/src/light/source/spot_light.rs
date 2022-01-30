use crate::light::{
    ev_to_candela, lux_to_candela, spot_light_candela_to_lumen, IntensityUnit,
};

// Intensity is Luminance Power (Luminance flux) in lumen
pub struct SpotLight {
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
            distance_m: 10.0,
        }
    }
}

impl SpotLight {
    pub fn new(
        light_intensity: f32,
        light_intensity_type: IntensityUnit,
        range: f32,
        inner: f32,
        outer: f32,
        distance_m: f32,
    ) -> Self {
        // the inner angle can't be greater then the outer angle.
        let inner_safe = inner.min(outer - 0.01f32);

        let intensity = match light_intensity_type {
            IntensityUnit::Candela => spot_light_candela_to_lumen(light_intensity, outer),
            IntensityUnit::Lux => {
                let luminous_intensity = lux_to_candela(light_intensity, distance_m);
                spot_light_candela_to_lumen(luminous_intensity, outer)
            }
            IntensityUnit::EV100 {
                iso,
                calibration_constant,
            } => {
                let luminance_intensive = ev_to_candela(light_intensity, iso, calibration_constant);
                spot_light_candela_to_lumen(luminance_intensive, outer)
            }
            IntensityUnit::Lumen => light_intensity,
        };

        Self {
            intensity,
            range,
            inner_cone: inner_safe,
            outer_cone: outer,
            distance_m,
        }
    }
}
