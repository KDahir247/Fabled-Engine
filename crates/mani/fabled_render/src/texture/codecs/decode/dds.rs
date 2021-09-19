use crate::texture::codecs::TextureDescriptor;
use crate::texture::container::{Extent3d, FlipAxis, Texture};
use crate::texture::CodecsError;
use image::GenericImageView;

#[derive(Default, Clone)]
pub struct DdsTextureLoader;

impl DdsTextureLoader {
    pub fn load<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        texture_descriptor: &TextureDescriptor,
    ) -> Result<Texture, CodecsError> {
        let file = std::fs::File::open(path.as_ref())?;

        let dds_decoder =
            image::codecs::dds::DdsDecoder::new(file).map_err(CodecsError::ImageError)?;

        let mut dyn_img =
            image::DynamicImage::from_decoder(dds_decoder).map_err(CodecsError::ImageError)?;

        match texture_descriptor.flip_axis {
            FlipAxis::FlipX => {
                dyn_img = dyn_img.fliph();
            }
            FlipAxis::FlipY => {
                dyn_img = dyn_img.flipv();
            }
            _ => {} // skips flipping
        };

        let dds_texture = Texture {
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
        Ok(dds_texture)
    }
}

#[cfg(test)]
mod dds_loader_codecs {

    use crate::texture::codecs::{DdsTextureLoader, TextureDescriptor};
    use crate::texture::common::*;

    #[test]
    fn load_dds() {
        let dds_loader = DdsTextureLoader::default();
        let dds_yellow = dds_loader
            .load(
                DDS_TEST_TEXTURE,
                &TextureDescriptor {
                    flip_axis: Default::default(),
                },
            )
            .unwrap();

        assert!(!dds_yellow.data.is_empty());
    }
}
