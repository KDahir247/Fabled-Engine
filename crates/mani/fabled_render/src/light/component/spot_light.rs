use fabled_component::{All, Component};
use fabled_math::Vector4;
use std::fmt::{Display, Formatter};

use crate::light::{
    ev_to_candela, lux_to_candela, spot_light_candela_to_lumen, IntensityUnit, Source,
};

// Spot light must have a intensity, radius, rotation, position,
// translation. Optional Parameters: color (treated as tint), temperature,
// Shadow Parameters.

// Perspective projection
// Intensity is Luminance Power (Luminance flux) in lumen
// We will need to pass it as Luminance Intensity to the shader to calculate the
// target illuminance.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct SpotLight {
    pub value: Vector4,
}

impl Default for SpotLight {
    fn default() -> Self {
        Self {
            value: Vector4::set(4000.0, 10.0, 0.0, 45.0f32.to_radians()),
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
    ) -> Self {
        // the inner angle can't be greater then the outer angle.
        let inner_safe = inner.min(outer - 0.01f32);

        let intensity = match light_intensity_type {
            IntensityUnit::Candela => spot_light_candela_to_lumen(light_intensity, outer),
            IntensityUnit::Lux => {
                let luminous_intensity = lux_to_candela(light_intensity, range);
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
            value: Vector4::set(intensity, range, inner_safe, outer),
        }
    }

    // Calculate spot scale for spot attenuation.
    pub fn spot_scale(self) -> f32 {
        let cos_outer = f32::cos(self.value.w());

        1.0 / f32::max(f32::cos(self.value.z()) - cos_outer, 0.0001)
    }


    // Calculate the spot offset for spot attenuation.
    pub fn spot_offset(self) -> f32 {
        let cos_outer = f32::cos(self.value.w());

        -cos_outer * self.spot_scale()
    }
}

impl Source for SpotLight {}

impl Component for SpotLight {
    type Tracking = All;
}

impl Display for SpotLight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SpotLight(\n\tLuminance flux : {}\n\tRange : {}\n\tInner Angle : {}\n\tOutAngle : {}\n)",
                 self.value.x(), self.value.y(), self.value.z(), self.value.w()
        )
    }
}
