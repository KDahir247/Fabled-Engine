use crate::{Missing, MissingTy};
use fabled_transform::{Rotation, Scale, Space, SpaceType, Translation};
use shipyard::*;

pub fn removed_deleted_transform_system(
    mut translation: shipyard::ViewMut<Translation>,
    mut rotation: shipyard::ViewMut<Rotation>,
    mut scale: shipyard::ViewMut<Scale>,
    mut space: shipyard::ViewMut<Space>,
) {
    let (removed_position, deleted_position) = translation.take_removed_and_deleted();

    {
        removed_position.iter().for_each(|&entity_id| {
            translation.add_component_unchecked(entity_id, Translation::default());
        });

        deleted_position
            .iter()
            .for_each(|&(entity_id, deleted_position)| {
                translation.add_component_unchecked(entity_id, deleted_position);
            });
    }

    let (removed_rotation, deleted_rotation) = rotation.take_removed_and_deleted();

    {
        removed_rotation.iter().for_each(|&entity_id| {
            rotation.add_component_unchecked(entity_id, Rotation::default());
        });

        deleted_rotation
            .iter()
            .for_each(|&(entity_id, deleted_rotation)| {
                rotation.add_component_unchecked(entity_id, deleted_rotation)
            });
    }

    let (removed_scale, deleted_scale) = scale.take_removed_and_deleted();

    {
        removed_scale.iter().for_each(|&entity_id| {
            scale.add_component_unchecked(entity_id, Scale::default());
        });

        deleted_scale
            .iter()
            .for_each(|&(entity_id, deleted_scale)| {
                scale.add_component_unchecked(entity_id, deleted_scale);
            });
    }

    let (removed_space, deleted_space) = space.take_removed_and_deleted();

    {
        removed_space.iter().for_each(|&entity_id| {
            space.add_component_unchecked(
                entity_id,
                Space {
                    value: SpaceType::World,
                },
            )
        });

        deleted_space
            .iter()
            .for_each(|&(entity_id, deleted_space)| {
                space.add_component_unchecked(entity_id, deleted_space);
            });
    }
}


pub fn missing_core_transform_system(
    entity: EntitiesView,
    mut missing: ViewMut<Missing>,
    translation: ViewMut<Translation>,
    rotation: ViewMut<Rotation>,
    scale: ViewMut<Scale>,
    mut space: shipyard::ViewMut<Space>,
) {
    (!&translation).iter().with_id().for_each(|(entity_id, _)| {
        if let Ok(mut missing) = (&mut missing).get(entity_id) {
            missing.ty.remove(MissingTy::TRANSLATION);
            missing.ty.insert(MissingTy::TRANSLATION);
        } else {
            missing.add_component_unchecked(
                entity_id,
                Missing {
                    ty: MissingTy::TRANSLATION,
                },
            )
        }
    });

    // iter over rotation component if there is no rotation component for the entity
    // then we will try to get the missing component and insert the missing type
    // as Rotation. If there is no missing component then we will add a missing
    // component and set the type to Rotation.
    // This will happen for scale and space.

    (!&rotation).iter().with_id().for_each(|(entity_id, _)| {
        if let Ok(mut missing) = (&mut missing).get(entity_id) {
            missing.ty.remove(MissingTy::ROTATION);
            missing.ty.insert(MissingTy::ROTATION);
        } else {
            missing.add_component_unchecked(
                entity_id,
                Missing {
                    ty: MissingTy::ROTATION,
                },
            )
        }
    });

    (!&scale).iter().with_id().for_each(|(entity_id, _)| {
        if let Ok(mut missing) = (&mut missing).get(entity_id) {
            //
        } else {
            missing.add_component_unchecked(
                entity_id,
                Missing {
                    ty: MissingTy::SCALE,
                },
            )
        }
    });

    // Once we validate translation, rotation, scale, and space we will iterate over
    // entities with the missing component and compare the bit and add the
    // correct missing component depending on the missing bit. We will then have to
    // delete/remove all the missing component added to the entities.


    // Check if the entity doesn't have a position if it doesn't then add a Position
    // component.

    // Check if the entity doesn't have a rotation if it doesn't then
    // add a Rotation component.

    // Check if the entity doesn't have a scale if it doesn't then add a Scale
    // component.
    (!&translation, !&rotation, !&scale)
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
    use crate::{Missing, MissingTy};
    use shipyard::{EntityId, World};

    #[test]
    fn transform_validation() {
        let mut world = World::new();
    }
}
