use crate::light::{
    ev_to_candela, lux_to_candela, point_light_candela_to_lumen, LightAppearance, IntensityUnit,
};

// Intensity is Luminance Power (Luminance flux) in lumen
pub struct PointLight {
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
            distance_m: 10.0,
        }
    }
}

impl PointLight {
    pub fn new(
        light_intensity: f32,
        light_intensity_type: IntensityUnit,
        radius: f32,
        range: f32,
        distance_m: f32,
    ) -> Self {

        let intensity = match light_intensity_type {
            IntensityUnit::Candela => {
                point_light_candela_to_lumen(light_intensity)
            }
            IntensityUnit::Lux => {
                let luminance_intensity = lux_to_candela(light_intensity, distance_m);
                point_light_candela_to_lumen(luminance_intensity)
            }
            IntensityUnit::EV100 {
                iso,
                calibration_constant,
            } => {
                let luminance_intensity = ev_to_candela(light_intensity, iso, calibration_constant);
                point_light_candela_to_lumen(luminance_intensity)
            }
            IntensityUnit::Lumen => light_intensity
        };

        Self {
            intensity,
            radius,
            range,
            distance_m,
        }
    }


    pub fn illuminance_interior(&self) -> f32 {
        self.intensity / self.radius.powf(2.0)
    }
}
