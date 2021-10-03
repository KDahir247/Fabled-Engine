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
        let audio_output_size = std::mem::size_of::<StandardOutput>();
        println!("{}", audio_output_size);

        let audio_spatial_output_size = std::mem::size_of::<AmbisonicOutput>();
        println!("{}", audio_spatial_output_size);


        let spatial_source_size = std::mem::size_of::<SpatialAmbisonicSource>();
        println!("{}", spatial_source_size);
    }


    #[test]
    fn data_alignment() {
        let audio_output_alignment = std::mem::align_of::<StandardOutput>();
        println!("{}", audio_output_alignment);

        let audio_spatial_output_alignment = std::mem::align_of::<AmbisonicOutput>();
        println!("{}", audio_spatial_output_alignment);

        let spatial_source_alignment = std::mem::align_of::<SpatialAmbisonicSource>();
        println!("{}", spatial_source_alignment);
    }
}
