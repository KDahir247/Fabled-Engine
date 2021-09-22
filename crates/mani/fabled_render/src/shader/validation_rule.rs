use crate::shader::ValidationError;

pub trait ValidationLayer {
    fn validate(
        &self,
        flag: naga::valid::ValidationFlags,
    ) -> Result<naga::valid::ModuleInfo, ValidationError>;
}
