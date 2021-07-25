use crate::convert::TryFrom;
use crate::texture::container::{Extent3d, FlipAxis};
use crate::{KTXDescriptor, Texture};
use libktx_rs as ktx;

#[derive(Default)]
pub struct KtxTextureLoader;

// --------------------Warning--------------------
//  When loading a ktx file that is compressed. The ktx file must not currently
//  have any mip level or the texture mapping for the model will be incorrect.
impl KtxTextureLoader {
    pub fn default_ktx1() -> Texture {
        let texture = ktx::Texture::new(ktx::sources::Ktx1CreateInfo::default())
            .expect("a default KTX1 texture");

        assert_eq!(texture.element_size(), 4);
        assert_eq!(texture.row_pitch(0), 4);
        assert_eq!(texture.data_size(), 4);

        let mut _mip_level = 0;
        let mut dimensions = Extent3d {
            width: 0,
            height: 0,
            depth_or_array_layers: 0,
        };

        texture
            .iterate_levels(|mip, _, width, height, depth, _| {
                _mip_level = mip;
                dimensions.width = width as u32;
                dimensions.height = height as u32;
                dimensions.depth_or_array_layers = depth as u32;
                _mip_level = mip;
                Ok(())
            })
            .expect("Retrieving KTX1 texture information");

        Texture {
            data: texture.data().to_vec(),
            size: dimensions,
            sample_count: 1,
            mip_level: _mip_level as u32,
            channel_count: 4, //todo not sure what color type and the channel count.
            rows_per_image: texture.row_pitch(0) as u32,
        }
    }

    pub fn default_ktx2() -> Texture {
        let texture = ktx::Texture::new(ktx::sources::Ktx2CreateInfo::default())
            .expect("a default KTX2 texture");

        assert_eq!(texture.element_size(), 4);
        assert_eq!(texture.row_pitch(0), 4);
        assert_eq!(texture.data_size(), 4);

        let mut mip_level = 0;
        let mut dimensions = Extent3d {
            width: 0,
            height: 0,
            depth_or_array_layers: 0,
        };

        texture
            .iterate_levels(|mip, _, width, height, depth, _| {
                dimensions.width = width as u32;
                dimensions.height = height as u32;
                dimensions.depth_or_array_layers = depth as u32;
                mip_level = mip;
                Ok(())
            })
            .expect("Retrieving KTX2 texture information");

        Texture {
            data: texture.data().to_vec(),
            size: dimensions,
            sample_count: 1,
            mip_level: mip_level as u32,
            channel_count: 4, //todo not sure what color type and the channel count.
            rows_per_image: texture.row_pitch(0) as u32,
        }
    }

    pub fn from_stream(file: std::fs::File, ktx_descriptor: &KTXDescriptor) -> Texture {
        let stream = ktx::RustKtxStream::new(Box::new(file)).expect("The RustKtxStream");
        let source = ktx::sources::StreamSource::new(
            std::sync::Arc::new(std::sync::Mutex::new(stream)),
            ktx::TextureCreateFlags::LOAD_IMAGE_DATA,
        );
        let mut stream_tex = ktx::Texture::new(source).expect("the loaded KTX");

        assert!(
            stream_tex.element_size() > 0,
            "The element byte size is zero"
        );
        assert!(
            stream_tex.row_pitch(0) > 0,
            "The mip level at 0 (core-image) dimensions can't be zero"
        );
        assert!(
            stream_tex.data_size() > 0,
            "There are no bytes in the KTX file. File is probably corrupted or contains no content"
        );
        assert!(
            stream_tex.ktx1().is_some() || stream_tex.ktx2().is_some(),
            "File read from the KTX_TEXTURE_LOADER is not KTX1 or KTX2"
        );

        let format = ktx::TranscodeFormat::try_from(ktx_descriptor.transcode_format as u32)
            .unwrap_or(ktx::TranscodeFormat::Rgba32);

        let transcode_flag = ktx::TranscodeFlags::from_bits(ktx_descriptor.transcode_flag.bits)
            .unwrap_or(ktx::TranscodeFlags::HIGH_QUALITY);

        if let Some(mut ktx2) = stream_tex.ktx2() {
            if ktx2.needs_transcoding() {
                println!("This KTX2 needs transcoding");

                ktx2.transcode_basis(format, transcode_flag)
                    .expect("transcoding KTX2 to work");
            }
        }

        let mut dimensions = Extent3d::default();
        let mut mip_level: u32 = 1;
        //let mut faces = 0;

        stream_tex
            .iterate_levels(|_mip, _face, width, height, depth, _| {
                //faces = face;
                dimensions.width = width as u32;
                dimensions.height = height as u32;
                dimensions.depth_or_array_layers = depth as u32;
                mip_level = _mip as u32;
                Ok(())
            })
            .expect("Iterating over the all mip level for the KTX file");

        let manipulated_buf =
            KtxTextureLoader::ktx_reorientation(ktx_descriptor, &stream_tex, &dimensions);

        Texture {
            data: manipulated_buf,
            size: dimensions,
            sample_count: 1,
            mip_level,
            channel_count: 4, //todo not sure what color type and the channel count.
            rows_per_image: stream_tex.row_pitch(0) as u32,
        }
    }

