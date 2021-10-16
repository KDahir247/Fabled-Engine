mod ambisonic_output;
mod spatial_source;
mod standard_output;


pub use ambisonic_output::*;
pub use spatial_source::*;
pub use standard_output::*;

#[cfg(test)]
mod data_test {
    use crate::{AmbisonicOutput, SpatialAmbisonicSource, StandardOutput};

    #[test]
    fn data_size() {
        let audio_output_u16_size = std::mem::size_of::<StandardOutput<u16>>();
        println!("{}", audio_output_u16_size);

        let audio_output_i16_size = std::mem::size_of::<StandardOutput<i16>>();
        println!("{}", audio_output_i16_size);

        let audio_output_f32_size = std::mem::size_of::<StandardOutput<f32>>();
        println!("{}", audio_output_f32_size);

        // -----------------------------------------------------------------

        let audio_spatial_output_size = std::mem::size_of::<AmbisonicOutput>();
        println!("{}", audio_spatial_output_size);


        let spatial_source_size = std::mem::size_of::<SpatialAmbisonicSource>();
        println!("{}", spatial_source_size);
    }


    #[test]
    fn data_alignment() {
        let audio_output_u16_alignment = std::mem::align_of::<StandardOutput<u16>>();
        assert_eq!(
            audio_output_u16_alignment & (audio_output_u16_alignment - 1),
            0
        );

        let audio_output_i16_alignment = std::mem::align_of::<StandardOutput<i16>>();
        assert_eq!(
            audio_output_i16_alignment & (audio_output_i16_alignment - 1),
            0
        );

        let audio_output_f32_alignment = std::mem::align_of::<StandardOutput<f32>>();
        assert_eq!(
            audio_output_f32_alignment & (audio_output_f32_alignment - 1),
            0
        );

        // -----------------------------------------------------------------

        let audio_spatial_output_alignment = std::mem::align_of::<AmbisonicOutput>();
        assert_eq!(
            audio_spatial_output_alignment & (audio_spatial_output_alignment - 1),
            0
        );

        // -----------------------------------------------------------------

        let spatial_source_alignment = std::mem::align_of::<SpatialAmbisonicSource>();
        assert_eq!(spatial_source_alignment & (spatial_source_alignment - 1), 0);
    }
}
