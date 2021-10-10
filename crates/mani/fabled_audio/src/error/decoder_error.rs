use thiserror::*;

#[derive(Debug, Error)]
pub enum AudioDecodingError {
    // Hound Specific Error
    #[error("Wav Error has occurred.See https://docs.rs/hound/3.4.0/hound/enum.Error.html.\nMessage : {:?}", .0)]
    WavError(hound::Error),

    // Claxon Specific Error
    #[error("Flac Error has occurred.See https://docs.rs/claxon/0.4.3/claxon/enum.Error.html.\n Message : {}", .0)]
    FlacError(claxon::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
