use crate::{ColorType, Extent3d, FlipAxis, Texture, TextureDescriptor};

#[derive(Default, Clone)]
pub struct HdrTextureLoader;

// HDR File Format
impl HdrTextureLoader {
    pub fn load<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        texture_descriptor: &TextureDescriptor,
    ) -> anyhow::Result<Texture> {
        let file = std::fs::File::open(path.as_ref())?;
        let buf_reader = std::io::BufReader::new(file);
        let hdr_decoder = image::codecs::hdr::HdrDecoder::new(buf_reader)?;

        let meta_data = hdr_decoder.metadata();
        let rgb_data = hdr_decoder.read_image_hdr()?;

        //RGBA32Float
        //RGBA (32 bit == 4 bytes)
        //4 channel (RGBA)
        let mut hdr_data = Vec::with_capacity(rgb_data.len() * 16);
        for rgb in rgb_data {
            hdr_data.extend_from_slice(&rgb.0[0].to_ne_bytes());
            hdr_data.extend_from_slice(&rgb.0[1].to_ne_bytes());
            hdr_data.extend_from_slice(&rgb.0[2].to_ne_bytes());

            //Alpha or exposure.
            hdr_data.extend_from_slice(&1.0f32.to_ne_bytes());
            //maybe store the exposure in the alpha channel.
        }

        let mut hdr_texture = Texture {
            data: hdr_data,
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

        Self::hdr_reorientation(texture_descriptor, &mut hdr_texture);

        Ok(hdr_texture)
    }

    fn hdr_reorientation(desc: &TextureDescriptor, stream_tex: &mut Texture) {
        let mut result = stream_tex.data.to_vec();

        match desc.flip_axis {
            FlipAxis::FlipX => { /*Unimplemented*/ }
            FlipAxis::FlipY => {
                result = stream_tex
                    .data
                    .chunks(stream_tex.size.width as usize * 4)
                    .rev()
                    .flat_map(|row| row.iter())
                    .cloned()
                    .collect::<_>()
            }
            FlipAxis::FlipZ => { /*Unimplemented*/ }
        }

        stream_tex.data = result;
    }
}

#[cfg(test)]
mod hdr_loader_codecs {
    use crate::texture::common::HDR_TEST_TEXTURE;
    use crate::{HdrTextureLoader, TextureDescriptor};

    #[test]
    fn load_hdr() {
        let hdr_loader = HdrTextureLoader::default();
        let hdr_yellow = hdr_loader
            .load(
                HDR_TEST_TEXTURE,
                &TextureDescriptor {
                    flip_axis: Default::default(),
                },
            )
            .unwrap();

        assert!(!hdr_yellow.data.is_empty());
    }
}
