use std::any::{type_name, TypeId};

#[derive(Clone, Default)]
pub struct PluginId(Vec<(TypeId, &'static str)>);
impl PluginId {
    pub(crate) fn contains(&self, type_id: TypeId) -> bool {
        self.0
            .iter()
            .any(|(existing_type_id, _)| type_id.eq(existing_type_id))
    }
    pub(crate) fn push<T: 'static>(&mut self) {
        self.0.push((TypeId::of::<T>(), type_name::<T>()));
    }
    pub(crate) fn pop(&mut self) {
        self.0.pop();
    }
}

impl std::fmt::Debug for PluginId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;
        std::fmt::Display::fmt(&self, f)?;
        f.write_str(")")?;
        Ok(())
    }
}

impl std::fmt::Display for PluginId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut needs_separator = false;
        for (_, type_name) in self.0.iter() {
            if needs_separator {
                f.write_str(" → ")?;
            } else {
                needs_separator = true;
            }
            f.write_str(*type_name)?;
        }
        Ok(())
    }
}
