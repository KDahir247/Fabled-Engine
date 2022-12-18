use crate::color::{SRGB_TO_XYZ_MATRIX, VON_KRIES, XYZ_TO_SRGB_MATRIX};
use fabled_component::{Component, Modification};
use fabled_math::Matrix3x3;

#[derive(Copy, Clone, PartialEq)]
pub struct ColorSpaceAdaption {
    // conversion matrix from color space XYZ
    pub tristimulus_matrix: Matrix3x3,
    // adaption matrix for color space
    pub adaption_matrix: Matrix3x3,
}

impl Default for ColorSpaceAdaption {
    fn default() -> Self {
        ColorSpaceAdaption {
            tristimulus_matrix: SRGB_TO_XYZ_MATRIX,
            adaption_matrix: VON_KRIES,
        }
    }
}

impl Component for ColorSpaceAdaption {
    type Tracking = Modification;
}
