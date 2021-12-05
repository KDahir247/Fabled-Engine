use fabled_transform::{
    get_rotation_matrix, Frozen, Parent, Rotation, Scale, ScaleType,
    Translation, WorldToLocal,
};

use shipyard::*;

#[rustfmt::skip]
pub fn world_local_system(
    translation_storage: shipyard::View<Translation>,
    rotation_storage: View<Rotation>,
    scale_storage: View<Scale>,
    frozen_storage: View<Frozen>,
    parent_storage: View<Parent>,
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
        .for_each(|(translation, rotation, scale, world_to_local_matrix,..)| {
            // column array matrix
            // [0   4    8   12]
            // [1   5    9   13]
            // [2   6   10   14]
            // [3   7   11   15]

            let translation_vector = translation.value;

            assert!(translation_vector[3].ne(&0.0));

            let inv_scalar = 1.0 / translation_vector[3];

            let norm_translation_vec = [
                translation_vector[0] * inv_scalar,
                translation_vector[1] * inv_scalar,
                translation_vector[2] * inv_scalar,
                1.0,
            ];

            let mut rotation_matrix = get_rotation_matrix(*rotation);

            let scale = match scale.value {
                ScaleType::Uniform(uniform) => [uniform; 3],
                ScaleType::NonUniform(non_uniform) => {
                    [non_uniform[0], non_uniform[1], non_uniform[2]]
                }
            };

            rotation_matrix = [
                rotation_matrix[0] * scale[0], rotation_matrix[1] * scale[0], rotation_matrix[2] * scale[0], // col 0
                rotation_matrix[3] * scale[1], rotation_matrix[4] * scale[1], rotation_matrix[5] * scale[1], // col 1
                rotation_matrix[6] * scale[2], rotation_matrix[7] * scale[2], rotation_matrix[8] * scale[2], // col 2
            ];

            let cross_yz = [
                rotation_matrix[4] * rotation_matrix[8] - rotation_matrix[5] * rotation_matrix[7],
                rotation_matrix[5] * rotation_matrix[6] - rotation_matrix[3] * rotation_matrix[8],
                rotation_matrix[3] * rotation_matrix[7] - rotation_matrix[4] * rotation_matrix[6],
            ];
            let cross_zx = [
                rotation_matrix[7] * rotation_matrix[2] - rotation_matrix[8] * rotation_matrix[1],
                rotation_matrix[8] * rotation_matrix[0] - rotation_matrix[6] * rotation_matrix[2],
                rotation_matrix[6] * rotation_matrix[1] - rotation_matrix[7] * rotation_matrix[0],
            ];

            let cross_xy = [
                rotation_matrix[1] * rotation_matrix[5] - rotation_matrix[2] * rotation_matrix[4],
                rotation_matrix[2] * rotation_matrix[3] - rotation_matrix[0] * rotation_matrix[5],
                rotation_matrix[0] * rotation_matrix[4] - rotation_matrix[1] * rotation_matrix[3],
            ];

            let determinant = rotation_matrix[6] * cross_xy[0]
                + rotation_matrix[7] * cross_xy[1]
                + rotation_matrix[8] * cross_xy[2];


            assert!(determinant.ne(&0.0));

            let inv_determinant = 1.0 / determinant;

            let inv_rotation_matrix = [
                cross_yz[0] * inv_determinant, cross_zx[0] * inv_determinant, cross_xy[0] * inv_determinant, // col 0
                cross_yz[1] * inv_determinant, cross_zx[1] * inv_determinant, cross_xy[1] * inv_determinant, // col 1
                cross_yz[2] * inv_determinant, cross_zx[2] * inv_determinant, cross_xy[2] * inv_determinant, // col 2
            ];

            let local_translation = [
                inv_rotation_matrix[0] * norm_translation_vec[0]
                    + inv_rotation_matrix[3] * norm_translation_vec[1]
                    + inv_rotation_matrix[6] * norm_translation_vec[2],
                inv_rotation_matrix[1] * norm_translation_vec[0]
                    + inv_rotation_matrix[4] * norm_translation_vec[1]
                    + inv_rotation_matrix[7] * norm_translation_vec[2],
                inv_rotation_matrix[2] * norm_translation_vec[0]
                    + inv_rotation_matrix[5] * norm_translation_vec[1]
                    + inv_rotation_matrix[8] * norm_translation_vec[2],
            ];

            let matrix4x4 = [
                inv_rotation_matrix[0], inv_rotation_matrix[1], inv_rotation_matrix[2], 0.0, // col 0
                inv_rotation_matrix[3], inv_rotation_matrix[4], inv_rotation_matrix[5], 0.0, // col 1
                inv_rotation_matrix[6], inv_rotation_matrix[7], inv_rotation_matrix[8], 0.0, // col 2
                -local_translation[0], -local_translation[1], -local_translation[2], 1.0 // col 3
            ];

            world_to_local_matrix.value = matrix4x4;
        });


    (
        &translation_storage,
        &rotation_storage,
        &scale_storage,
        &parent_storage,
        &mut world_to_local_storage,
        !&frozen_storage,
    )
        .fast_iter()
        .for_each(|(translation, rotation, scale, parent,mut world_to_local_matrix, _)| {
            let parent_entity_index = parent.value;
            
            let mut parent_entity_id = shipyard::EntityId::from_inner(parent_entity_index);
            
            while parent_entity_id.is_some(){
                let valid_parent_entity = parent_entity_id.unwrap();


                let inner_parent_component = (&parent_storage).fast_get(valid_parent_entity).unwrap_or(&Parent{ value: 0 });

                parent_entity_id = shipyard::EntityId::from_inner(inner_parent_component.value);
            }
            
            
            
        });
}

#[cfg(test)]
mod world_local_test {
    use crate::system::transform::calculate_world_local_system::world_local_system;
    use fabled_transform::{Parent, ScaleType, WorldToLocal};
    use shipyard::{Get, ViewMut};

    #[test]
    fn retrieve_modified_local_matrix() {
        let mut world = shipyard::World::new();

        let entity = world.add_entity((
            fabled_transform::Translation {
                value: [5.0, 13.0, 2.0, 1.0],
            },
            fabled_transform::Scale {
                value: ScaleType::Uniform(1.0),
            },
            fabled_transform::Rotation {
                value: [0.3097265, 0.2101103, 0.5141426, 0.7717387],
            },
            fabled_transform::WorldToLocal::default(),
        ));

        shipyard::Workload::builder("run_test")
            .with_system(&world_local_system)
            .add_to_world(&world)
            .unwrap();

        world.run_workload("run_test").unwrap();

        let (parent_storage, world_to_local_storage) = world
            .borrow::<(ViewMut<Parent>, ViewMut<WorldToLocal>)>()
            .unwrap();

        if let Ok(parent_component) = (&parent_storage).get(entity) {
        } else {
            let world_local_matrix =
                (&world_to_local_storage).get(entity).unwrap();
            println!("{:?}", world_local_matrix);
        }
    }
}
