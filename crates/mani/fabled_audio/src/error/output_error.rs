use thiserror::*;

#[derive(Debug, Error)]
pub enum SpatialOutputError {
    #[error("Error occurred when calling play_stream\nMessage: {:?}", .0)]
    PlayStreamError(cpal::PlayStreamError),
    #[error("No Device or Lost Device mid way while acquiring it before.")]
    NoDevice,
}
