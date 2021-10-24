use crate::texture::codecs::TextureDescriptor;
use crate::texture::container::{Extent3d, FlipAxis, TextureData};
use crate::texture::CodecsError;
use image::GenericImageView;

#[derive(Default, Clone)]
pub struct PngTextureLoader;

impl PngTextureLoader {
    pub fn load<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        texture_descriptor: &TextureDescriptor,
    ) -> Result<TextureData, CodecsError> {
        let file = std::fs::File::open(path.as_ref())?;

        let png_decoder =
            image::codecs::png::PngDecoder::new(file).map_err(CodecsError::ImageError)?;

        let dyn_img =
            image::DynamicImage::from_decoder(png_decoder).map_err(CodecsError::ImageError)?;

        let data = match texture_descriptor.flip_axis {
            FlipAxis::FlipX => dyn_img.fliph().to_bytes(),
            FlipAxis::FlipY => dyn_img.flipv().to_bytes(),
            _ => dyn_img.to_bytes(), // skips flipping
        };

        let png_image = TextureData {
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

        Ok(png_image)
    }
}

#[cfg(test)]
mod png_loader_codecs {

    use crate::texture::codecs::{PngTextureLoader, TextureDescriptor};
    use crate::texture::common::*;

    #[test]
    fn load_png() {
        let texture = load_test_textures("png").pop().unwrap();


        let png_loader = PngTextureLoader::default();
        let png_yellow = png_loader.load(
            texture,
            &TextureDescriptor {
                flip_axis: Default::default(),
            },
        );

        match png_yellow {
            Ok(result) => {
                assert!(!result.data.is_empty());
            }
            Err(err) => {
                panic!("{}", err)
            }
        }

        //-----------------------------------------------------------

        let texture = load_test_textures("jpg").pop().unwrap();


        let invalid_png_yellow = png_loader.load(
            texture,
            &TextureDescriptor {
                flip_axis: Default::default(),
            },
        );

        match invalid_png_yellow {
            Ok(_) => {
                panic!("Should not pass")
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
