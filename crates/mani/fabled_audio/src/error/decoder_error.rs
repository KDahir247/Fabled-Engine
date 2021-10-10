use thiserror::*;

#[derive(Debug, Error)]
pub enum AudioDecodingError {
    // Hound Specific Error
    #[error("Wav Error has occurred.See https://docs.rs/hound/3.4.0/hound/enum.Error.html.\nMessage : {:?}", .0)]
    WavError(hound::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
