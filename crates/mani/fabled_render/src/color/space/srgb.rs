use crate::color::contract::ColorSpace;
use crate::color::space::rgb::Rgb;

const SRGB_POW_TRANSFER_FUNCTION: f32 = 1.0 / 2.4;

#[derive(Default)]
pub struct SRgb {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl ColorSpace for SRgb {
    type TargetSpace = Rgb;

    fn convert_space_from(rgb: Self::TargetSpace) -> Self {
        let mut srgb = SRgb::default();

        if rgb.red > 0.0031308 {
            srgb.red = 1.055 * rgb.red.powf(SRGB_POW_TRANSFER_FUNCTION) - 0.055;
        } else {
            srgb.red = 12.92 * rgb.red;
        }

        if rgb.green > 0.0031308 {
            srgb.green = 1.055 * rgb.green.powf(SRGB_POW_TRANSFER_FUNCTION) - 0.055;
        } else {
            srgb.green = 12.92 * rgb.green;
        }

        if rgb.blue > 0.0031308 {
            srgb.blue = 1.055 * rgb.blue.powf(SRGB_POW_TRANSFER_FUNCTION) - 0.055
        } else {
            srgb.blue = 12.92 * rgb.blue;
        }

        srgb
    }
}
