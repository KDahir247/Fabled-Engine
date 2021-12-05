use fabled_transform::{
    decompose_transformation_matrix, get_transformation_matrix, transform, Frozen, LocalToWorld,
    Parent, Rotation, Scale, ScaleType, Translation,
};
use rayon::iter::ParallelIterator;

use shipyard::*;

pub fn calculate_local_world_system(
    translation_storage: View<Translation>,
    rotation_storage: View<Rotation>,
    scale_storage: View<Scale>,
    frozen_storage: View<Frozen>,
    parent_storage: View<Parent>,
    mut local_to_world_storage: ViewMut<LocalToWorld>,
) {
    (
        &translation_storage,
        &rotation_storage,
        &scale_storage,
        &mut local_to_world_storage,
        !&parent_storage,
        !&frozen_storage,
    )
        .fast_par_iter()
        .for_each(|(translation, rotation, scale, mut local_world, _, _)| {
            local_world.value = get_transformation_matrix(*translation, *rotation, *scale);
        });
}

pub fn calculate_local_world_parent_system(
    translation_storage: View<Translation>,
    rotation_storage: View<Rotation>,
    scale_storage: View<Scale>,
    frozen_storage: View<Frozen>,
    parent_storage: View<Parent>,
    mut local_to_world_storage: ViewMut<LocalToWorld>,
) {
    (
        &translation_storage,
        &rotation_storage,
        &scale_storage,
        &parent_storage,
        !&frozen_storage,
    )
        .fast_iter()
        .with_id()
        .for_each(|(entity_id, (translation, rotation, scale, parent, _))| {
            let identity_matrix = LocalToWorld::default();

            let parent_entity_id =
                EntityId::from_inner(parent.value).unwrap_or_else(EntityId::dead);

            let parent_local_to_world_matrix = (&local_to_world_storage)
                .fast_get(parent_entity_id)
                .unwrap_or(&identity_matrix);

            let (parent_world_translation, parent_world_rotation, parent_world_scale) =
                decompose_transformation_matrix(parent_local_to_world_matrix.value);

            let parent_world_translation = parent_world_translation.value;
            let parent_world_rotation = parent_world_rotation.value;
            let parent_world_scale = match parent_world_scale.value {
                ScaleType::Uniform(uniform) => [uniform; 3],
                ScaleType::NonUniform(non_uniform) => non_uniform,
            };

            let child_local_translation = translation.value;
            let child_local_rotation = rotation.value;
            let child_local_scale = match scale.value {
                ScaleType::Uniform(uniform) => [uniform; 3],
                ScaleType::NonUniform(non_uniform) => non_uniform,
            };

            let transform_child = transform(child_local_translation, parent_world_rotation);

            let child_world_translation = Translation {
                value: [
                    transform_child[0] * parent_world_scale[0] + parent_world_translation[0],
                    transform_child[1] * parent_world_scale[1] + parent_world_translation[1],
                    transform_child[2] * parent_world_scale[2] + parent_world_translation[2],
                    1.0,
                ],
            };

            let child_world_rotation = Rotation {
                value: [
                    parent_world_rotation[3] * child_local_rotation[0]
                        + parent_world_rotation[0] * child_local_rotation[3]
                        + parent_world_rotation[1] * child_local_rotation[2]
                        - parent_world_rotation[2] * child_local_rotation[1],
                    //
                    parent_world_rotation[3] * child_local_rotation[1]
                        - parent_world_rotation[0] * child_local_rotation[2]
                        + parent_world_rotation[1] * child_local_rotation[3]
                        + parent_world_rotation[2] * child_local_rotation[0],
                    //
                    parent_world_rotation[3] * child_local_rotation[2]
                        + parent_world_rotation[0] * child_local_rotation[1]
                        - parent_world_rotation[1] * child_local_rotation[0]
                        + parent_world_rotation[2] * child_local_rotation[3],
                    //
                    parent_world_rotation[3] * child_local_rotation[3]
                        - parent_world_rotation[0] * child_local_rotation[0]
                        - parent_world_rotation[1] * child_local_rotation[1]
                        - parent_world_rotation[2] * child_local_rotation[2],
                ],
            };


            let child_world_scale = Scale {
                value: ScaleType::NonUniform([
                    parent_world_scale[0] * child_local_scale[0],
                    parent_world_scale[1] * child_local_scale[1],
                    parent_world_scale[2] * child_local_scale[2],
                ]),
            };

            let child_local_to_world_matrix =
                (&mut local_to_world_storage).fast_get(entity_id).unwrap();

            child_local_to_world_matrix.value = get_transformation_matrix(
                child_world_translation,
                child_world_rotation,
                child_world_scale,
            );
        });
}

#[cfg(test)]
mod local_world_test {
    use crate::system::transform::calculate_local_world_system::{
        calculate_local_world_parent_system, calculate_local_world_system,
    };
    use fabled_transform::{Parent, ScaleType};

    #[test]
    fn retrieve_modified_world_matrix() {
        let mut world = shipyard::World::new();

        world.add_entity((
            fabled_transform::Translation::default(),
            fabled_transform::Scale {
                value: ScaleType::Uniform(1.0),
            },
            fabled_transform::Rotation {
                value: [0.3097265, 0.2101103, 0.5141426, 0.7717387],
            },
            fabled_transform::LocalToWorld::default(),
        ));

        shipyard::Workload::builder("run_test")
            .with_system(&calculate_local_world_system)
            .add_to_world(&world)
            .unwrap();

        world.run_workload("run_test").unwrap();
    }

    #[test]
    fn parent_entity() {
        let mut world = shipyard::World::new();

        let aentity_parent = world.add_entity((
            fabled_transform::Translation {
                value: [44.42781, 13.65856, 41.4725, 1.0],
            },
            fabled_transform::Scale {
                value: ScaleType::Uniform(2.0),
            },
            fabled_transform::Rotation {
                value: [0.0, 0.0, 0.0, 1.0],
            },
            fabled_transform::LocalToWorld::default(),
        ));


        let entity_parent = world.add_entity((
            fabled_transform::Translation {
                value: [34.42781, -13.65856, -1.472504, 1.0],
            },
            fabled_transform::Scale {
                value: ScaleType::Uniform(3.0),
            },
            fabled_transform::Rotation {
                value: [0.3097265, 0.2101103, 0.5141426, 0.7717387],
            },
            fabled_transform::LocalToWorld::default(),
            fabled_transform::Parent {
                value: aentity_parent.inner(),
            },
        ));


        let _entity_child = world.add_entity((
            fabled_transform::Translation {
                value: [12.2, 9.45, 11.0, 1.0],
            },
            fabled_transform::Scale {
                value: ScaleType::Uniform(2.0),
            },
            fabled_transform::Rotation {
                value: [0.3097265, 0.2101103, 0.5141426, 0.7717387],
            },
            fabled_transform::LocalToWorld::default(),
            Parent {
                value: entity_parent.inner(),
            },
        ));

        shipyard::Workload::builder("run_test")
            .with_system(&calculate_local_world_system)
            .with_system(&calculate_local_world_parent_system)
            .add_to_world(&world)
            .unwrap();

        world.run_workload("run_test").unwrap();
    }
}
