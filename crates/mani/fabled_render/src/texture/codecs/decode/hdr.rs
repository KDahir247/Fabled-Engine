use crate::texture::codecs::TextureDescriptor;
use crate::texture::container::{ColorType, Extent3d, FlipAxis, TextureData};
use crate::texture::CodecsError;

#[derive(Default, Clone)]
pub struct HdrTextureLoader;

impl HdrTextureLoader {
    pub fn load<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        texture_descriptor: &TextureDescriptor,
    ) -> Result<TextureData, CodecsError> {
        let file = std::fs::File::open(path.as_ref())?;

        let buf_reader = std::io::BufReader::new(file);

        let hdr_decoder =
            image::codecs::hdr::HdrDecoder::new(buf_reader).map_err(CodecsError::ImageError)?;

        let meta_data = hdr_decoder.metadata();

        let rgb_data = hdr_decoder
            .read_image_hdr()
            .map_err(CodecsError::ImageError)?;

        let mut hdr_data = Vec::with_capacity(rgb_data.len() * 16);

        for rgb in rgb_data {
            hdr_data.extend_from_slice(&rgb.0[0].to_ne_bytes());
            hdr_data.extend_from_slice(&rgb.0[1].to_ne_bytes());
            hdr_data.extend_from_slice(&rgb.0[2].to_ne_bytes());
            hdr_data.extend_from_slice(&1.0f32.to_ne_bytes());
        }

        let channels = hdr_data.chunks((meta_data.width * 4) as usize);

        let data = match texture_descriptor.flip_axis {
            FlipAxis::Skip => hdr_data,
            FlipAxis::FlipX => channels
                .rev()
                .flat_map(|row| row.iter())
                .cloned()
                .collect::<_>(),
            FlipAxis::FlipY => hdr_data, // not supported yet.
        };


        let hdr_texture = TextureData {
            data,
            size: Extent3d {
                width: meta_data.width,
                height: meta_data.height,
                depth_or_array_layers: 1,
            },
            sample_count: 1,
            mip_level: 0,
            color_type: ColorType::Rgba16,
            rows_per_image: meta_data.width * 16,
        };


        Ok(hdr_texture)
    }
}

#[cfg(test)]
mod hdr_loader_codecs {
    use crate::texture::codecs::{HdrTextureLoader, TextureDescriptor};
    use crate::texture::common::*;

    #[test]
    fn load_hdr() {
        let texture = load_test_textures("hdr").pop().unwrap();

        let hdr_loader = HdrTextureLoader::default();
        let hdr_yellow = hdr_loader.load(
            texture,
            &TextureDescriptor {
                flip_axis: Default::default(),
            },
        );

        match hdr_yellow {
            Ok(result) => {
                assert!(!result.data.is_empty());
                println!("Pass");
            }
            Err(err) => {
                panic!("{}", err);
            }
        }

        //--------------------------------------------------------

        let texture = load_test_textures("dds").pop().unwrap();

        let invalid_hdr_yellow = hdr_loader.load(
            texture,
            &TextureDescriptor {
                flip_axis: Default::default(),
            },
        );

        match invalid_hdr_yellow {
            Ok(_) => {
                panic!("Should not pass")
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
