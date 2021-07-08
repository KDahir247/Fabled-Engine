pub trait ValidationLayer {
    fn validate(
        &self,
        flag: naga::valid::ValidationFlags,
    ) -> anyhow::Result<naga::valid::ModuleInfo>;
}