    fn ktx_reorientation(
        ktx_descriptor: &KTXDescriptor,
        stream_tex: &ktx::Texture,
        dimensions: &Extent3d,
    ) -> Vec<u8> {
        let mut result = stream_tex.data().to_vec();

        if let Some(target) = ktx_descriptor.flip_axis {
            match target {
                FlipAxis::FlipX => { /*Unimplemented*/ }
                FlipAxis::FlipY => {
                    result = stream_tex
                        .data()
                        .chunks(dimensions.width as usize * 4)
                        .rev()
                        .flat_map(|row| row.iter())
                        .cloned()
                        .collect::<Vec<_>>();
                }
                FlipAxis::FlipZ => { /*Unimplemented*/ }
            }
        }
        result
    }
}

#[cfg(test)]
mod ktx_mod_test {

    use crate::ext::*;
    use crate::texture::common::*;
    #[test]
    fn basic_ktx1_test() {
        let ktx1_texture = KtxTextureLoader::default_ktx1();

        assert_eq!(ktx1_texture.size.depth_or_array_layers, 1);
        assert_eq!(ktx1_texture.size.height, 1);
        assert_eq!(ktx1_texture.size.width, 1);
        assert_ne!(ktx1_texture.data.len(), 0);

        let mut container: u64 = 0;

        for ktx1_bit in ktx1_texture.data {
            container += ktx1_bit as u64;
        }
        assert!(container > 0);
    }

    #[test]
    fn basic_ktx2_test() {
        let ktx2_texture = KtxTextureLoader::default_ktx2();

        let mut container = 0;

        assert_eq!(ktx2_texture.size.depth_or_array_layers, 1);
        assert_eq!(ktx2_texture.size.height, 1);
        assert_eq!(ktx2_texture.size.width, 1);
        assert_ne!(ktx2_texture.data.len(), 0);

        assert_eq!(container, 0);

        for ktx2_bit in ktx2_texture.data {
            container += ktx2_bit as u64;
        }

        assert!(container > 0);
    }

    #[test]
    fn ktx_stream_load() {
        let ktx_stream_texture = KtxTextureLoader::from_stream(
            std::fs::File::open(KTX_2_TEST_TEXTURE).unwrap(),
            &KTXDescriptor {
                flip_axis: None,
                transcode_flag: KtxTranscodeFlag::HIGHEST_QUALITY,
                transcode_format: KtxTranscodeFormat::RGBA32,
            },
        );

        let mut container = 0;
        assert_eq!(ktx_stream_texture.size.width, 1024);
        assert_eq!(ktx_stream_texture.size.height, 1024);
        assert_ne!(ktx_stream_texture.data.len(), 0);

        for ktx_data in ktx_stream_texture.data {
            container += ktx_data as u64;
        }

        assert!(container > 0);
    }
}
