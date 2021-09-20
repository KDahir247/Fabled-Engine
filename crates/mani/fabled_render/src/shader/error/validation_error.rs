use thiserror::Error;


// This will be more in-depth to convert handle to specific variable type from
// naga.
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Shader validation failed from\n{}", .0)]
    ValidationFailedError(naga::valid::ValidationError),
}
