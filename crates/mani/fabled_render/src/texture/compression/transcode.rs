use crate::TranscodeSource;

use basis_universal::{TranscodeParameters, Transcoder, TranscoderTextureFormat};
use std::io::Read;

//todo not complete
pub fn transcode(basis_texture: TranscodeSource) {
    let source = match basis_texture {
        TranscodeSource::BasisTexture { basis } => basis.data,
        TranscodeSource::BasisPath { path: file_path } => {
            let mut buffer = Vec::new();
            let path = std::path::Path::new(file_path.as_str());
            let mut basis_file = std::fs::File::open(path).unwrap();
            let num_buf = basis_file.read_to_end(&mut buffer).unwrap();

            assert!(num_buf > 0);

            buffer
        }
    };

    let mut transcoder = Transcoder::new();

    let mip_level_count = transcoder.image_level_count(&source, 0);

    transcoder.prepare_transcoding(&source).unwrap();
    let result = transcoder
        .transcode_image_level(
            source.as_slice(),
            TranscoderTextureFormat::RGB565,
            TranscodeParameters {
                image_index: 0,
                level_index: mip_level_count,
                ..Default::default()
            },
        )
        .unwrap();

    let description = transcoder
        .image_level_description(&source, 0, mip_level_count)
        .unwrap();

    let image = image::RgbImage::from_raw(
        description.original_width,
        description.original_height,
        result,
    )
    .unwrap();
}

//KTX file is not supported.
//TODO ffi causes heap corruption
#[cfg(test)]
mod basis_transcode_test {

    use crate::transcode;
    use crate::transcode::TranscodeSource;
    #[test]
    fn transcoder_test() {
        transcode(TranscodeSource::BasisPath {
            path: "D:\\Study\\Fabled Engine\\crates\\mani\\fabled_render\\src\\texture\\texture\\test\\albedo\\basisyellow.basis".to_string(),
        });
    }
}
