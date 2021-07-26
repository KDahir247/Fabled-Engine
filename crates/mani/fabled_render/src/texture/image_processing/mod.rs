pub mod image_proc;

pub use image_proc::*;

pub enum Target {
    ImageLuma8(GrayImage),
    ImageLuma16(Gray16Image),

    ImageLumA8(GrayAlphaImage),
    ImageLumA16(GrayAlpha16Image),

    ImageRgb8(RgbImage),
    ImageRgb16(Rgb16Image),

    ImageRgba8(RgbaImage),
    ImageRgba16(Rgba16Image),

    ImageBgr8(BgrImage),
    ImageBgra8(BgraImage),
}

impl From<Target> for image::DynamicImage {
    fn from(target: Target) -> Self {
        match target {
            Target::ImageRgb8(rgb_8buf) => image::DynamicImage::ImageRgb8(rgb_8buf),
            Target::ImageRgba8(rgba_8buf) => image::DynamicImage::ImageRgba8(rgba_8buf),
            Target::ImageLuma8(luma_8buf) => image::DynamicImage::ImageLuma8(luma_8buf),
            Target::ImageLuma16(luma_16buf) => image::DynamicImage::ImageLuma16(luma_16buf),
            Target::ImageLumA8(luma_plus_alpha_8buf) => {
                image::DynamicImage::ImageLumaA8(luma_plus_alpha_8buf)
            }
            Target::ImageLumA16(luma_plus_alpha_16buf) => {
                image::DynamicImage::ImageLumaA16(luma_plus_alpha_16buf)
            }
            Target::ImageRgb16(rgb_16buf) => image::DynamicImage::ImageRgb16(rgb_16buf),
            Target::ImageRgba16(rgba_16buf) => image::DynamicImage::ImageRgba16(rgba_16buf),
            Target::ImageBgr8(bgr_8buf) => image::DynamicImage::ImageBgr8(bgr_8buf),
            Target::ImageBgra8(bgra_8buf) => image::DynamicImage::ImageBgra8(bgra_8buf),
        }
    }
}

pub type GrayImage = image::ImageBuffer<image::Luma<u8>, Vec<u8>>;
pub type Gray16Image = image::ImageBuffer<image::Luma<u16>, Vec<u16>>;

pub type GrayAlphaImage = image::ImageBuffer<image::LumaA<u8>, Vec<u8>>;
pub type GrayAlpha16Image = image::ImageBuffer<image::LumaA<u16>, Vec<u16>>;

pub type RgbImage = image::ImageBuffer<image::Rgb<u8>, Vec<u8>>;
pub type Rgb16Image = image::ImageBuffer<image::Rgb<u16>, Vec<u16>>;

pub type RgbaImage = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;
pub type Rgba16Image = image::ImageBuffer<image::Rgba<u16>, Vec<u16>>;

pub type BgrImage = image::ImageBuffer<image::Bgr<u8>, Vec<u8>>;
pub type BgraImage = image::ImageBuffer<image::Bgra<u8>, Vec<u8>>;
