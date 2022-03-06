use crate::color::contract::ColorSpace;
use crate::color::space::rgb::Rgb;

const SRGB_POW_TRANSFER_FUNCTION: f32 = 1.0 / 2.4;

const RCP_FALLOFF: f32 = 1.0 / 12.92;
const RCP_LINEAR_FALLOFF: f32 = 1.0 / 1.055;


#[derive(Default, Copy, Clone)]
pub struct SRgb {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}


impl SRgb {
    pub fn s_rgb_to_linear(&self) -> SRgb {
        let mut srgb = self.to_owned();

        if self.red > 0.0031308 {
            srgb.red = ((self.red + 0.055) * RCP_LINEAR_FALLOFF).powf(2.4);
        } else {
            srgb.red = self.red * RCP_FALLOFF;
        }

        if self.green > 0.0031308 {
            srgb.green = ((self.green + 0.055) * RCP_LINEAR_FALLOFF).powf(2.4);
        } else {
            srgb.green = self.green * RCP_FALLOFF;
        }

        if self.blue > 0.0031308 {
            srgb.blue = ((self.blue + 0.055) * RCP_LINEAR_FALLOFF).powf(2.4);
        } else {
            srgb.blue = self.blue * RCP_FALLOFF;
        }

        srgb
    }

    pub fn linear_to_s_rgb(&self) -> SRgb {
        let mut srgb = self.to_owned();

        if self.red > 0.0031308 {
            srgb.red = 1.055 * self.red.powf(SRGB_POW_TRANSFER_FUNCTION) - 0.055;
        } else {
            srgb.red = 12.92 * self.red;
        }

        if self.green > 0.0031308 {
            srgb.green = 1.055 * self.green.powf(SRGB_POW_TRANSFER_FUNCTION) - 0.055;
        } else {
            srgb.green = 12.92 * self.green;
        }

        if self.blue > 0.0031308 {
        } else {
        }


        srgb
    }
}
