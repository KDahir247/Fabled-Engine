use crate::texture::container::{ColorTarget, Extent3d, TextureData};
use crate::texture::ImageProcError;
use image::GenericImageView;

#[repr(align(32))]
pub struct ColorProcessing {
    dyn_texture: image::DynamicImage,
}

impl ColorProcessing {
    pub fn new<T: 'static>(
        texture: TextureData,
        color_target_predicate: fn(image::ImageBuffer<T, Vec<u8>>) -> ColorTarget,
    ) -> Result<ColorProcessing, ImageProcError>
    where
        T: image::Pixel<Subpixel = u8>, {
        let dyn_texture =
            image::ImageBuffer::from_raw(texture.size.width, texture.size.height, texture.data)
                .ok_or(ImageProcError::InSufficientAllocationSize)?;


        let texture_target = color_target_predicate(dyn_texture);

        let dyn_texture: image::DynamicImage = texture_target.into();


        Ok(Self { dyn_texture })
    }

    pub fn brighten(mut self, brightness_factor: i32) -> Self {
        self.dyn_texture = self.dyn_texture.brighten(brightness_factor);
        self
    }

    pub fn contrast(mut self, contrast_factor: f32) -> Self {
        self.dyn_texture = self.dyn_texture.adjust_contrast(contrast_factor);
        self
    }

    pub fn grayscale(mut self) -> Self {
        self.dyn_texture = self.dyn_texture.grayscale();
        self
    }

    pub fn hue_rotate(mut self, hue_degree: i32) -> Self {
        self.dyn_texture = self.dyn_texture.huerotate(hue_degree);
        self
    }

    pub fn invert(mut self) -> Self {
        self.dyn_texture.invert();
        self
    }

    pub fn build(self) -> TextureData {
        let dyn_texture = self.dyn_texture;

        TextureData {
            data: dyn_texture.to_bytes(),
            size: Extent3d {
                width: dyn_texture.width(),
                height: dyn_texture.height(),
                depth_or_array_layers: 1,
            },
            sample_count: 1,
            mip_level: 0,
            color_type: dyn_texture.color().into(),
            rows_per_image: dyn_texture.width() * dyn_texture.color().channel_count() as u32,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::ColorProcessing;
    use crate::texture::codecs::{PngTextureLoader, TextureDescriptor};
    use crate::texture::common::*;
    use crate::texture::container::ColorTarget;

    fn init_test() -> ColorProcessing {
        let texture = load_test_textures("png").pop().unwrap();


        let png_loader = PngTextureLoader::default();
        let png_yellow = png_loader
            .load(
                texture,
                &TextureDescriptor {
                    flip_axis: Default::default(),
                },
            )
            .unwrap();

        ColorProcessing::new(png_yellow, ColorTarget::ImageRgba8).unwrap()
    }


    #[test]
    fn brighten() {
        let color_proc = init_test();
        let result = color_proc.brighten(50).build();

        let sav_dir = save_test_texture("test/albedo/color_proc/pngyellow_brightness.png");
        result.write_to(sav_dir, ColorTarget::ImageRgba8).unwrap();
    }

    #[test]
    fn contrast() {
        let color_proc = init_test();
        let result = color_proc.contrast(30.).build();

        let sav_dir = save_test_texture("test/albedo/color_proc/pngyellow_contrast.png");

        result.write_to(sav_dir, ColorTarget::ImageRgba8).unwrap();
    }

    #[test]
    fn grayscale() {
        let color_proc = init_test();
        let result = color_proc.grayscale().build();

        let sav_dir = save_test_texture("test/albedo/color_proc/pngyellow_grayscale.png");

        result.write_to(sav_dir, ColorTarget::ImageLuma8).unwrap();
    }

    #[test]
    fn hue_rotate() {
        let color_proc = init_test();
        let result = color_proc.hue_rotate(150).build();

        let sav_dir = save_test_texture("test/albedo/color_proc/pngyellow_huerot.png");
        result.write_to(sav_dir, ColorTarget::ImageRgba8).unwrap();
    }

    #[test]
    fn invert() {
        let color_proc = init_test();
        let result = color_proc.invert().build();

        let sav_dir = save_test_texture("test/albedo/color_proc/pngyellow_invert.png");

        result.write_to(sav_dir, ColorTarget::ImageRgba8).unwrap();
    }
}
