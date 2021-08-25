use crate::texture::compression::{TranscodeDescriptor, TranscodeSource};
use crate::texture::container::{ColorType, Extent3d, Texture};

use anyhow::Context;
use basis_universal::{TranscodeParameters, Transcoder};
use image::GenericImageView;
use std::io::Read;

pub fn transcode(
    basis_texture: TranscodeSource,
    transcode_desc: &TranscodeDescriptor,
) -> anyhow::Result<Texture> {
    let source = match basis_texture {
        TranscodeSource::BasisTexture { basis } => basis.data,
        TranscodeSource::BasisPath { path: file_path } => {
            let mut buffer = Vec::new();
            let path = std::path::Path::new(file_path.as_str());
            let mut basis_file = std::fs::File::open(path)?;
            let num_buf = basis_file.read_to_end(&mut buffer)?;

            assert!(
                num_buf > 0,
                "basis file has less or equal to 0 bytes (EMPTY DATA)"
            );

            buffer
        }
    };

    let mut transcoder = Transcoder::new();
    let mip_level_count = transcoder.image_level_count(&source, 0);

    transcoder.prepare_transcoding(&source).unwrap();

    let result = transcoder
        .transcode_image_level(
            source.as_slice(),
            transcode_desc.texture_format.into(),
            TranscodeParameters {
                image_index: 0,
                level_index: mip_level_count,
                decode_flags: Some(transcode_desc.decode_flags.into()),
                output_row_pitch_in_blocks_or_pixels: transcode_desc
                    .pixel_manipulation
                    .output_row_pitch_in_blocks_or_pixels,
                output_rows_in_pixels: transcode_desc.pixel_manipulation.output_rows_in_pixels,
            },
        )
        .unwrap();

    let description = transcoder
        .image_level_description(&source, 0, mip_level_count)
        .context("Original width and height is a multiple of 4 or transcoder wasn't initialized. Retrieving image level description failed")?;

    transcoder.end_transcoding();

    let img_buf = image::RgbImage::from_raw(
        description.original_width,
        description.original_height,
        result,
    )
    .context("Creating image buffer from transcode basis image failed")?;

    let dyn_img: image::DynamicImage = image::DynamicImage::ImageRgb8(img_buf);

    Ok(Texture {
        data: dyn_img.to_bytes(),
        size: Extent3d {
            width: dyn_img.width(),
            height: dyn_img.height(),
            depth_or_array_layers: 1,
        },
        sample_count: 0,
        mip_level: 1,
        color_type: ColorType::Rgb8,
        rows_per_image: dyn_img.width() * 4,
    })
}

//KTX file is not supported.
//TODO ffi causes heap corruption
#[cfg(test)]
mod basis_transcode_test {
    use crate::texture::compression::*;
    #[test]
    fn transcoder_test() {
        transcode(TranscodeSource::BasisPath {
            path: "D:\\Study\\Fabled Engine\\crates\\mani\\fabled_render\\src\\texture\\texture\\test\\albedo\\basis_yellow.basis".to_string(),
        }, &TranscodeDescriptor::default())
            .unwrap();
    }
}