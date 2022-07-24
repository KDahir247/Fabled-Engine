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
        .par_iter()
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
        .iter()
        .with_id()
        .for_each(|(entity_id, (translation, rotation, scale, parent, _))| {
            let identity_matrix = LocalToWorld::default();

            let parent_entity_id =
                EntityId::from_inner(parent.value).unwrap_or_else(EntityId::dead);

            let parent_local_to_world_matrix = (&local_to_world_storage)
                .get(parent_entity_id)
                .unwrap_or(&identity_matrix);

            let (parent_world_translation, parent_world_rotation, parent_world_scale) =
                decompose_transformation_matrix(parent_local_to_world_matrix.value);

            let parent_world_translation = parent_world_translation.value;

            let rcp_parent_world_scalar = 1.0 / (parent_world_translation[3] + f32::EPSILON);

            let parent_world_translation = [
                parent_world_translation[0] * rcp_parent_world_scalar,
                parent_world_translation[1] * rcp_parent_world_scalar,
                parent_world_translation[2] * rcp_parent_world_scalar,
            ];

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

            let transform_child = transform(child_local_translation, parent_world_rotation.into());

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
                    parent_world_rotation[3] * child_local_rotation[0] // 0
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

            let mut child_local_to_world_matrix =
                (&mut local_to_world_storage).get(entity_id).unwrap();


            child_local_to_world_matrix.value = get_transformation_matrix(
                child_world_translation,
                child_world_rotation,
                child_world_scale,
            );
        });
}

#[cfg(test)]
mod local_world_test {
    use crate::system::transform::local_world_system::{
        calculate_local_world_parent_system, calculate_local_world_system,
    };
    use fabled_transform::{LocalToWorld, Parent, ScaleType};
    use shipyard::Get;

    #[test]
    fn retrieve_modified_world_matrix() {
        const THRESHOLD: f32 = 0.00001;

        let mut world = shipyard::World::new();

        let entity = world.add_entity((
            fabled_transform::Translation {
                value: [94.53416, 50.44112, 26.40424, 1.0],
            },
            fabled_transform::Scale {
                value: ScaleType::NonUniform([1.0, 2.1, 11.1]),
            },
            fabled_transform::Rotation {
                value: [0.3097265, 0.2101103, 0.5141426, 0.7717387],
            },
            LocalToWorld::default(),
        ));

        shipyard::Workload::builder("run_test")
            .with_system(&calculate_local_world_system)
            .add_to_world(&world)
            .unwrap();

        world.run_workload("run_test").unwrap();

        let local_world_storage = world.borrow::<shipyard::View<LocalToWorld>>().unwrap();

        let local_world = (&local_world_storage).get(entity).unwrap();

        let local_world_inner = local_world.value;

        // Unity's result (our matrix is column-major)
        //  0.38302	 -1.39317   7.13494	  94.53416
        //  0.92372	  0.58685  -2.90823	  50.44112
        // -0.00581	  1.45763   7.99029	  26.40424
        //  0.00000	  0.00000   0.00000	  1.00000

        let proven_result = [
            0.38302, 0.92372, -0.00581, 0.00000, // col 0
            -1.39317, 0.58685, 1.45763, 0.00000, // col 1
            7.13494, -2.90823, 7.99029, 0.00000, // col 2
            94.53416, 50.44112, 26.40424, 1.00000, // col 3
        ];

        for each in 0..16_usize {
            assert!((local_world_inner[each] - proven_result[each]).abs() <= THRESHOLD);
        }
    }

    #[test]
    fn retrieve_parent_modified_world_matrix() {
        const THRESHOLD: f32 = 0.0001;

        let mut world = shipyard::World::new();

        let root_entity_parent = world.add_entity((
            fabled_transform::Translation {
                value: [44.42781, 13.65856, 41.4725, 1.0],
            },
            fabled_transform::Scale {
                value: ScaleType::Uniform(2.0),
            },
            fabled_transform::Rotation {
                value: [0.0, 0.0, 0.0, 1.0],
            },
            LocalToWorld::default(),
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
            LocalToWorld::default(),
            Parent {
                value: root_entity_parent.inner(),
            },
        ));


        let entity_child = world.add_entity((
            fabled_transform::Translation {
                value: [12.2, 9.45, 11.0, 1.0],
            },
            fabled_transform::Scale {
                value: ScaleType::Uniform(2.0),
            },
            fabled_transform::Rotation {
                value: [0.3097265, 0.2101103, 0.5141426, 0.7717387],
            },
            LocalToWorld::default(),
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

        let local_world_storage = world.borrow::<shipyard::View<LocalToWorld>>().unwrap();

        let root_parent_local_world = (&local_world_storage).get(root_entity_parent).unwrap();
        let parent_local_world = (&local_world_storage).get(entity_parent).unwrap();
        let child_local_world = (&local_world_storage).get(entity_child).unwrap();

        // Unity's root parent local to world matrix result (our matrix is column-major)
        // 2.00000	0.00000	0.00000	44.42781
        // 0.00000	2.00000	0.00000	13.65856
        // 0.00000	0.00000	2.00000	41.47250
        // 0.00000	0.00000	0.00000	 1.00000

        let proven_root_parent_local_world = [
            2.00000, 0.00000, 0.00000, 0.00000, // col 0
            0.00000, 2.00000, 0.00000, 0.00000, // col 1
            0.00000, 0.00000, 2.00000, 0.00000, // col 2
            44.42781, 13.65856, 41.472_5, 1.00000, // col 3
        ];

        // Unity's parent local to world matrix result (our matrix is column-major)
        //  2.29813	-3.98048	 3.85673	113.28340
        //  5.54233	 1.67672	-1.57202	-13.65856
        // -0.03488	 4.16466	 4.31908	 38.52750
        //  0.00000	 0.00000	 0.00000	  1.00000

        let proven_parent_local_world = [
            2.29813, 5.54233, -0.03488, 0.00000, // col 0
            -3.98048, 1.67672, 4.16466, 0.00000, // col 1
            3.85673, -1.57202, 4.31908, 0.00000, // col 2
            113.283_4, -13.65856, 38.527_5, 1.00000, // col 3
        ];

        // Unity's child local to world matrix result (our matrix is column-major)
        // -5.63808	  0.08003	10.59272	146.12910
        //  7.36159	 -8.59888	3.98325	     52.51069
        //  7.61702	  8.36976	3.99100	    124.96780
        //  0.00000	  0.00000	0.00000	      1.00000

        let proven_child_local_world = [
            -5.63808, 7.36159, 7.61702, 0.00000, // col 0
            0.08003, -8.59888, 8.36976, 0.00000, // col 1
            10.59272, 3.98325, 3.99100, 0.00000, // col 2
            146.129_1, 52.51069, 124.967_8, 1.00000, // col 3
        ];

        for each in 0..16_usize {
            assert!(
                (proven_root_parent_local_world[each] - root_parent_local_world.value[each]).abs()
                    <= THRESHOLD
            );
            assert!(
                (proven_parent_local_world[each] - parent_local_world.value[each]).abs()
                    <= THRESHOLD
            );
            assert!(
                (proven_child_local_world[each] - child_local_world.value[each]).abs() <= THRESHOLD
            );
        }
    }
}
