use crate::{ColorType, Extent3d, FlipAxis, Texture, TextureDescriptor};
use image::GenericImageView;

#[derive(Default, Clone)]
pub struct DdsTextureLoader;

// DDS File Format
// The default value is: R8G8B8.
// No generating Mip Map level
// If mipmap is "Yes", the sizes of saved images must be equal to the power of two (128, 256, 512, 1024 etc.).
impl DdsTextureLoader {
    //Decoder
    pub fn load<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        texture_descriptor: &TextureDescriptor,
    ) -> anyhow::Result<Texture> {
        //RGBA8 (R8, G8, B8, A9)

        let file = std::fs::File::open(path.as_ref())?;

        let dds_decoder = image::codecs::dds::DdsDecoder::new(file)?;

        let dyn_img = image::DynamicImage::from_decoder(dds_decoder)?.flipv();

        match texture_descriptor.flip_axis {
            FlipAxis::FlipX => dyn_img.fliph(),
            FlipAxis::FlipY => dyn_img.flipv(),
            FlipAxis::FlipZ => unimplemented!(),
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

    use crate::codecs::*;
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
