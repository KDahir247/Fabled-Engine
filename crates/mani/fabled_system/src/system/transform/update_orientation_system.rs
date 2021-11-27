use fabled_transform::{Frozen, Orientation, Rotation};

use rayon::iter::ParallelIterator;

use shipyard::IntoIter;

pub fn re_orientate_entity(
    mut orientation: shipyard::ViewMut<Orientation>,
    rotation: shipyard::View<Rotation>,
    frozen: shipyard::View<Frozen>,
) {
    (&mut orientation, &rotation, !&frozen).par_iter().for_each(
        |(mut orientation, quaternion, _)| {
            let (i, j, k, w) = (
                quaternion.value[0],
                quaternion.value[1],
                quaternion.value[2],
                quaternion.value[3],
            );

            let quat_dot = i * i + j * j + k * k;

            let k2 = k * 2.0;
            let w2 = w * 2.0;

            let result_upper = [k2 * i, k2 * j, k2 * k];

            let sq_sclr_min_sq_forw = w * w - quat_dot;

            let result_middle = [0.0, 0.0, sq_sclr_min_sq_forw];

            let result_bottom = [w2 * j, w2 * -i, 0.0];

            let forward = [
                result_upper[0] + result_middle[0] + result_bottom[0],
                result_upper[1] + result_middle[1] + result_bottom[1],
                result_upper[2] + result_middle[2] + result_bottom[2],
            ];

            orientation.forward = forward;

            let result_upper = [2.0 * i * i, 2.0 * i * j, 2.0 * i * k];

            let result_middle = [sq_sclr_min_sq_forw, 0.0, 0.0];

            let result_bottom = [0.0, w2 * k, w2 * -j];

            let right = [
                result_upper[0] + result_middle[0] + result_bottom[0],
                result_upper[1] + result_middle[1] + result_bottom[1],
                result_upper[2] + result_middle[2] + result_bottom[2],
            ];

            orientation.right = right;
        },
    );
}


#[cfg(test)]
mod mutate_orientation_test {
    use crate::system::transform::update_orientation_system::re_orientate_entity;
    use shipyard::Get;

    #[test]
    fn orientation_update() {
        let mut world = shipyard::World::new();

        let entity = world.add_entity((
            fabled_transform::Orientation::default(),
            fabled_transform::Rotation {
                value: [0.49759552, -0.10885537, 0.85859334, -0.058024056],
            },
        ));

        shipyard::Workload::builder("run_test")
            .with_system(&re_orientate_entity)
            .add_to_world(&world)
            .unwrap();

        world.run_workload("run_test").unwrap();

        let mut orientation = world
            .borrow::<shipyard::ViewMut<fabled_transform::Orientation>>()
            .unwrap();
        let modified_orientation = *(&mut orientation).get(entity).unwrap();

        // proven right from game engines  [-0.49806398, -0.20797002, 0.8418319]
        // proven forward from game engines [0.8670969, -0.12917997, 0.48109853]

        let proven_right = [-0.49806398, -0.20797002, 0.8418319];
        let proven_forward = [0.8670969, -0.12917997, 0.48109853];

        assert!(modified_orientation.right[0].eq(&proven_right[0]));
        assert!(modified_orientation.right[1].eq(&proven_right[1]));
        assert!(modified_orientation.right[2].eq(&proven_right[2]));

        assert!(modified_orientation.forward[0].eq(&proven_forward[0]));
        assert!(modified_orientation.forward[1].eq(&proven_forward[1]));
        assert!(modified_orientation.forward[2].eq(&proven_forward[2]));
    }

    #[test]
    pub fn static_entity() {
        let mut world = shipyard::World::new();

        let entity = world.add_entity((
            fabled_transform::Orientation::default(),
            fabled_transform::Rotation {
                value: [0.49759552, -0.10885537, 0.85859334, -0.058024056],
            },
            fabled_transform::Frozen::default(),
        ));


        shipyard::Workload::builder("run_test")
            .with_system(&re_orientate_entity)
            .add_to_world(&world)
            .unwrap();


        world.run_workload("run_test").unwrap();

        let mut orientation = world
            .borrow::<shipyard::ViewMut<fabled_transform::Orientation>>()
            .unwrap();
        let modified_orientation = *(&mut orientation).get(entity).unwrap();


        let default_orientation = fabled_transform::Orientation::default();

        assert!(modified_orientation.right[0].eq(&default_orientation.right[0]));
        assert!(modified_orientation.right[1].eq(&default_orientation.right[1]));
        assert!(modified_orientation.right[2].eq(&default_orientation.right[2]));

        assert!(modified_orientation.forward[0].eq(&default_orientation.forward[0]));
        assert!(modified_orientation.forward[1].eq(&default_orientation.forward[1]));
        assert!(modified_orientation.forward[2].eq(&default_orientation.forward[2]));
    }
}
