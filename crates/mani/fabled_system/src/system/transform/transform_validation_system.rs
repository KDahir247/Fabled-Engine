use fabled_transform::{Position, Rotation, Scale};
use shipyard::*;
pub fn transform_validation_system(
    mut entity: shipyard::EntitiesViewMut,
    position: shipyard::View<Position>,
    rotation: shipyard::View<Rotation>,
    scale: shipyard::View<Scale>,
) {
    (!&position, !&rotation, !&scale)
        .fast_iter()
        .with_id()
        .for_each(|(entity_id, _)| {
            if entity.is_alive(entity_id) {
                entity.clear();

                // We need to add local to world.
                // We need to add world to local.
                // Both local and world will be the same, since
                // we are clearing the entity. Entity will not have a parent.
                // Add transform.
            }
        });
}


#[cfg(test)]
mod transform_validation_test {


    #[test]
    fn transform_validation() {}
}
