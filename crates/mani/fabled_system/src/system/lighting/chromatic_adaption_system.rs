use fabled_render::light::{ChromaticDesignation, LightAppearance};
use shipyard::*;

#[rustfmt::skip]
pub fn calculate_color_adaption(
    mut light_appearance: ViewMut<LightAppearance>,
    chromaticity_target: View<ChromaticDesignation>,
) {
    (&mut light_appearance, &chromaticity_target)
        .iter()
        .with_id()
        .filter(|(entity_id,_)| chromaticity_target.is_modified(*entity_id))
        .for_each(|(_, (mut light_appearance,  chromaticity_dest))| {
            let [src_tristimulus_x, src_tristimulus_y, src_tristimulus_z] = light_appearance.chromaticity_coord;

            let [dst_tristimulus_x, dst_tristimulus_y, dst_tristimulus_z] = chromaticity_dest.0;

            let adaption_transform_matrix = fabled_render::light::VON_KRIES;

            let inverse_chromatic_adaption_matrix = fabled_render::light::VON_KRIES_INVERSE;
            
            let transposed_chromatic_adaption_matrix = fabled_render::light::VON_KRIES_TRANSPOSE;
            
            let src_white_scalar = 1.0 / src_tristimulus_y;

            let dst_white_scalar = 1.0 / dst_tristimulus_y;

            let src_white_point = [
                src_tristimulus_x * src_white_scalar,
                1.0,
                src_tristimulus_z * src_white_scalar,
            ];

            let dst_white_point = [
                dst_tristimulus_x * dst_white_scalar,
                1.0,
                dst_tristimulus_z * dst_white_scalar,
            ];

            let src_cone_lms = [
                adaption_transform_matrix[0] * src_white_point[0]
                    + adaption_transform_matrix[1] * src_white_point[1]
                    + adaption_transform_matrix[2] * src_white_point[2],
                adaption_transform_matrix[3] * src_white_point[0]
                    + adaption_transform_matrix[4] * src_white_point[1]
                    + adaption_transform_matrix[5] * src_white_point[2],
                adaption_transform_matrix[6] * src_white_point[0]
                    + adaption_transform_matrix[7] * src_white_point[1]
                    + adaption_transform_matrix[8] * src_white_point[2],
            ];

            let dst_cone_lms = [
                adaption_transform_matrix[0] * dst_white_point[0]
                    + adaption_transform_matrix[1] * dst_white_point[1]
                    + adaption_transform_matrix[2] * dst_white_point[2],
                adaption_transform_matrix[3] * dst_white_point[0]
                    + adaption_transform_matrix[4] * dst_white_point[1]
                    + adaption_transform_matrix[5] * dst_white_point[2],
                adaption_transform_matrix[6] * dst_white_point[0]
                    + adaption_transform_matrix[7] * dst_white_point[1]
                    + adaption_transform_matrix[8] * dst_white_point[2],
            ];

            let chromatic_adaptation_difference_matrix = 
                [
                    dst_cone_lms[0] / src_cone_lms[0], 0.0, 0.0, 
                    0.0, dst_cone_lms[1] / src_cone_lms[1], 0.0, 
                    0.0, 0.0, dst_cone_lms[2] / src_cone_lms[2], 
                ];
            
            let intermediate_calculation = 
                [
                    inverse_chromatic_adaption_matrix[0] * chromatic_adaptation_difference_matrix[0], inverse_chromatic_adaption_matrix[1] * chromatic_adaptation_difference_matrix[4],  inverse_chromatic_adaption_matrix[2] * chromatic_adaptation_difference_matrix[8], 
                    inverse_chromatic_adaption_matrix[3] * chromatic_adaptation_difference_matrix[0], inverse_chromatic_adaption_matrix[4] * chromatic_adaptation_difference_matrix[4],  inverse_chromatic_adaption_matrix[5] * chromatic_adaptation_difference_matrix[8], 
                    inverse_chromatic_adaption_matrix[6] * chromatic_adaptation_difference_matrix[0], inverse_chromatic_adaption_matrix[7] * chromatic_adaptation_difference_matrix[4], inverse_chromatic_adaption_matrix[8] * chromatic_adaptation_difference_matrix[8]
                ];
            
            
            let chromatic_adaptation_matrix = 
                [
                    intermediate_calculation[0] * transposed_chromatic_adaption_matrix[0] + intermediate_calculation[1] * transposed_chromatic_adaption_matrix[1] + intermediate_calculation[2] * transposed_chromatic_adaption_matrix[2], 
                    intermediate_calculation[0] * transposed_chromatic_adaption_matrix[3] + intermediate_calculation[1] * transposed_chromatic_adaption_matrix[4] + intermediate_calculation[2] * transposed_chromatic_adaption_matrix[5],
                    intermediate_calculation[0] * transposed_chromatic_adaption_matrix[6] + intermediate_calculation[1] * transposed_chromatic_adaption_matrix[7] + intermediate_calculation[2] * transposed_chromatic_adaption_matrix[8],

                    intermediate_calculation[3] * transposed_chromatic_adaption_matrix[0] + intermediate_calculation[4] * transposed_chromatic_adaption_matrix[1] + intermediate_calculation[5] * transposed_chromatic_adaption_matrix[2],
                    intermediate_calculation[3] * transposed_chromatic_adaption_matrix[3] + intermediate_calculation[4] * transposed_chromatic_adaption_matrix[4] + intermediate_calculation[5] * transposed_chromatic_adaption_matrix[5],
                    intermediate_calculation[3] * transposed_chromatic_adaption_matrix[6] + intermediate_calculation[4] * transposed_chromatic_adaption_matrix[7] + intermediate_calculation[5] * transposed_chromatic_adaption_matrix[8],
                    
                    intermediate_calculation[6] * transposed_chromatic_adaption_matrix[0] + intermediate_calculation[7] * transposed_chromatic_adaption_matrix[1] + intermediate_calculation[8] * transposed_chromatic_adaption_matrix[2],
                    intermediate_calculation[6] * transposed_chromatic_adaption_matrix[3] + intermediate_calculation[7] * transposed_chromatic_adaption_matrix[4] + intermediate_calculation[8] * transposed_chromatic_adaption_matrix[5],
                    intermediate_calculation[6] * transposed_chromatic_adaption_matrix[6] + intermediate_calculation[7] * transposed_chromatic_adaption_matrix[7] + intermediate_calculation[8] * transposed_chromatic_adaption_matrix[8],
                ];

            println!("{:?}", chromatic_adaptation_matrix);
            let adapted_tristimulus = [
                chromatic_adaptation_matrix[0] * src_tristimulus_x + chromatic_adaptation_matrix[1] * src_tristimulus_y + chromatic_adaptation_matrix[2] * src_tristimulus_z,
                chromatic_adaptation_matrix[3] * src_tristimulus_x + chromatic_adaptation_matrix[4] * src_tristimulus_y + chromatic_adaptation_matrix[5] * src_tristimulus_z,
                chromatic_adaptation_matrix[6] * src_tristimulus_x + chromatic_adaptation_matrix[7] * src_tristimulus_y + chromatic_adaptation_matrix[8] * src_tristimulus_z
            ];

            light_appearance.chromaticity_coord = adapted_tristimulus;

        });


}

