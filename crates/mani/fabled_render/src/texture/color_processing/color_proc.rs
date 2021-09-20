use crate::texture::container::{ColorTarget, Extent3d, Texture};
use crate::texture::ImageProcError;
use image::GenericImageView;

#[repr(align(32))]
pub struct ColorProcessing {
    dyn_texture: image::DynamicImage,
}

impl ColorProcessing {
    pub fn new<T: 'static>(
        texture: Texture,
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

    pub fn build(self) -> Texture {
        let dyn_texture = self.dyn_texture;

        Texture {
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
        let png_loader = PngTextureLoader::default();
        let png_yellow = png_loader
            .load(
                PNG_TEST_TEXTURE,
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

        result
            .write_to(PNG_TEST_TEXTURE_BRIGHTEN, ColorTarget::ImageRgba8)
            .unwrap();
    }

    #[test]
    fn contrast() {
        let color_proc = init_test();

        let result = color_proc.contrast(30.).build();

        result
            .write_to(PNG_TEST_TEXTURE_CONTRAST, ColorTarget::ImageRgba8)
            .unwrap();
    }

    #[test]
    fn grayscale() {
        let color_proc = init_test();
        let result = color_proc.grayscale().build();

        result
            .write_to(PNG_TEST_TEXTURE_GRAYSCALE, ColorTarget::ImageLuma8)
            .unwrap();
    }

    #[test]
    fn hue_rotate() {
        let color_proc = init_test();
        let result = color_proc.hue_rotate(150).build();

        result
            .write_to(PNG_TEST_TEXTURE_HUE_ROT, ColorTarget::ImageRgba8)
            .unwrap();
    }

    #[test]
    fn invert() {
        let color_proc = init_test();
        let result = color_proc.invert().build();

        result
            .write_to(PNG_TEST_TEXTURE_INVERT, ColorTarget::ImageRgba8)
            .unwrap();
    }
}
