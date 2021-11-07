use thiserror::*;

#[derive(Debug, Error)]
pub enum ObjError {
    #[error(
        "Error occurred while loading either obj or mtl file.\nMessage : {:?}",
        0.
    )]
    ObjError(tobj::LoadError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
