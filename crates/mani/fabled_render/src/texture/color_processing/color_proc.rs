use crate::texture::container::{ColorTarget, ColorType, Extent3d, Texture};
use anyhow::Context;
use image::GenericImageView;

#[repr(align(32))]
pub struct ColorProcessing {
    dyn_texture: image::DynamicImage,
}

impl ColorProcessing {
    pub fn new<T: 'static>(
        texture: Texture,
        color_target_predicate: fn(image::ImageBuffer<T, Vec<u8>>) -> ColorTarget,
    ) -> anyhow::Result<ColorProcessing>
    where
        T: image::Pixel<Subpixel = u8>, {
        let dyn_texture =
            image::ImageBuffer::from_raw(texture.size.width, texture.size.height, texture.data)
                .context("ColorTarget matches requirement for creating ImageBuffer from texture")?;

        let color_target: ColorType = dyn_texture
            .as_flat_samples()
            .color_hint
            .context("Texture has color channel")?
            .into();

        let texture_target = color_target_predicate(dyn_texture);

        assert_eq!(
            color_target, texture.color_type,
            "Transforming color channel to ColorTarget is not supported yet"
        );

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
    use crate::texture::container::{ColorTarget, Texture};

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

    fn write_back<T: 'static>(
        path: &str,
        texture: Texture,
        color_target_predicate: fn(image::ImageBuffer<T, Vec<u8>>) -> ColorTarget,
    ) where
        T: image::Pixel<Subpixel = u8>, {
        let img_buf =
            image::ImageBuffer::from_raw(texture.size.width, texture.size.height, texture.data)
                .expect("Created image buffer");

        let dyn_img: image::DynamicImage = color_target_predicate(img_buf).into();

        dyn_img
            .save_with_format(path, image::ImageFormat::Png)
            .expect("saving transformed image");
    }

    #[test]
    fn brighten() {
        let color_proc = init_test();

        let result = color_proc.brighten(50).build();

        write_back(PNG_TEST_TEXTURE_BRIGHTEN, result, ColorTarget::ImageRgba8);
    }

    #[test]
    fn contrast() {
        let color_proc = init_test();

        let result = color_proc.contrast(30.).build();

        write_back(PNG_TEST_TEXTURE_CONTRAST, result, ColorTarget::ImageRgba8);
    }

    #[test]
    fn grayscale() {
        let color_proc = init_test();
        let result = color_proc.grayscale().build();
        write_back(PNG_TEST_TEXTURE_GRAYSCALE, result, ColorTarget::ImageLuma8);
    }

    #[test]
    fn hue_rotate() {
        let color_proc = init_test();
        let result = color_proc.hue_rotate(150).build();
        write_back(PNG_TEST_TEXTURE_HUE_ROT, result, ColorTarget::ImageRgba8);
    }

    #[test]
    fn invert() {
        let color_proc = init_test();
        let result = color_proc.invert().build();
        write_back(PNG_TEST_TEXTURE_INVERT, result, ColorTarget::ImageRgba8);
    }
}
