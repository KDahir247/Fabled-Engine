use crate::{Extent3d, FlipAxis, Texture, TextureDescriptor};
use image::GenericImageView;

#[derive(Default, Clone)]
pub struct PngTextureLoader;

// Png File Format
// The default value is: R8G8B8A8
// Yes generating Mip Map level
// todo write the general information of this codec loader
impl PngTextureLoader {
    pub fn load<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        texture_descriptor: &TextureDescriptor,
    ) -> anyhow::Result<Texture> {
        //grayscale luminance (with or without transparency) or RGB24 (R8G8B8) or RGBA32 (R8G8B8A8)

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
            format: texture_descriptor.format,
            usage: texture_descriptor.usage,
            sample_count: 1,
            mip_level: 0,
            dimensions: texture_descriptor.dimensions,
            rows_per_image: dyn_img.width() * 4,
        };

        Ok(png_image)
    }
}

#[cfg(test)]
mod png_loader_codecs {

    use crate::codecs::*;
    use crate::texture::common::*;

    #[test]
    fn load_png() {
        let png_loader = PngTextureLoader::default();
        let pngyellow = png_loader
            .load(
                PNG_TEST_TEXTURE,
                &TextureDescriptor {
                    flip_axis: Default::default(),
                    dimensions: Default::default(),
                    format: 18,
                    usage: 6,
                },
            )
            .unwrap();
        assert!(!pngyellow.data.is_empty());
    }
}
