use crate::color::{hsl_to_rgb, hsv_to_rgb, srgb_to_cmyk, ColorSpace};
use fabled_component::{Component, Modification};
use fabled_math::Vector3;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub struct ACESCompression {
    // Distance from achromatic which will be compressed to the gamut boundary
    pub limit_cmy: Vector3,
    // Percentage of the core gamut to protect
    pub threshold_cmy: Vector3,
    // Aggressiveness of the compression curve
    pub power: f32,
}

impl Default for ACESCompression {
    fn default() -> Self {
        Self {
            limit_cmy: Vector3::set(1.147, 1.264, 1.312),
            threshold_cmy: Vector3::set(0.815, 0.803, 0.88),
            power: 1.2,
        }
    }
}

impl ACESCompression {
    pub fn new(limit: Vector3, threshold: Vector3, power: f32, space: ColorSpace) -> Self {
        let limit_cmy = match space {
            ColorSpace::RGB => srgb_to_cmyk(limit).trunc_vec3(),
            ColorSpace::HSL => srgb_to_cmyk(hsl_to_rgb(limit)).trunc_vec3(),
            ColorSpace::HSV => srgb_to_cmyk(hsv_to_rgb(limit)).trunc_vec3(),
            ColorSpace::CMY => limit,
        };


        let threshold_cmy = match space {
            ColorSpace::RGB => srgb_to_cmyk(threshold).trunc_vec3(),
            ColorSpace::HSL => srgb_to_cmyk(hsl_to_rgb(threshold)).trunc_vec3(),
            ColorSpace::HSV => srgb_to_cmyk(hsv_to_rgb(threshold)).trunc_vec3(),
            ColorSpace::CMY => threshold,
        };

        ACESCompression {
            limit_cmy,
            threshold_cmy,
            power,
        }
    }
}

impl Display for ACESCompression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ACESCompression(limit_cmy: {}, threshold_cmy: {}, power: {})",
            self.limit_cmy, self.threshold_cmy, self.power
        )
    }
}

impl Component for ACESCompression {
    type Tracking = Modification;
}
