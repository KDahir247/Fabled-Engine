use crate::color::{hsl_to_rgb, hsv_to_rgb, srgb_to_cmyk, ColorSpace};
use fabled_component::{Component, Modification};
use fabled_math::{Vector3, Vector4};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub struct ACESCompression {
    // Distance from achromatic which will be compressed to the gamut boundary
    pub limit: Vector3,
    // Percentage of the core gamut to protect
    // Aggressiveness of the compression curve in W
    pub threshold_power: Vector4,
}

impl Default for ACESCompression {
    fn default() -> Self {
        Self {
            limit: Vector3::set(1.147, 1.264, 1.312),
            threshold_power: Vector4::set(0.815, 0.803, 0.88, 1.2),
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

        let threshold = Vector4::set(threshold_cmy.x(), threshold_cmy.y(), threshold.z(), power);

        ACESCompression {
            limit: limit_cmy,
            threshold_power: threshold,
        }
    }
}

impl Display for ACESCompression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ACESCompression(limit_cmy: {}, threshold_cmy: {}, power: {})",
            self.limit,
            self.threshold_power.trunc_vec3(),
            self.threshold_power.w()
        )
    }
}

impl Component for ACESCompression {
    type Tracking = Modification;
}
