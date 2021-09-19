use crate::texture::codecs::TextureDescriptor;
use crate::texture::container::{Extent3d, FlipAxis, Texture};
use crate::texture::CodecsError;
use image::GenericImageView;

#[derive(Default, Clone)]
pub struct JpgTextureLoader;

impl JpgTextureLoader {
    pub fn load<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        texture_descriptor: &TextureDescriptor,
    ) -> Result<Texture, CodecsError> {
        let file = std::fs::File::open(path.as_ref())?;

        let jpg_decoder =
            image::codecs::jpeg::JpegDecoder::new(file).map_err(CodecsError::ImageError)?;

        let mut dyn_img =
            image::DynamicImage::from_decoder(jpg_decoder).map_err(CodecsError::ImageError)?;

        match texture_descriptor.flip_axis {
            FlipAxis::FlipX => {
                dyn_img = dyn_img.fliph();
            }
            FlipAxis::FlipY => {
                dyn_img = dyn_img.flipv();
            }
            _ => {} // skips flipping
        };

        let jpg_texture = Texture {
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

        Ok(jpg_texture)
    }
}

#[cfg(test)]
mod jpg_loader_codecs {

    use crate::texture::codecs::{JpgTextureLoader, TextureDescriptor};
    use crate::texture::common::*;

    #[test]
    fn load_jpg() {
        let jpg_loader = JpgTextureLoader::default();
        let jpg_yellow = jpg_loader
            .load(
                JPG_TEST_TEXTURE,
                &TextureDescriptor {
                    flip_axis: Default::default(),
                },
            )
            .unwrap();

        assert!(!jpg_yellow.data.is_empty());
    }
}
