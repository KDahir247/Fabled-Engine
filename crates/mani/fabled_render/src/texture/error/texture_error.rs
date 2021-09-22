use thiserror::Error;

#[derive(Error, Debug)]
pub enum TextureError {
    #[error("The container is not big enough for the texture")]
    InSufficientAllocationSize,

    #[error("Image error {}", .0)]
    ImageError(image::ImageError),
    
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