#[cfg(test)]
mod adaption_test {
    use crate::system::lighting::calculate_color_adaption;
    use shipyard::Get;

    #[test]
    fn chromatic_adaption_test() {
        let mut world = shipyard::World::new();

        println!(
            "{:?}",
            fabled_render::light::LightAppearance::new([0.448, 0.408], [1.0, 1.0, 1.0])
        );

        let entity = world.add_entity((
            fabled_render::light::LightAppearance::new([0.448, 0.408], [1.0, 1.0, 1.0]),
            fabled_render::light::ChromaticDesignation::default(),
        ));

        shipyard::Workload::builder("run_test")
            .with_system(&calculate_color_adaption)
            .add_to_world(&world)
            .unwrap();

        {
            let mut chromatic_designation_storage = world
                .borrow::<shipyard::ViewMut<fabled_render::light::ChromaticDesignation>>()
                .unwrap();

            let mut chromatic_designation =
                (&mut chromatic_designation_storage).get(entity).unwrap();

            chromatic_designation.0 = [0.384, 0.352, 0.264];
        }

        // should only happen once since it was modified once and not 10 times.
        for _ in 0..10 {
            world.run_workload("run_test").unwrap();
        }

        let light_appearance_storage = world
            .borrow::<shipyard::View<fabled_render::light::ChromaticDesignation>>()
            .unwrap();

        let light_appearance = (&light_appearance_storage).get(entity).unwrap();

        println!("{:?}", light_appearance);
    }
}
