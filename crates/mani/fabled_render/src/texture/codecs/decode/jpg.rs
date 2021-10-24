use crate::texture::codecs::TextureDescriptor;
use crate::texture::container::{Extent3d, FlipAxis, TextureData};
use crate::texture::CodecsError;
use image::GenericImageView;

#[derive(Default, Clone)]
pub struct JpgTextureLoader;

impl JpgTextureLoader {
    pub fn load<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        texture_descriptor: &TextureDescriptor,
    ) -> Result<TextureData, CodecsError> {
        let file = std::fs::File::open(path.as_ref())?;

        let jpg_decoder =
            image::codecs::jpeg::JpegDecoder::new(file).map_err(CodecsError::ImageError)?;

        let dyn_img =
            image::DynamicImage::from_decoder(jpg_decoder).map_err(CodecsError::ImageError)?;

        let data = match texture_descriptor.flip_axis {
            FlipAxis::FlipX => dyn_img.fliph().to_bytes(),
            FlipAxis::FlipY => dyn_img.flipv().to_bytes(),
            _ => dyn_img.to_bytes(), // skips flipping
        };

        let jpg_texture = TextureData {
            data,
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
        let texture = load_test_textures("jpg").pop().unwrap();

        let jpg_loader = JpgTextureLoader::default();
        let jpg_yellow = jpg_loader.load(
            texture,
            &TextureDescriptor {
                flip_axis: Default::default(),
            },
        );

        match jpg_yellow {
            Ok(result) => {
                assert!(!result.data.is_empty());
            }
            Err(err) => {
                panic!("{}", err)
            }
        }

        //-------------------------------------------------------
        let texture = load_test_textures("tiff").pop().unwrap();


        let invalid_jpg_yellow = jpg_loader.load(
            texture,
            &TextureDescriptor {
                flip_axis: Default::default(),
            },
        );


        match invalid_jpg_yellow {
            Ok(_) => {
                panic!("Should not pass")
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
