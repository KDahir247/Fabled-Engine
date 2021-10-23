use thiserror::*;

#[derive(Debug, Error)]
pub enum ObjError {
    #[error("{:?}", 0.)]
    ObjError(tobj::LoadError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
