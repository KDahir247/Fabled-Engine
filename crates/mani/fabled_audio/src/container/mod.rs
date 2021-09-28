pub use audio_clip::*;
pub use audio_descriptor::*;
pub use audio_listener::*;
pub use audio_track::*;
pub use raw_clip::*;
pub use sample_format::*;

mod audio_clip;
mod audio_descriptor;
mod audio_listener;
mod audio_track;
mod raw_clip;
mod sample_format;


#[cfg(test)]
mod data_test {
    use crate::{AudioClip, AudioListener, SampleFormat};

    #[test]
    fn data_size() {
        let audio_clip_size = std::mem::size_of::<AudioClip>();
        println!("{}", audio_clip_size);

        let audio_listener_size = std::mem::size_of::<AudioListener>();
        println!("{}", audio_listener_size);

        let sample_format_size = std::mem::size_of::<SampleFormat>();
        println!("{}", sample_format_size);
    }


    #[test]
    fn data_alignment() {
        let audio_clip_alignment = std::mem::align_of::<AudioClip>();
        println!("{}", audio_clip_alignment);

        let audio_listener_alignment = std::mem::align_of::<AudioListener>();
        println!("{}", audio_listener_alignment);

        let sample_format_alignment = std::mem::align_of::<SampleFormat>();
        println!("{}", sample_format_alignment);
    }
}
