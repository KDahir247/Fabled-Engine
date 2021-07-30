use crate::texture::codecs::TextureDescriptor;
use crate::texture::container::{Extent3d, FlipAxis, Texture};
use image::GenericImageView;

#[derive(Default, Clone)]
pub struct PngTextureLoader;

impl PngTextureLoader {
    pub fn load<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        texture_descriptor: &TextureDescriptor,
    ) -> anyhow::Result<Texture> {
        let file = std::fs::File::open(path.as_ref())?;

        let png_decoder = image::codecs::png::PngDecoder::new(file)?;

        let dyn_img = image::DynamicImage::from_decoder(png_decoder)?;

        match texture_descriptor.flip_axis {
            FlipAxis::FlipX => dyn_img.fliph(),
            FlipAxis::FlipY => dyn_img.flipv(),
            FlipAxis::FlipZ => unimplemented!(),
        };

        let png_image = Texture {
            data: dyn_img.to_bytes(),
            size: Extent3d {
                width: dyn_img.width(),
                height: dyn_img.height(),
                depth_or_array_layers: 1,
            },
            sample_count: 1,
            mip_level: 0,
            color_type: dyn_img.color().into(),
            rows_per_image: dyn_img.width() * 4,
        };

        Ok(png_image)
    }
}

#[cfg(test)]
mod png_loader_codecs {

    use crate::texture::codecs::{PngTextureLoader, TextureDescriptor};
    use crate::texture::common::*;

    #[test]
    fn load_png() {
        let png_loader = PngTextureLoader::default();
        let png_yellow = png_loader
            .load(
                PNG_TEST_TEXTURE,
                &TextureDescriptor {
                    flip_axis: Default::default(),
                },
            )
            .unwrap();
        assert!(!png_yellow.data.is_empty());
    }
}
