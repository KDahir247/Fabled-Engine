use fabled_transform::{
    get_transformation_matrix, inverse_rotation_matrix, transform, Frozen, Parent, Rotation, Scale,
    ScaleType, Translation, WorldToLocal,
};

use fabled_transform::ScaleType::NonUniform;
use shipyard::*;


// The Local Matrix rotation is the inverse of the parent rotation vector if any
// only if the scale is uniform. converting to local rotation is calculated by
// i, j, k, w * -1.0, -1.0, -1.0, 1.0 = -i, -j, -k, w
#[rustfmt::skip]
pub fn calculate_world_local_system(
    frozen_storage: View<Frozen>,
    translation_storage : View<Translation>,
    rotation_storage : View<Rotation>,
    scale_storage : View<Scale>,
    parent_storage : View<Parent>,
    mut world_to_local_storage: ViewMut<WorldToLocal>,
) {
    (
        &translation_storage,
        &rotation_storage, 
        &scale_storage,
        &mut world_to_local_storage,
        !&parent_storage,
        !&frozen_storage
    )
        .fast_iter()
        .for_each(|(translation, rotation, scale, world_to_local,..)| {

            let world_matrix = get_transformation_matrix(*translation, *rotation, *scale);

            let translation = [world_matrix[12], world_matrix[13], world_matrix[14]];
            
            let rotation_matrix = [
                world_matrix[0], world_matrix[1], world_matrix[2], // col 0
                world_matrix[4], world_matrix[5], world_matrix[6], // col 1
                world_matrix[8], world_matrix[9], world_matrix[10] // col 2
            ];
            
            let inv_rotation_matrix = inverse_rotation_matrix(rotation_matrix);
            

            let local_translation = [
                inv_rotation_matrix[0] * translation[0]
                    + inv_rotation_matrix[3] * translation[1]
                    + inv_rotation_matrix[6] * translation[2],
                inv_rotation_matrix[1] * translation[0]
                    + inv_rotation_matrix[4] * translation[1]
                    + inv_rotation_matrix[7] * translation[2],
                inv_rotation_matrix[2] * translation[0]
                    + inv_rotation_matrix[5] * translation[1]
                    + inv_rotation_matrix[8] * translation[2],
            ];

            let matrix4x4 = [
                inv_rotation_matrix[0], inv_rotation_matrix[1], inv_rotation_matrix[2], 0.0, // col 0
                inv_rotation_matrix[3], inv_rotation_matrix[4], inv_rotation_matrix[5], 0.0, // col 1
                inv_rotation_matrix[6], inv_rotation_matrix[7], inv_rotation_matrix[8], 0.0, // col 2
                -local_translation[0], -local_translation[1], -local_translation[2], 1.0 // col 3
            ];

            world_to_local.value = matrix4x4;
        });

}

pub fn calculate_world_local_parent_system(
    translation_storage: View<Translation>,
    rotation_storage: View<Rotation>,
    scale_storage: View<Scale>,
    frozen: View<Frozen>,
    parent_storage: View<Parent>,
    mut world_to_local_storage: ViewMut<WorldToLocal>,
) {
    (
        &translation_storage,
        &rotation_storage,
        &scale_storage,
        &parent_storage,
        !&frozen,
    )
        .fast_iter()
        .with_id()
        .for_each(|(entity_id, (translation, rotation, scale, parent, ..))| {
            let identity_matrix = WorldToLocal::default();

            let parent_entity_id =
                EntityId::from_inner(parent.value).unwrap_or_else(EntityId::dead);


            let parent_translation = (&translation_storage).fast_get(parent_entity_id).unwrap();

            let parent_translation = parent_translation.value;

            let parent_scale = (&scale_storage).fast_get(parent_entity_id).unwrap();

            let parent_scale = match parent_scale.value {
                ScaleType::Uniform(uniform) => [uniform; 3],
                NonUniform(non_uniform) => non_uniform,
            };


            let child_translation = translation.value;
            let child_rotation = rotation.value;
            let child_scale = match scale.value {
                ScaleType::Uniform(uniform) => [uniform; 3],
                ScaleType::NonUniform(non_uniform) => non_uniform,
            };

            let child_local_scale = [
                child_scale[0].recip() * parent_scale[0].recip(),
                child_scale[1].recip() * parent_scale[1].recip(),
                child_scale[2].recip() * parent_scale[2].recip(),
            ];


            let world_matrix = get_transformation_matrix(*translation, *rotation, *scale);

            // todo to get the local rotation we will negate the i j k in the quaternion and
            // keep w the same. after that we will scale it by the reciprocal of
            // the scale.
            let rotation_matrix = [
                world_matrix[0],
                world_matrix[1],
                world_matrix[2], // col 0
                world_matrix[4],
                world_matrix[5],
                world_matrix[6], // col 1
                world_matrix[8],
                world_matrix[9],
                world_matrix[10], // col 2
            ];

            let inverse_rotation_matrix = inverse_rotation_matrix(rotation_matrix);
            let scaled_inverse_rotation_matrix = [
                inverse_rotation_matrix[0] * parent_scale[0].recip(),
                inverse_rotation_matrix[1] * parent_scale[0].recip(),
                inverse_rotation_matrix[2] * parent_scale[0].recip(),
                inverse_rotation_matrix[3] * parent_scale[1].recip(),
                inverse_rotation_matrix[4] * parent_scale[1].recip(),
                inverse_rotation_matrix[5] * parent_scale[1].recip(),
                inverse_rotation_matrix[6] * parent_scale[2].recip(),
                inverse_rotation_matrix[7] * parent_scale[2].recip(),
                inverse_rotation_matrix[8] * parent_scale[2].recip(),
            ];


            // parent translation mul the child local scale.

            println!("{:?}", scaled_inverse_rotation_matrix);

            println!(
                "{:?} {:?} {:?}",
                child_local_scale[0],
                parent_translation[0] * child_local_scale[0],
                child_translation[0] * child_scale[0].recip()
            );


            println!(
                "{:?} {:?} {:?}",
                child_local_scale[1],
                parent_translation[1] * child_local_scale[1],
                child_translation[1] * child_scale[1].recip()
            );

            println!(
                "{:?} {:?} {:?}",
                child_local_scale[2],
                parent_translation[2] * child_local_scale[2],
                child_translation[2] * child_scale[2].recip()
            );

            // todo have to add quaternion which affect the translation.
            let a = [
                -parent_translation[0] * child_local_scale[0]
                    - child_translation[0] * child_scale[0].recip(),
                -parent_translation[1] * child_local_scale[1]
                    - child_translation[1] * child_scale[1].recip(),
                -parent_translation[2] * child_local_scale[2]
                    - child_translation[2] * child_scale[2].recip(),
            ];


            println!("{:?}", a);
            // todo calculate the local translation
        });
}

