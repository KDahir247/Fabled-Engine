use shipyard::*;

pub trait AddDistinct: AddComponent {
    /// Assign this component if the value is distinct from that which is already in the storage.
    ///
    ///  * If the component doesn't exist already, it will be inserted.
    ///  * If the component already exists, and is not equal the component will be modified to the new value.
    ///  * If the component already exists, and is equal no mutation will occur (`update_pack` will remain clean).
    fn add_distinct(&mut self, entity: EntityId, component: Self::Component) -> bool;
}

impl<T: Component + PartialEq> AddDistinct for ViewMut<'_, T> {
    fn add_distinct(&mut self, entity: EntityId, component: Self::Component) -> bool {
        if let Ok(has_value) = (&*self).get(entity) {
            if &component == has_value {
                return false;
            }
        }

        self.add_component_unchecked(entity, component);
        true
    }
}
