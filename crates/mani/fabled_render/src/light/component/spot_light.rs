use fabled_component::{All, Component};

use crate::light::{ev_to_candela, lux_to_candela, spot_light_candela_to_lumen, IntensityUnit};

// Spot light must have a intensity, radius, rotation, position,
// translation. Optional Parameters: color (treated as tint), temperature,
// Shadow Parameters.


// Intensity is Luminance Power (Luminance flux) in lumen
// We will need to pass it as Luminance Intensity to the shader to calculate the target illuminance.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct SpotLight {
    pub intensity: f32,
    pub radius: f32,
    pub inner_cone: f32,
    pub outer_cone: f32,
}

impl Default for SpotLight {
    fn default() -> Self {
        Self {
            intensity: 40000.0,
            radius: 10.0,
            inner_cone: 0.0,
            outer_cone: 45.0f32.to_radians(),
        }
    }
}

impl SpotLight {
    pub fn new(
        light_intensity: f32,
        light_intensity_type: IntensityUnit,
        radius: f32,
        inner: f32,
        outer: f32,
    ) -> Self {
        // the inner angle can't be greater then the outer angle.
        let inner_safe = inner.min(outer - 0.01f32);

        let intensity = match light_intensity_type {
            IntensityUnit::Candela => spot_light_candela_to_lumen(light_intensity, outer),
            IntensityUnit::Lux  => {
                let luminous_intensity = lux_to_candela(light_intensity, radius);
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
            radius,
            inner_cone: inner_safe,
            outer_cone: outer,
        }
    }

    // Calculate spot scale for spot attenuation.
    pub fn spot_scale(self) -> f32{
        let cos_outer = f32::cos(self.outer_cone);

        1.0 / f32::max(f32::cos(self.inner_cone) - cos_outer, 0.0001)
    }


    // Calculate the spot offset for spot attenuation.
    pub fn spot_offset(self) -> f32{
        let cos_outer = f32::cos(self.outer_cone);

        -cos_outer  * self.spot_scale()

    }
}

impl Component for SpotLight {
    type Tracking = All;
}
