use fabled_component::{All, Component};

use crate::light::{
    ev_to_candela, point_light_candela_to_lumen, point_light_lux_to_lumen, IntensityUnit,
};
// Approximation of illuminance to pass to shader.
// luminance flux / (4 * pi * radius * radius)
// Physically correct illuminance to pass to the shader
// luminance flux / (distance(light position, each point being shaded))
// We will substitute the denominator with a attenuation function to decrease to
// zero at a distance.
// luminance flux * attenuation fn

// We will still pass the radius and use it as a check between the distance from
// the light position to the point being shaded. if it is less the the point
// light radius we can assume that the light will not hit the point and we will
// not added lighting to it otherwise add lighting and attenuation function.

// Point light must have a intensity, radius, rotation, position
// translation. Optional Parameters: Color (treated as tint), Temperature,
// Shadow Parameters.

// Intensity is Luminance Power (Luminance flux) in lumen

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct PointLight {
    pub intensity: f32,
    pub radius: f32,
}

impl Default for PointLight {
    fn default() -> Self {
        Self {
            intensity: 40000.0,
            radius: 10.0,
        }
    }
}

impl PointLight {
    pub fn new(light_intensity: f32, light_intensity_type: IntensityUnit, radius: f32) -> Self {
        let intensity = match light_intensity_type {
            IntensityUnit::Candela => point_light_candela_to_lumen(light_intensity),
            IntensityUnit::Lux => point_light_lux_to_lumen(light_intensity, radius),
            IntensityUnit::EV100 {
                iso,
                calibration_constant,
            } => {
                let luminance_intensity = ev_to_candela(light_intensity, iso, calibration_constant);
                point_light_candela_to_lumen(luminance_intensity)
            }
            IntensityUnit::Lumen => light_intensity,
        };

        Self { intensity, radius }
    }

    pub fn illuminance_interior(&self) -> f32 {
        self.intensity / self.radius.powf(2.0)
    }
}

impl Component for PointLight {
    type Tracking = All;
}