#[cfg(test)]
mod world_local_test {

    use crate::system::transform::world_local_system::{
        calculate_world_local_parent_system, calculate_world_local_system,
    };
    use fabled_transform::{ScaleType, WorldToLocal};
    use shipyard::Get;

    #[test]
    fn retrieve_modified_local_matrix() {
        const THRESHOLD: f32 = 0.0001;

        let mut world = shipyard::World::new();

        let entity = world.add_entity((
            fabled_transform::Translation {
                value: [44.42781, 13.65856, 41.4725, 1.0],
            },
            fabled_transform::Scale {
                value: ScaleType::NonUniform([1.0, 2.1, 11.1]),
            },
            fabled_transform::Rotation {
                value: [0.2439353, -0.0781964, 0.6303434, 0.7328356],
            },
            fabled_transform::WorldToLocal::default(),
        ));

        shipyard::Workload::builder("run_test")
            .with_system(&calculate_world_local_system)
            .add_to_world(&world)
            .unwrap();

        world.run_workload("run_test").unwrap();

        // Unity's world to local result (our matrix is column-major)
        //  0.19310	  0.88573	0.42214	 -38.18402
        // -0.45811	  0.04111	0.12331	  14.67734
        //  0.01738	 -0.04109	0.07827	  -3.45682
        //  0.00000	  0.00000	0.00000	   1.00000

        let world_local_storage = world.borrow::<shipyard::View<WorldToLocal>>().unwrap();

        let world_local = (&world_local_storage).get(entity).unwrap();

        let proven_result = [
            0.19310, -0.45811, 0.01738, 0.00000, // col 0
            0.88573, 0.04111, -0.04109, 0.00000, // col 1
            0.42214, 0.12331, 0.07827, 0.00000, // col 2
            -38.18402, 14.67734, -3.45682, 1.00000, // col 3
        ];

        for each in 0..16_usize {
            assert!((world_local.value[each] - proven_result[each]).abs() <= THRESHOLD);
        }
    }

    #[test]
    fn retrieve_parent_modified_local_matrix() {
        let mut world = shipyard::World::new();

        // 0.3097265, 0.2101103, 0.5141426, 0.7717387
        let entity_parent = world.add_entity((
            fabled_transform::Translation {
                value: [30.0, 40.0, 50.0, 1.0],
            },
            fabled_transform::Scale {
                value: ScaleType::NonUniform([12.123, 4.1, 1.11]),
            },
            fabled_transform::Rotation {
                value: [0.0, 0.0, 0.0, 1.],
            },
            fabled_transform::WorldToLocal::default(),
        ));


        let entity_child = world.add_entity((
            fabled_transform::Translation {
                value: [10.0, 20.0, 30.0, 1.0],
            },
            fabled_transform::Scale {
                value: ScaleType::NonUniform([1.55, 2.21, 4.11]),
            },
            fabled_transform::Rotation {
                value: [0.0, 0.0, 0.0, 1.],
            },
            fabled_transform::Parent {
                value: entity_parent.inner(),
            },
            fabled_transform::WorldToLocal::default(),
        ));
        shipyard::Workload::builder("run_test")
            .with_system(&calculate_world_local_system)
            .with_system(&calculate_world_local_parent_system)
            .add_to_world(&world)
            .unwrap();

        world.run_workload("run_test").unwrap();
    }
}
