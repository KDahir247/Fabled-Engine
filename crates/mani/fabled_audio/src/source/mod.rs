mod audio_output;
mod audio_spatial_output;
mod spatial_source;


pub use audio_output::*;
pub use audio_spatial_output::*;
pub use spatial_source::*;


#[cfg(test)]
mod data_test {
    use crate::{AudioOutput, AudioSpatialOutput, SpatialSource};

    #[test]
    fn data_size() {
        let audio_output_size = std::mem::size_of::<AudioOutput>();
        println!("{}", audio_output_size);

        let audio_spatial_output_size = std::mem::size_of::<AudioSpatialOutput>();
        println!("{}", audio_spatial_output_size);


        let spatial_source_size = std::mem::size_of::<SpatialSource>();
        println!("{}", spatial_source_size);
    }


    #[test]
    fn data_alignment() {
        let audio_output_alignment = std::mem::align_of::<AudioOutput>();
        println!("{}", audio_output_alignment);

        let audio_spatial_output_alignment = std::mem::align_of::<AudioSpatialOutput>();
        println!("{}", audio_spatial_output_alignment);

        let spatial_source_alignment = std::mem::align_of::<SpatialSource>();
        println!("{}", spatial_source_alignment);
    }
}
