#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ColorType {
    L8,

    La8,

    Rgba8,

    L16,

    La16,

    Rgb8,

    Rgb16,

    Rgba16,

    Bgr8,

    Bgra8,

    Nil,
}

impl From<image::ColorType> for ColorType {
    fn from(color_type: image::ColorType) -> Self {
        match color_type {
            image::ColorType::L8 => ColorType::L8,
            image::ColorType::La8 => ColorType::La8,
            image::ColorType::Rgba8 => ColorType::Rgba8,
            image::ColorType::L16 => ColorType::L16,
            image::ColorType::La16 => ColorType::La16,
            image::ColorType::Rgb16 => ColorType::Rgb16,
            image::ColorType::Rgba16 => ColorType::Rgba16,
            image::ColorType::Bgr8 => ColorType::Bgr8,
            image::ColorType::Bgra8 => ColorType::Bgra8,
            image::ColorType::Rgb8 => ColorType::Rgb8,
            _ => ColorType::Nil,
        }
    }
}

#[repr(align(64))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ColorTarget {
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

impl From<ColorTarget> for image::DynamicImage {
    fn from(target: ColorTarget) -> Self {
        match target {
            ColorTarget::ImageRgb8(rgb_8buf) => image::DynamicImage::ImageRgb8(rgb_8buf),
            ColorTarget::ImageRgba8(rgba_8buf) => image::DynamicImage::ImageRgba8(rgba_8buf),
            ColorTarget::ImageLuma8(luma_8buf) => image::DynamicImage::ImageLuma8(luma_8buf),
            ColorTarget::ImageLuma16(luma_16buf) => image::DynamicImage::ImageLuma16(luma_16buf),
            ColorTarget::ImageLumA8(luma_plus_alpha_8buf) => {
                image::DynamicImage::ImageLumaA8(luma_plus_alpha_8buf)
            }
            ColorTarget::ImageLumA16(luma_plus_alpha_16buf) => {
                image::DynamicImage::ImageLumaA16(luma_plus_alpha_16buf)
            }
            ColorTarget::ImageRgb16(rgb_16buf) => image::DynamicImage::ImageRgb16(rgb_16buf),
            ColorTarget::ImageRgba16(rgba_16buf) => image::DynamicImage::ImageRgba16(rgba_16buf),
            ColorTarget::ImageBgr8(bgr_8buf) => image::DynamicImage::ImageBgr8(bgr_8buf),
            ColorTarget::ImageBgra8(bgra_8buf) => image::DynamicImage::ImageBgra8(bgra_8buf),
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
