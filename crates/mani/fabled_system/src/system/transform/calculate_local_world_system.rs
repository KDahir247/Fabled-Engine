use fabled_transform::{
    get_rotation_matrix, Frozen, LocalToWorld, Parent, Rotation, Scale, ScaleType, Translation,
};
use shipyard::*;


pub fn local_world_system(
    translation: View<Translation>,
    rotation: View<Rotation>,
    scale: View<Scale>,
    frozen: View<Frozen>,
    parent: View<Parent>,
    mut local_to_world: ViewMut<LocalToWorld>,
) {
    (
        &translation,
        &rotation,
        &scale,
        &mut local_to_world,
        !&parent,
        !&frozen,
    )
        .fast_iter()
        .for_each(
            |(translation, rotation, scale, mut local_to_world_matrix, ..)| {
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
                    1.0, // translation[3] / translation[3] should be 1.0 after passing assertion
                ];

                let rotation_matrix = get_rotation_matrix(*rotation);

                let scale = match scale.value {
                    ScaleType::Uniform(uniform) => [uniform; 3],
                    ScaleType::NonUniform(non_uniform) => non_uniform,
                };

                let matrix4x4 = [
                    rotation_matrix[0] * scale[0],
                    rotation_matrix[1] * scale[0],
                    rotation_matrix[2] * scale[0],
                    0.0, // col 0
                    rotation_matrix[3] * scale[1],
                    rotation_matrix[4] * scale[1],
                    rotation_matrix[5] * scale[1],
                    0.0, // col 1
                    rotation_matrix[6] * scale[2],
                    rotation_matrix[7] * scale[2],
                    rotation_matrix[8] * scale[2],
                    0.0, // col 2
                    norm_translation_vec[0],
                    norm_translation_vec[1],
                    norm_translation_vec[2],
                    1.0,
                ];

                local_to_world_matrix.value = matrix4x4;
            },
        );


    (
        &translation,
        &rotation,
        &scale,
        &parent,
        &mut local_to_world,
        !&frozen,
    )
        .iter()
        .for_each(|_| {
            // todo entity with parent.
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


        if let Ok(parent_component) = (&parent_storage).get(entity) {
            // test the result with the entity having a parent in mind.
        } else {
            let local_world_matrix = (&local_to_world_storage).get(entity).unwrap();
            println!("{:?}", local_world_matrix);
        }
    }
}
