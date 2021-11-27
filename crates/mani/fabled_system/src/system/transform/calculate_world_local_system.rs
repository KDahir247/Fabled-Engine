use fabled_transform::{
    get_rotation_matrix, Parent, Rotation, Scale, ScaleType, Translation, WorldToLocal,
};

use shipyard::*;

pub fn world_local_system(
    translation: shipyard::View<Translation>,
    rotation: View<Rotation>,
    scale: View<Scale>,
    parent: View<Parent>,
    mut world_to_local: ViewMut<WorldToLocal>,
) {
    (
        &translation,
        &rotation,
        &scale,
        !&parent,
        &mut world_to_local,
    )
        .fast_iter()
        .for_each(|(translation, rotation, scale, _, world_to_local_matrix)| {
            // column array matrix
            // [0   4    8   12]
            // [1   5    9   13]
            // [2   6   10   14]
            // [3   7   11   15]

            let translation_vector = translation.value;

            assert!(translation_vector[3].ne(&0.0));

            let mut matrix4x4 = WorldToLocal::default().value;

            let inv_scalar = 1.0 / translation_vector[3];

            let norm_translation_vec = [
                translation_vector[0] * inv_scalar,
                translation_vector[1] * inv_scalar,
                translation_vector[2] * inv_scalar,
                1.0,
            ];

            let rotation_matrix = get_rotation_matrix(*rotation);

            let inv_scale = match scale.value {
                ScaleType::Uniform(uniform) => {
                    let inv_scale = 1.0 / uniform;
                    [inv_scale, inv_scale, inv_scale]
                }
                ScaleType::NonUniform(non_uniform) => [
                    1.0 / non_uniform[0],
                    1.0 / non_uniform[1],
                    1.0 / non_uniform[2],
                ],
            };
        });

    (
        &translation,
        &rotation,
        &scale,
        &parent,
        &mut world_to_local,
    )
        .fast_iter()
        .for_each(|_| {
            // todo entity with parent.
        });
}

#[cfg(test)]
mod world_local_test {
    use fabled_transform::{vec_mul_qut, Rotation, Translation};

    #[test]
    fn a() {
        let rotation = Rotation {
            value: [0.3097265, 0.2101103, 0.5141426, 0.7717387],
        };

        let a = vec_mul_qut(
            rotation,
            Translation {
                value: [-10.0, -13.0, -10.0, 1.0],
            },
        );

        println!("{:?}", a);
    }
}
