use fabled_transform::{
    decompose_transformation_matrix, get_transformation_matrix, transform, Frozen, LocalToWorld,
    Parent, Rotation, Scale, ScaleType, Translation,
};
use shipyard::*;

pub fn local_world_system(
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
        .fast_iter()
        .for_each(
            |(translation, rotation, scale, mut local_to_world_matrix, ..)| {
                local_to_world_matrix.value =
                    get_transformation_matrix(*translation, *rotation, *scale);
            },
        );


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
            let parent_entity_id = shipyard::EntityId::from_inner(parent.value);

            let child_scale = match scale.value {
                ScaleType::Uniform(uniform) => [uniform; 3],
                ScaleType::NonUniform(non_uniform) => non_uniform,
            };

            if parent_entity_id.is_some() {
                let valid_parent_entity = parent_entity_id.unwrap();

                let parent_local_to_world_matrix = (&local_to_world_storage)
                    .fast_get(valid_parent_entity)
                    .unwrap();

                let (parent_translation, parent_rotation, parent_scale) =
                    decompose_transformation_matrix(parent_local_to_world_matrix.value);

                let parent_scale = match parent_scale.value {
                    ScaleType::Uniform(uniform) => [uniform; 3],
                    ScaleType::NonUniform(non_uniform) => non_uniform,
                };

                let child_world_scale = Scale {
                    value: ScaleType::NonUniform([
                        parent_scale[0] * child_scale[0],
                        parent_scale[1] * child_scale[1],
                        parent_scale[2] * child_scale[2],
                    ]),
                };

                let child_rotation = rotation.value;
                let parent_rotation = parent_rotation.value;

                let child_world_rotation = Rotation {
                    value: [
                        parent_rotation[3] * child_rotation[0]
                            + parent_rotation[0] * child_rotation[3]
                            + parent_rotation[1] * child_rotation[2]
                            - parent_rotation[2] * child_rotation[1],
                        parent_rotation[3] * child_rotation[1]
                            - parent_rotation[0] * child_rotation[2]
                            + parent_rotation[1] * child_rotation[3]
                            + parent_rotation[2] * child_rotation[0],
                        parent_rotation[3] * child_rotation[2]
                            + parent_rotation[0] * child_rotation[1]
                            - parent_rotation[1] * child_rotation[0]
                            + parent_rotation[2] * child_rotation[3],
                        parent_rotation[3] * child_rotation[3]
                            - parent_rotation[0] * child_rotation[0]
                            - parent_rotation[1] * child_rotation[1]
                            - parent_rotation[2] * child_rotation[2],
                    ],
                };

                let parent_translation = parent_translation.value;
                let child_translation = translation.value;

                let transform_child = transform(child_translation, parent_rotation);

                let child_world_translation = Translation {
                    value: [
                        transform_child[0] * parent_scale[0] + parent_translation[0],
                        transform_child[1] * parent_scale[1] + parent_translation[1],
                        transform_child[2] * parent_scale[2] + parent_translation[2],
                        1.0,
                    ],
                };

                let local_to_world_matrix =
                    (&mut local_to_world_storage).fast_get(entity_id).unwrap();

                local_to_world_matrix.value = get_transformation_matrix(
                    child_world_translation,
                    child_world_rotation,
                    child_world_scale,
                );

                println!("{:?}", local_to_world_matrix.value);
            } else {
                let local_to_world_matrix =
                    (&mut local_to_world_storage).fast_get(entity_id).unwrap();

                local_to_world_matrix.value =
                    get_transformation_matrix(*translation, *rotation, *scale);
            }
        });
}

#[cfg(test)]
mod local_world_test {
    use crate::system::transform::calculate_local_world_system::local_world_system;
    use fabled_transform::{LocalToWorld, Parent, ScaleType};
    use shipyard::{Get, ViewMut};

    #[test]
    fn retrieve_modified_world_matrix() {
        let mut world = shipyard::World::new();

        let entity = world.add_entity((
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
            .with_system(&local_world_system)
            .add_to_world(&world)
            .unwrap();

        world.run_workload("run_test").unwrap();

        let (parent_storage, local_to_world_storage) = world
            .borrow::<(ViewMut<Parent>, ViewMut<LocalToWorld>)>()
            .unwrap();


        if let Ok(_parent_component) = (&parent_storage).get(entity) {
            // test the result with the entity having a parent in mind.
        } else {
            let local_world_matrix = (&local_to_world_storage).get(entity).unwrap();
            println!("{:?}", local_world_matrix);
        }
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
            .with_system(&local_world_system)
            .add_to_world(&world)
            .unwrap();

        world.run_workload("run_test").unwrap();
    }
}
