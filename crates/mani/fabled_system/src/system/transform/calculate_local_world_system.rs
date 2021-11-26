use fabled_transform::{get_rotation_matrix, LocalToWorld, Rotation, Scale, Translation};
use shipyard::*;

#[rustfmt::skip]
pub fn local_world_system(
    mut position: shipyard::ViewMut<Translation>,
    mut rotation: shipyard::ViewMut<Rotation>,
    mut scale: shipyard::ViewMut<Scale>,
    mut local_to_world: shipyard::ViewMut<LocalToWorld>,
) {
    let mut modified_inserted_entities = Vec::new();
    
    //component tracking.
    // todo delegate the tracking responsibility to some other system.
    position.track_insertion().track_modification();
    rotation.track_insertion().track_modification();
    scale.track_insertion().track_modification();
    
    
        let modified_inserted_position = position.inserted_or_modified();
        
        (modified_inserted_position, &mut local_to_world)
            .iter()
            .with_id()
            .for_each(|(entity_id,(modified_position, mut local_world))| {
                modified_inserted_entities.push(entity_id);
                
                // column array matrix
                // [0   4    8   12]
                // [1   5    9   13]
                // [2   6   10   14]
                // [3   7   11   15]
                let matrix4x4 = local_world.value;

                let position = modified_position.value;

                let scalar = position[3];

                let translation = [
                    position[0] * scalar,
                    position[1] * scalar,
                    position[2] * scalar,
                ];

                let modified_matrix =
                    [
                        matrix4x4[0], matrix4x4[1], matrix4x4[ 2], 0.0, // col 0
                        matrix4x4[4], matrix4x4[5], matrix4x4[ 6], 0.0, // col 1
                        matrix4x4[8], matrix4x4[9], matrix4x4[10], 0.0, // col 2
                        translation[0], translation[1], translation[2], 1.0 // col 3
                    ];
                
                local_world.value = modified_matrix;
            });
        

    
        let modified_rotation = rotation.inserted_or_modified();

        (modified_rotation, &mut local_to_world)
            .iter()
            .with_id()
            .for_each(|(entity_id,(modified_rotation, mut local_world))|{
                
                modified_inserted_entities.push(entity_id);

                let matrix4x4 = local_world.value;
                let translation = [matrix4x4[12], matrix4x4[13], matrix4x4[14], 1.0];
                
                let rotation_matrix = get_rotation_matrix(*modified_rotation);

                let modified_matrix =
                    [
                        rotation_matrix[0], rotation_matrix[1], rotation_matrix[2], 0.0,
                        rotation_matrix[3], rotation_matrix[4], rotation_matrix[5], 0.0,
                        rotation_matrix[6], rotation_matrix[7], rotation_matrix[8], 0.0,
                        translation[0], translation[1], translation[2], translation[3]
                    ];
                
                local_world.value = modified_matrix;
            });
    
        let modified_scale = scale.inserted_or_modified();

        (modified_scale, &mut local_to_world)
            .iter()
            .with_id()
            .for_each(|(entity_id,(modified_scale, mut local_world))|{
                
                modified_inserted_entities.push(entity_id);
                
                let matrix4x4 = local_world.value;
                
                let scale = match modified_scale.scale{
                    fabled_transform::ScaleType::Uniform(uniform) => [uniform, uniform, uniform],
                    fabled_transform::ScaleType::NonUniform(non_uniform) => non_uniform,
                };
                
                let translation = 
                    [
                        matrix4x4[12], matrix4x4[13], matrix4x4[14]
                    ];
                
                let modified_matrix = 
                    [
                        matrix4x4[0] * scale[0], matrix4x4[1] * scale[0], matrix4x4[ 2] * scale[0], 0.0, // col 0
                        matrix4x4[4] * scale[1], matrix4x4[5] * scale[1], matrix4x4[ 6] * scale[1], 0.0, // col 1
                        matrix4x4[8] * scale[2], matrix4x4[9] * scale[2], matrix4x4[10] * scale[2], 0.0, // col 2
                        translation[0], translation[1], translation[2], 1.0 // col 3
                    ];

                local_world.value = modified_matrix;
                
            });

    for entity_id in  &modified_inserted_entities{
        position.clear_inserted_and_modified(*entity_id);
        rotation.clear_inserted_and_modified(*entity_id);
        scale.clear_inserted_and_modified(*entity_id);
    }

    modified_inserted_entities.clear();
}

#[cfg(test)]
mod local_world_test {
    use crate::system::transform::calculate_local_world_system::local_world_system;
    use fabled_transform::{Rotation, Translation};
    use shipyard::Get;

    #[test]
    fn modfied_transforms() {
        let mut world = shipyard::World::new();

        let entity = world.add_entity((
            fabled_transform::Orientation::default(),
            fabled_transform::Translation::default(),
            fabled_transform::Scale::default(),
            fabled_transform::Rotation {
                value: [0.49759552, -0.10885537, 0.85859334, -0.058024056],
            },
            fabled_transform::LocalToWorld::default(),
        ));

        shipyard::Workload::builder("run_test")
            .with_system(&local_world_system)
            .add_to_world(&world)
            .unwrap();

        let mut view_mut_pos = world.borrow::<shipyard::ViewMut<Rotation>>().unwrap();

        *(&mut view_mut_pos).get(entity).unwrap() = Rotation {
            value: [0.0, 0.8378122, 0.0, -0.5459588],
        };

        println!("{:?}", *(&mut view_mut_pos).get(entity).unwrap());
        drop(view_mut_pos);

        world.run_workload("run_test").unwrap();
    }
}
