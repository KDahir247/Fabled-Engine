use fabled_transform::{Position, Rotation, Scale, Space, SpaceType};
use shipyard::*;

pub fn transform_validation_system(
    entity: shipyard::EntitiesViewMut,
    position: shipyard::ViewMut<Position>,
    rotation: shipyard::ViewMut<Rotation>,
    scale: shipyard::ViewMut<Scale>,
    mut space: shipyard::ViewMut<Space>,
) {
    // We will be tracking position, rotation and scale for updating LocalToWorld
    // matrix and WorldToLocal matrix in the designated system.


    (!&position, !&rotation, !&scale)
        .to_owned()
        .iter()
        .with_id()
        .for_each(|(entity_id, _)| {
            if entity.is_alive(entity_id) {
                // todo 1. add position
                //  2. add rotation
                //  3. add scale.

                space.add_component_unchecked(
                    entity_id,
                    Space {
                        value: SpaceType::World,
                    },
                )
                // Add transform.
                // I need the world
            }
        });
}


#[cfg(test)]
mod transform_validation_test {


    #[test]
    fn transform_validation() {}
}
