use crate::{startup, World};

pub fn create_entity(world: Option<World>) -> u64 {
    let mut world = world.unwrap_or_default();
    let entity_id = world.value.add_entity(());
    entity_id.inner()
    // we need a world
}
