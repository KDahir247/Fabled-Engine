use crate::texture::_core::convert::TryFrom;
use crate::texture::container::{ColorType, Extent3d, FlipAxis, TextureData};
use crate::texture::ext::KTXDescriptor;
use crate::texture::KTXError;

use libktx_rs as ktx;

#[derive(Default)]
pub struct KtxTextureLoader;

// --------------------Warning--------------------
//  When loading a ktx file that is compressed. The ktx file must not currently
//  have any mip level or the texture mapping for the model will be incorrect.

impl KtxTextureLoader {
    pub fn default_ktx1() -> TextureData {
        let texture = ktx::Texture::new(ktx::sources::Ktx1CreateInfo::default()).unwrap();

        let dimensions = Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        };

        // gl_internal : 0x8058 == GL_RGBA8
        TextureData {
            data: texture.data().to_vec(),
            size: dimensions,
            sample_count: 1,
            mip_level: 0,
            color_type: ColorType::Rgba8,
            rows_per_image: 4,
        }
    }

    pub fn default_ktx2() -> TextureData {
        let texture = ktx::Texture::new(ktx::sources::Ktx2CreateInfo::default()).unwrap();

        let dimensions = Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        };

        // vk_format 37 == VK_R8G8B8A8_UNORM
        TextureData {
            data: texture.data().to_vec(),
            size: dimensions,
            sample_count: 1,
            mip_level: 0,
            color_type: ColorType::Rgba8,
            rows_per_image: 4,
        }
    }

    pub fn super_compress<'a>(
        file: std::fs::File,
        quality: u32,
    ) -> Result<libktx_rs::Texture<'a>, KTXError> {
        let stream = ktx::RustKtxStream::new(Box::new(file)).map_err(|err_code| {
            KTXError::KTXError(libktx_rs::KtxError::try_from(err_code).unwrap())
        })?;

        let source = ktx::sources::StreamSource::new(
            std::sync::Arc::new(std::sync::Mutex::new(stream)),
            ktx::TextureCreateFlags::LOAD_IMAGE_DATA,
        );

        let mut stream_tex = ktx::Texture::new(source).map_err(KTXError::KTXError)?;

        if let Some(mut ktx2) = stream_tex.ktx2() {
            ktx2.compress_basis(quality).map_err(KTXError::KTXError)?;
        }

        Ok(stream_tex)
    }


    pub fn from_texture(
        mut stream_tex: libktx_rs::Texture,
        ktx_descriptor: &KTXDescriptor,
    ) -> Result<TextureData, KTXError> {
        let format = ktx::TranscodeFormat::try_from(ktx_descriptor.transcode_format as u32)
            .unwrap_or(ktx::TranscodeFormat::Rgba32);

        let transcode_flag = ktx::TranscodeFlags::from_bits(ktx_descriptor.transcode_flag.bits)
            .unwrap_or(ktx::TranscodeFlags::HIGH_QUALITY);

        if let Some(mut ktx2) = stream_tex.ktx2() {
            if ktx2.needs_transcoding() {
                ktx2.transcode_basis(format, transcode_flag)
                    .map_err(KTXError::KTXError)?;
            }
        }

        let mut dimensions = Extent3d::default();
        let mut mip_level: u32 = 1;

        // Safe error just in case data hasn't been loaded.
        stream_tex
            .iterate_levels(|_mip, _face, width, height, depth, _| {
                dimensions.width = width as u32;
                dimensions.height = height as u32;
                dimensions.depth_or_array_layers = depth as u32;
                mip_level = _mip as u32;
                Ok(())
            })
            .map_err(KTXError::KTXError)?;

        let channels = stream_tex.data().chunks(dimensions.width as usize * 4);

        let data = match ktx_descriptor.flip_axis {
            FlipAxis::Skip => stream_tex.data().to_vec(),
            FlipAxis::FlipX => stream_tex.data().to_vec(),
            FlipAxis::FlipY => channels
                .rev()
                .flat_map(|buf| buf.iter())
                .cloned()
                .collect::<Vec<_>>(),
        };


        // Generic handle for ktx1 gl_format and ktx2 vk_format.
        let color_type = match stream_tex.element_size() {
            1 => ColorType::L8,
            2 => ColorType::La8,
            3 => ColorType::Rgb8,
            4 => ColorType::Rgba8,
            _ => panic!("Texture has more then 4 channel and is not supported"),
        };

        Ok(TextureData {
            data,
            size: dimensions,
            sample_count: 1,
            mip_level,
            color_type,
            rows_per_image: stream_tex.row_pitch(0) as u32,
        })
    }

    pub fn from_stream(
        file: std::fs::File,
        ktx_descriptor: &KTXDescriptor,
    ) -> Result<TextureData, KTXError> {
        let stream = ktx::RustKtxStream::new(Box::new(file)).map_err(|err_code| {
            KTXError::KTXError(libktx_rs::KtxError::try_from(err_code).unwrap())
        })?;

        let source = ktx::sources::StreamSource::new(
            std::sync::Arc::new(std::sync::Mutex::new(stream)),
            ktx::TextureCreateFlags::LOAD_IMAGE_DATA,
        );

        let stream_tex = ktx::Texture::new(source).map_err(KTXError::KTXError)?;


        Self::from_texture(stream_tex, ktx_descriptor)
    }
}

#[cfg(test)]
mod ktx_mod_test {

    use crate::texture::common::*;
    use crate::texture::ext::{
        KTXDescriptor, KtxTextureLoader, KtxTranscodeFlag, KtxTranscodeFormat,
    };
    use crate::texture::FlipAxis;


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
    fn ktx_compress() {
        let ktx_path = load_ktx_textures();

        let ktx_super = KtxTextureLoader::super_compress(
            std::fs::File::open(ktx_path[1].as_str()).unwrap(),
            100,
        );

        match ktx_super {
            Ok(result) => {
                let ktx = KtxTextureLoader::from_texture(
                    result,
                    &KTXDescriptor {
                        flip_axis: FlipAxis::Skip,
                        transcode_flag: KtxTranscodeFlag::HIGHEST_QUALITY,
                        transcode_format: KtxTranscodeFormat::ETC1RGB,
                    },
                )
                .unwrap_or_else(|_| KtxTextureLoader::default_ktx2());

                println!("{:?}", ktx.color_type);
            }
            Err(err) => {
                println!("{}", err);
            }
        };
    }

    #[test]
    fn ktx_stream_load() {
        let ktx_path = load_ktx_textures();

        let ktx_stream_texture = KtxTextureLoader::from_stream(
            std::fs::File::open(ktx_path[1].as_str()).unwrap(),
            &KTXDescriptor {
                flip_axis: FlipAxis::Skip,
                transcode_flag: KtxTranscodeFlag::HIGHEST_QUALITY,
                transcode_format: KtxTranscodeFormat::RGBA32,
            },
        )
        .unwrap_or_else(|_| KtxTextureLoader::default_ktx2());

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
