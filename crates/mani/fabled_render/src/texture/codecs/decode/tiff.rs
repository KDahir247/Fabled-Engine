use crate::texture::codecs::TextureDescriptor;
use crate::texture::container::{Extent3d, FlipAxis, Texture};

use image::GenericImageView;

#[derive(Default, Clone)]
pub struct TiffTextureLoader;

impl TiffTextureLoader {
    pub fn load<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        texture_descriptor: &TextureDescriptor,
    ) -> anyhow::Result<Texture> {
        let file = std::fs::File::open(path.as_ref())?;

        let tiff_decoder = image::codecs::tiff::TiffDecoder::new(file)?;

        let dyn_img = image::DynamicImage::from_decoder(tiff_decoder)?;

        match texture_descriptor.flip_axis {
            FlipAxis::FlipX => dyn_img.fliph(),
            FlipAxis::FlipY => dyn_img.flipv(),
            FlipAxis::FlipZ => unimplemented!(),
        };

        let tiff_texture = Texture {
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

        Ok(tiff_texture)
    }
}

#[cfg(test)]
mod tiff_loader_codecs {
    use crate::texture::codecs::{TextureDescriptor, TiffTextureLoader};
    use crate::texture::common::*;

    #[test]
    fn load_tiff() {
        let tiff_loader = TiffTextureLoader::default();
        let tiff_yellow = tiff_loader
            .load(
                TIFF_TEST_TEXTURE,
                &TextureDescriptor {
                    flip_axis: Default::default(),
                },
            )
            .unwrap();
        assert!(!tiff_yellow.data.is_empty());
    }
}
