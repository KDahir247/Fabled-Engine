use fabled_core::concurrent::container::thread_op::ThreadOperation;

use crate::texture::compression::{BasisTexture, CompressionDescriptor, CompressionQuality};
use crate::{ColorType, Texture};
use basis_universal::{Compressor, CompressorParams};

pub fn super_compress(
    texture: &Texture,
    compression_desc: &CompressionDescriptor,
    thread_op: ThreadOperation,
) -> BasisTexture {
    let channel_count = match texture.color_type {
        ColorType::L8 | ColorType::L16 => 1,
        ColorType::La8 | ColorType::La16 => 2,
        ColorType::Rgb8 | ColorType::Rgb16 | ColorType::Bgr8 => 3,
        ColorType::Rgba8 | ColorType::Bgra8 | ColorType::Rgba16 => 4,
        ColorType::Nil => 0,
    };

    let mut compression_params = CompressorParams::default();

    compression_params.set_basis_format(compression_desc.compression_format.into());

    //Set the compression quality
    match compression_desc.compression_quality {
        CompressionQuality::Minimum => {
            compression_params.set_etc1s_quality_level(basis_universal::ETC1S_QUALITY_MIN);
            compression_params.set_uastc_quality_level(basis_universal::UASTC_QUALITY_MIN);
        }
        CompressionQuality::Maximum => {
            compression_params.set_etc1s_quality_level(basis_universal::ETC1S_QUALITY_MAX);
            compression_params.set_uastc_quality_level(basis_universal::UASTC_QUALITY_MAX);
        }
        _ => {} //Default compression quality for ETC1S and UASTC4x4 is set on creation
    }

    compression_params.set_color_space(compression_desc.color_space.into());

    if let Some(mip_desc) = compression_desc.mip_map_desc {
        //Set the mip map config
        compression_params.set_generate_mipmaps(mip_desc.generate_mipmap);
        compression_params.set_mip_color_space(mip_desc.color_space.into());
        compression_params.set_mipmap_smallest_dimension(mip_desc.smallest_dimensions);
    }

    if let Some(rdo_desc) = compression_desc.rdo_desc {
        //Set the rdo config
        compression_params.set_rdo_uastc(rdo_desc.rdo_uastc_quality_scalar);
        compression_params.set_no_endpoint_rdo(rdo_desc.no_endpoint_rdo);
        compression_params.set_no_selector_rdo(rdo_desc.no_selector_rdo);
    }

    let mut compress_image = compression_params.source_image_mut(0);

    compress_image.init(
        &texture.data,
        texture.size.width,
        texture.size.height,
        channel_count,
    );

    let thread_count: usize = thread_op.into();
    let mut compressor = Compressor::new(thread_count as u32);

    unsafe {
        compressor.init(&compression_params);
        compressor.process().unwrap();
    }

    let basis_file = compressor.basis_file();
    let basis_size = compressor.basis_file_size();

    BasisTexture {
        data: basis_file.to_vec(),
        file_size: basis_size,
    }
}

//KTX file is not supported.
//TODO ffi causes heap corruption. Source : CompressorParams dropping and Compressor.Init. Description: exit code: 0xc0000374, STATUS_HEAP_CORRUPTION
#[cfg(test)]
mod basis_compression_test {
    use crate::texture::compression::{
        BasisCompressionFormat, CompressionDescriptor, CompressionQuality,
    };
    use crate::{
        super_compress, ColorSpace, JpgTextureLoader, TextureDescriptor, JPG_TEST_TEXTURE,
    };
    use fabled_core::concurrent::thread_op::ThreadOperation;

    #[test]
    fn compression_test() {
        let jpg_loader = JpgTextureLoader::default();
        let jpgyellow = jpg_loader
            .load(
                JPG_TEST_TEXTURE,
                &TextureDescriptor {
                    flip_axis: Default::default(),
                },
            )
            .unwrap();

        let basis_texture = super_compress(
            &jpgyellow,
            &CompressionDescriptor {
                compression_format: BasisCompressionFormat::UASTC4x4,
                compression_quality: CompressionQuality::Default,
                color_space: ColorSpace::GammaSpace,
                mip_map_desc: None,
                rdo_desc: None,
            },
            ThreadOperation::Automatic,
        );

        std::fs::write("D:\\Study\\Fabled Engine\\crates\\mani\\fabled_render\\src\\texture\\texture\\test\\albedo\\basisyellow.basis", basis_texture.data).unwrap();
    }
}
