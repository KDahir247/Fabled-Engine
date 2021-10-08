use thiserror::*;

#[derive(Debug, Error)]
pub enum AudioEncodingError {
    #[error("")]
    NoDeviceError,
    #[error("")]
    NoInputConfigError,
    #[error("")]
    NoOutputConfigError,
    #[error("")]
    BuildStreamError(cpal::BuildStreamError)
}
