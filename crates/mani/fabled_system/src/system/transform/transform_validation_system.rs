use fabled_transform::{LocalToWorld, Rotation, Scale, Translation};
use shipyard::*;

pub fn removed_deleted_transform_system(
    mut translation_storage: shipyard::ViewMut<Translation>,
    mut rotation_storage: shipyard::ViewMut<Rotation>,
    mut scale_storage: shipyard::ViewMut<Scale>,
    mut local_to_world_storage: ViewMut<LocalToWorld>,
) {
    let (removed_translation_entities, deleted_translation_metas) =
        translation_storage.take_removed_and_deleted();

    {
        removed_translation_entities
            .iter()
            .for_each(|&removed_translation_entity_id| {
                translation_storage
                    .add_component_unchecked(removed_translation_entity_id, Translation::default());
            });

        deleted_translation_metas.iter().for_each(
            |&(deleted_translation_entity_id, deleted_translation)| {
                translation_storage
                    .add_component_unchecked(deleted_translation_entity_id, deleted_translation);
            },
        );
    }

    let (removed_rotation_entities, deleted_rotation_metas) =
        rotation_storage.take_removed_and_deleted();

    {
        removed_rotation_entities
            .iter()
            .for_each(|&removed_rotation_entity_id| {
                rotation_storage
                    .add_component_unchecked(removed_rotation_entity_id, Rotation::default());
            });

        deleted_rotation_metas.iter().for_each(
            |&(deleted_rotation_entity_id, deleted_rotation)| {
                rotation_storage
                    .add_component_unchecked(deleted_rotation_entity_id, deleted_rotation)
            },
        );
    }

    let (removed_scale_entities, deleted_scale_metas) = scale_storage.take_removed_and_deleted();

    {
        removed_scale_entities
            .iter()
            .for_each(|&removed_scale_entity_id| {
                scale_storage.add_component_unchecked(removed_scale_entity_id, Scale::default());
            });

        deleted_scale_metas
            .iter()
            .for_each(|&(deleted_scale_entity_id, deleted_scale)| {
                scale_storage.add_component_unchecked(deleted_scale_entity_id, deleted_scale);
            });
    }


    let (removed_local_world_entities, deleted_local_world_metas) =
        local_to_world_storage.take_removed_and_deleted();

    {
        removed_local_world_entities
            .iter()
            .for_each(|&removed_local_world_entity_id| {
                local_to_world_storage.add_component_unchecked(
                    removed_local_world_entity_id,
                    LocalToWorld::default(),
                );
            });

        deleted_local_world_metas.iter().for_each(
            |&(deleted_local_world_entity_id, deleted_local_world)| {
                local_to_world_storage
                    .add_component_unchecked(deleted_local_world_entity_id, deleted_local_world);
            },
        );
    }
}


#[cfg(test)]
mod transform_validation_test {

    use crate::system::transform::transform_validation_system::removed_deleted_transform_system;
    use crate::TransformPlugin;
    use fabled_transform::{LocalToWorld, Rotation, Scale, Translation};
    use shipyard::{Get, Remove, World};

    #[test]
    fn transform_validation() {
        let mut world = World::new();


        let entity_id = world.add_entity((
            Translation::default(),
            Rotation::default(),
            Scale::default(),
            LocalToWorld::default(),
        ));
        {
            let (mut local_world_storage, mut scale_storage, mut rotation_storage) = world
                .borrow::<(
                    shipyard::ViewMut<LocalToWorld>,
                    shipyard::ViewMut<Scale>,
                    shipyard::ViewMut<Rotation>,
                )>()
                .unwrap();

            rotation_storage.remove(entity_id);
            scale_storage.remove(entity_id);
            local_world_storage.remove(entity_id);
        }

        let app = shipyard_app::App::new_with_world(world);
        let mut app_builder = shipyard_app::AppBuilder::new(&app);
        app_builder.add_plugin(TransformPlugin::default());
        app_builder.finish().run(&app);


        {
            let (local_world_storage, scale_storage, rotation_storage) = app
                .world
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
}
