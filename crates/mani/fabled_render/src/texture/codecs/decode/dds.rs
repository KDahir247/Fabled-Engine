use crate::{Extent3d, FlipAxis, Texture, TextureDescriptor};
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

        //todo make flip an arg
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
            format: texture_descriptor.format,
            usage: texture_descriptor.usage,
            sample_count: 1,
            mip_level: 0,
            dimensions: texture_descriptor.dimensions,
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
                    dimensions: Default::default(),
                    format: 18,
                    usage: 6,
                },
            )
            .unwrap();

        assert!(!dds_yellow.data.is_empty());
    }
}
