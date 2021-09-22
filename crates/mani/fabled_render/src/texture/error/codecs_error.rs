use crate::texture::_core::fmt::Formatter;
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug)]
pub enum FlipType {
    Horizontal,
    Vertical,
}

impl Display for FlipType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Error, Debug)]
pub enum CodecsError {
    #[error("UnSupported Flip Operation Texture can't flip {}", .0)]
    InvalidFlipOperationError(FlipType),

    #[error("Image Error either from dimension mismatch, a mismatch file extension or missing required signature \nSpecific Error: {:?}", .0)]
    ImageError(image::ImageError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
