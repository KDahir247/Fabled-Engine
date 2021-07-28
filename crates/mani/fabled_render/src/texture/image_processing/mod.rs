pub mod image_proc;
pub use image_proc::*;

pub enum FilterType {
    Nearest,
    Triangle,
    CatmullRom,
    Gaussian,
    Lanczos3,
}

impl From<FilterType> for image::imageops::FilterType {
    fn from(filter: FilterType) -> Self {
        match filter {
            FilterType::Nearest => image::imageops::FilterType::Nearest,
            FilterType::Triangle => image::imageops::FilterType::Triangle,
            FilterType::CatmullRom => image::imageops::FilterType::CatmullRom,
            FilterType::Gaussian => image::imageops::FilterType::Gaussian,
            FilterType::Lanczos3 => image::imageops::FilterType::Lanczos3,
        }
    }
}

#[repr(align(64))]
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

#[cfg(test)]
mod data_alignment_test {
    use crate::texture::image_processing::{ColorTarget, FilterType, ImageProcessing};

    #[test]
    fn data_alignment() {
        let target_color = std::mem::size_of::<ColorTarget>();
        assert_eq!(target_color & (target_color - 1), 0);

        let filter_type = std::mem::size_of::<FilterType>();
        assert_eq!(filter_type & (filter_type - 1), 0);

        //todo not aligned
        let img_proc = std::mem::size_of::<ImageProcessing>();
        assert_eq!(img_proc & (img_proc - 1), 0);
    }
}
