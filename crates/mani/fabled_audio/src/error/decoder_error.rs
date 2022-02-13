use thiserror::*;

#[derive(Debug, Error)]
pub enum AudioDecodingError {
    #[error("Unrecognized audio format type.\nThe support audio format are flac, mp3, ogg, wav")]
    DecoderError(rodio::decoder::DecoderError),

    #[error("Wav Error has occurred. See https://docs.rs/hound/3.4.0/hound/enum.Error.html.\nMessage : {:?}", .0)]
    WavError(hound::Error),

    #[error("Flac Error has occurred. See https://docs.rs/claxon/0.4.3/claxon/enum.Error.html.\n Message : {}", .0)]
    FlacError(claxon::Error),

    #[error("Ogg Error has occurred. See https://docs.rs/lewton/0.10.2/lewton/enum.VorbisError.html.\n Message : {}", .0)]
    OggError(lewton::VorbisError),

    #[error("Mp3 Error has occurred. See https://docs.rs/minimp3/0.5.1/minimp3/enum.Error.html.\n Message : {}", .0)]
    Mp3Error(minimp3::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
