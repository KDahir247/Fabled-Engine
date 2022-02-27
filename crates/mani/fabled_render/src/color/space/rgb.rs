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

impl ColorSpace for Rgb {
    type TargetSpace = SRgb;

    fn convert_space_from(srgb: Self::TargetSpace) -> Self {
        let mut rgb = Rgb::default();

        if srgb.red > 0.0031308 {
            rgb.red = ((rgb.red + 0.055) * RCP_LINEAR_FALLOFF).powf(2.4);
        } else {
            rgb.red = srgb.red * RCP_FALLOFF;
        }

        if srgb.blue > 0.0031308 {
            rgb.blue = ((rgb.blue + 0.055) * RCP_LINEAR_FALLOFF).powf(2.4);
        } else {
            rgb.blue = srgb.blue * RCP_FALLOFF;
        }

        if srgb.green > 0.0031308 {
            rgb.green = ((rgb.green + 0.055) * RCP_LINEAR_FALLOFF).powf(2.4);
        } else {
            rgb.green = srgb.green * RCP_FALLOFF;
        }

        rgb
    }
}

impl From<Rgb> for [f32; 3] {
    // Scene Referred simply means the image data is maintained in a format that as
    // closely as possible represents the original scene, without effective
    // restriction on colour or dynamic range. If we remove the cameras gamma curve
    // and just record the data coming off the sensor we are recording a measurement
    // of the true light coming from the scene just as it is.
    fn from(rgb: Rgb) -> Self {
        let exponent = (rgb.red.max(rgb.green.max(rgb.blue))).log2() + 128.0;


        todo!()
    }
}
