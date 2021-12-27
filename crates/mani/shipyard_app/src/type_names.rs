use std::{
    any::{type_name, TypeId},
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Clone, Default)]
pub(crate) struct TypeNames(Arc<RwLock<HashMap<TypeId, &'static str>>>);

impl TypeNames {
    pub(crate) fn tracked_type_id_of<T: 'static>(&self) -> TypeId {
        let type_id = TypeId::of::<T>();
        self.0
            .try_write()
            .expect("all mine!")
            .entry(type_id)
            .or_insert_with(type_name::<T>);
        type_id
    }

    pub fn lookup_name(&self, type_id: &TypeId) -> Option<&'static str> {
        self.0
            .try_read()
            .expect("all mine!")
            .get(type_id)
            .map(|a| *a as &'static str)
    }
}
