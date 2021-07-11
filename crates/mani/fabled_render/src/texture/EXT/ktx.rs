use crate::texture::container::Extent3d;
use crate::Texture;
use crate::{KtxTranscodeFlag, TextureViewDimension};
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

        let mut mip_level = 0;
        let mut dimensions = Extent3d {
            width: 0,
            height: 0,
            depth_or_array_layers: 0,
        };

        texture
            .iterate_levels(|mip, _, width, height, depth, _| {
                mip_level = mip;
                dimensions.width = width as u32;
                dimensions.height = height as u32;
                dimensions.depth_or_array_layers = depth as u32;
                Ok(())
            })
            .expect("Retrieving KTX1 texture information");

        Texture {
            data: texture.data().to_vec(),
            size: dimensions,
            format: 43,
            usage: 6,
            sample_count: 1,
            mip_level,
            dimensions: TextureViewDimension::D2,
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
                Ok(())
            })
            .expect("Retrieving KTX2 texture information");

        Texture {
            data: texture.data().to_vec(),
            size: dimensions,
            format: 43,
            usage: 6,
            sample_count: 1,
            mip_level,
            dimensions: TextureViewDimension::D2,
        }
    }

    //todo clean
    //ARGS file, flip v,
    pub fn from_stream<'a>(file: std::fs::File) -> ktx::Texture<'a> {
        let stream = ktx::RustKtxStream::new(Box::new(file)).expect("The RustKtxStream");
        let source = ktx::sources::StreamSource::new(
            std::sync::Arc::new(std::sync::Mutex::new(stream)),
            ktx::TextureCreateFlags::LOAD_IMAGE_DATA,
        );

        let mut stream_tex = ktx::Texture::new(source).expect("the loaded KTX");

        //todo got to check if
        if stream_tex.ktx1().is_some() {
            println!("Texture is KTX1");
        } else if stream_tex.ktx2().is_some() {
            println!("Texture is KTX2");
        } else {
            panic!("the loaded texture is not KTX 1 or 2");
        }

        if let Some(mut ktx2) = stream_tex.ktx2() {
            if ktx2.needs_transcoding() {
                println!("This KTX2 needs transcoding");
                ktx2.transcode_basis(
                    ktx::TranscodeFormat::Bc3Rgba,
                    KtxTranscodeFlag::HIGHEST_QUALITY.into(),
                )
                .expect("transcoding to work");
            }
        }

        /* let a = Texture {
            data: vec![],
            size: Default::default(),
            format: 0,
            usage: 0,
            sample_count: 0,
            mip_level: 0,
            dimensions: TextureViewDimension::D1,
        };*/

        //todo return texture from texture_data.rs file.
        stream_tex
    }
}

#[cfg(test)]
mod ktx_mod_test {

    use crate::texture::common::*;
    use crate::EXT::*;
    #[test]
    fn basic_ktx1() {
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

        let ktx2_texture = KtxTextureLoader::default_ktx2();

        container = 0;

        assert_eq!(ktx2_texture.size.depth_or_array_layers, 1);
        assert_eq!(ktx2_texture.size.height, 1);
        assert_eq!(ktx2_texture.size.width, 1);
        assert_ne!(ktx2_texture.data.len(), 0);

        assert_eq!(container, 0);

        for ktx2_bit in ktx2_texture.data {
            container += ktx2_bit as u64;
        }

        assert!(container > 0);

        KtxTextureLoader::from_stream(std::fs::File::open(KTX_2_TEST_TEXTURE).unwrap());
    }
}
