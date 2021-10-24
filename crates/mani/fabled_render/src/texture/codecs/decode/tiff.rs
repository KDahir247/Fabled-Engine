use crate::texture::codecs::TextureDescriptor;
use crate::texture::container::{Extent3d, FlipAxis, TextureData};
use crate::texture::CodecsError;
use image::GenericImageView;

#[derive(Default, Clone)]
pub struct TiffTextureLoader;

impl TiffTextureLoader {
    pub fn load<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        texture_descriptor: &TextureDescriptor,
    ) -> Result<TextureData, CodecsError> {
        let file = std::fs::File::open(path.as_ref())?;

        let tiff_decoder =
            image::codecs::tiff::TiffDecoder::new(file).map_err(CodecsError::ImageError)?;

        let dyn_img =
            image::DynamicImage::from_decoder(tiff_decoder).map_err(CodecsError::ImageError)?;

        let data = match texture_descriptor.flip_axis {
            FlipAxis::FlipX => dyn_img.fliph().to_bytes(),
            FlipAxis::FlipY => dyn_img.flipv().to_bytes(),
            _ => dyn_img.to_bytes(), // skips flipping
        };

        let tiff_texture = TextureData {
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

        Ok(tiff_texture)
    }
}

#[cfg(test)]
mod tiff_loader_codecs {
    use crate::texture::codecs::{TextureDescriptor, TiffTextureLoader};
    use crate::texture::common::*;


    #[test]
    fn load_tiff() {
        let texture = load_test_textures("tiff").pop().unwrap();


        let tiff_loader = TiffTextureLoader::default();
        let tiff_yellow = tiff_loader.load(
            texture,
            &TextureDescriptor {
                flip_axis: Default::default(),
            },
        );

        match tiff_yellow {
            Ok(result) => {
                assert!(!result.data.is_empty());
            }
            Err(err) => {
                panic!("{}", err)
            }
        }

        //----------------------------------------------------

        let texture = load_test_textures("dds").pop().unwrap();

        let invalid_tiff_yellow = tiff_loader.load(
            texture,
            &TextureDescriptor {
                flip_axis: Default::default(),
            },
        );

        match invalid_tiff_yellow {
            Ok(_) => {
                panic!("Should not pass")
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
