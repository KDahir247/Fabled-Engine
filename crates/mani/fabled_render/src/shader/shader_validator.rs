use super::validation_rule::*;
use crate::shader::ValidationError;
use naga::valid::ValidationFlags;

// This will be more in-depth to convert handle to specific variable type from
// naga from error.
impl ValidationLayer for naga::Module {
    fn validate(&self, flag: ValidationFlags) -> Result<naga::valid::ModuleInfo, ValidationError> {
        naga::valid::Validator::new(flag, naga::valid::Capabilities::all())
            .validate(self)
            .map_err(ValidationError::ValidationFailedError)
    }
}
