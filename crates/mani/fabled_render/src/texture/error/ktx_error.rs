use crate::texture::_core::fmt::Formatter;
use libktx_rs::KtxError;
use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KTXError {
    KTXError(libktx_rs::KtxError),
}


impl Display for KTXError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let description = Self::retrieve_error(self);

        let KTXError::KTXError(ty) = self;

        write!(f, "{}, Error Code {}", description, ty)
    }
}


impl KTXError {
    pub fn retrieve_error(ktx_error: &KTXError) -> String {
        match ktx_error {
            KTXError::KTXError(ktx_err) => match ktx_err {
                KtxError::FileDataError => "KTX FileDataError".to_string(),
                KtxError::FileIsPipe => "KTX FileIsPipe".to_string(),
                KtxError::FileOpenFailed => "KTX FileOpenFailed".to_string(),
                KtxError::FileOverflow => "KTX FileOverflow".to_string(),
                KtxError::FileReadError => "KTX FileReadError".to_string(),
                KtxError::FileSeekError => "KTX FileSeekError".to_string(),
                KtxError::FileUnexpectedEof => "KTX FileUnexpectedEof".to_string(),
                KtxError::FileWriteError => "KTX FileWriteError".to_string(),
                KtxError::GlError => "KTX GlError".to_string(),
                KtxError::InvalidOperation => "KTX InvalidOperation".to_string(),
                KtxError::InvalidValue => "KTX InvalidValue".to_string(),
                KtxError::NotFound => "KTX NotFound".to_string(),
                KtxError::OutOfMemory => "KTX OutOfMemory".to_string(),
                KtxError::TranscodeFailed => "KTX TranscodeFailed".to_string(),
                KtxError::UnknownFileFormat => "KTX UnknownFileFormat".to_string(),
                KtxError::UnsupportedTextureType => "KTX UnsupportedTextureType".to_string(),
                KtxError::UnsupportedFeature => "KTX UnsupportedFeature".to_string(),
                KtxError::LibraryNotLinked => {
                    panic!("KTX Library is not linked!")
                }
            },
        }
    }
}
