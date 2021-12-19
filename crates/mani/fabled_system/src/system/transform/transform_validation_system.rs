use fabled_transform::{LocalToWorld, Rotation, Scale, Translation};
use shipyard::*;

pub fn removed_deleted_transform_system(
    mut translation_storage: shipyard::ViewMut<Translation>,
    mut rotation_storage: shipyard::ViewMut<Rotation>,
    mut scale_storage: shipyard::ViewMut<Scale>,
    mut local_to_world_storage: ViewMut<LocalToWorld>,
) {
    translation_storage.track_deletion().track_removal();
    rotation_storage.track_deletion().track_removal();
    scale_storage.track_deletion().track_removal();
    local_to_world_storage.track_deletion().track_removal();


    let (removed_position, deleted_position) = translation_storage.take_removed_and_deleted();

    {
        removed_position.iter().for_each(|&entity_id| {
            translation_storage.add_component_unchecked(entity_id, Translation::default());
        });

        deleted_position
            .iter()
            .for_each(|&(entity_id, deleted_position)| {
                translation_storage.add_component_unchecked(entity_id, deleted_position);
            });
    }

    let (removed_rotation, deleted_rotation) = rotation_storage.take_removed_and_deleted();

    {
        removed_rotation.iter().for_each(|&entity_id| {
            rotation_storage.add_component_unchecked(entity_id, Rotation::default());
        });

        deleted_rotation
            .iter()
            .for_each(|&(entity_id, deleted_rotation)| {
                rotation_storage.add_component_unchecked(entity_id, deleted_rotation)
            });
    }

    let (removed_scale, deleted_scale) = scale_storage.take_removed_and_deleted();

    {
        removed_scale.iter().for_each(|&entity_id| {
            scale_storage.add_component_unchecked(entity_id, Scale::default());
        });

        deleted_scale
            .iter()
            .for_each(|&(entity_id, deleted_scale)| {
                scale_storage.add_component_unchecked(entity_id, deleted_scale);
            });
    }


    let (removed_local_world, deleted_local_world) =
        local_to_world_storage.take_removed_and_deleted();

    {
        removed_local_world.iter().for_each(|&entity_id| {
            local_to_world_storage.add_component_unchecked(entity_id, LocalToWorld::default());
        });

        deleted_local_world
            .iter()
            .for_each(|&(entity_id, deleted_local_world)| {
                local_to_world_storage.add_component_unchecked(entity_id, deleted_local_world);
            });
    }
}

pub fn missing_core_transform_system(
    entity: EntitiesView,
    mut translation_storage: ViewMut<Translation>,
    mut rotation_storage: ViewMut<Rotation>,
    mut scale_storage: ViewMut<Scale>,
    mut local_to_world_storage: ViewMut<LocalToWorld>,
) {
    (&entity).iter().for_each(|entity_id| {
        if !translation_storage.contains(entity_id) {
            translation_storage.add_component_unchecked(entity_id, Translation::default());
        }

        if !rotation_storage.contains(entity_id) {
            rotation_storage.add_component_unchecked(entity_id, Rotation::default());
        }

        if !scale_storage.contains(entity_id) {
            scale_storage.add_component_unchecked(entity_id, Scale::default());
        }
        if !local_to_world_storage.contains(entity_id) {
            local_to_world_storage.add_component_unchecked(entity_id, LocalToWorld::default());
        }
    });
}


#[cfg(test)]
mod transform_validation_test {

    use crate::system::transform::transform_validation_system::{
        missing_core_transform_system, removed_deleted_transform_system,
    };
    use fabled_transform::{LocalToWorld, Rotation, Scale, Translation};
    use shipyard::{Get, World};

    #[test]
    fn transform_validation() {
        let mut world = World::new();

        let entity_id = world.add_entity((Translation::default(),));

        shipyard::Workload::builder("run_test")
            .with_system(&missing_core_transform_system)
            .with_system(&removed_deleted_transform_system)
            .add_to_world(&world)
            .unwrap();

        world.run_workload("run_test").unwrap();

        let (local_world_storage, scale_storage, rotation_storage) = world
            .borrow::<(
                shipyard::View<LocalToWorld>,
                shipyard::View<Scale>,
                shipyard::View<Rotation>,
            )>()
            .unwrap();

        (&local_world_storage).get(entity_id).unwrap();
        (&scale_storage).get(entity_id).unwrap();
        (&rotation_storage).get(entity_id).unwrap();
    }
}
