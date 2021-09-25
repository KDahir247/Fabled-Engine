mod audio_clip;
mod audio_listener;
mod audio_output;
mod audio_spatial_output;
mod audio_system;
mod audio_track;
mod sample_format;
mod spatial_source;

pub use audio_clip::*;
pub use audio_listener::*;
pub use audio_spatial_output::*;
pub use audio_system::*;
pub use audio_track::*;
pub use sample_format::*;
pub use spatial_source::*;

#[cfg(test)]
mod data_test {
    use crate::{AudioClip, AudioSpatialOutput};

    #[test]
    fn data_size() {
        let audio_clip_size = std::mem::size_of::<AudioClip>();
        println!("{}", audio_clip_size);

        let a = std::mem::size_of::<AudioSpatialOutput>();
        println!("{}", a);
    }


    #[test]
    fn data_alignment() {
        let audio_clip_alignment = std::mem::align_of::<AudioClip>();
        println!("{}", audio_clip_alignment);
    }
}
