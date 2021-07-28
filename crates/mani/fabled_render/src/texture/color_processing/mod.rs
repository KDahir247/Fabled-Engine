pub mod color_proc;
use crate::texture::image_processing::*;
pub use color_proc::*;

/*pub trait channel_conversion {
    fn rgb(&self, rgb_img: &image::DynamicImage) -> image::DynamicImage;

    fn rgba(&self, rgba_img: &RgbaImage) -> image::DynamicImage;

    fn luma(&self, luma_img: &GrayImage) -> image::DynamicImage;

    fn luma_alpha(&self, luma_a_img: &GrayAlphaImage) -> image::DynamicImage;

    fn bgr(&self, bgr_img: &BgraImage) -> image::DynamicImage;

    fn bgra(&self, bgra_img: &BgraImage) -> image::DynamicImage;
}
*/
