use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageProcError {
    #[error("The container is not big enough for the texture")]
    InSufficientAllocationSize,
    #[error("Can not determine the color type for the specific texture.\n Texture file might be corrupt")]
    UndefinedColorType,
}
