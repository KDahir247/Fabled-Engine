use crate::color::contract::ColorSpace;
use crate::color::space::srgb::SRgb;

const RCP_FALLOFF: f32 = 1.0 / 12.92;
const RCP_LINEAR_FALLOFF: f32 = 1.0 / 1.055;


#[derive(Default)]
pub struct Rgb {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
