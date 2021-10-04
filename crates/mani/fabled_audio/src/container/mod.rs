pub use audio_clip::*;
pub use audio_listener::*;
pub use fade_filter::*;
pub use raw_clip::*;
pub use sample_format::*;

mod audio_clip;
mod audio_listener;
mod fade_filter;
mod raw_clip;
mod sample_format;


#[cfg(test)]
mod data_test {
    use crate::{AudioClip, AudioListener, RawAmbisonicClip, RawClip, SampleFormat};

    #[test]
    fn data_size() {
        let audio_clip_u16_size = std::mem::size_of::<AudioClip<u16>>();
        println!("audio clip u16 {}", audio_clip_u16_size);

        let audio_clip_i16_size = std::mem::size_of::<AudioClip<i16>>();
        println!("audio clip i16 {}", audio_clip_i16_size);

        let audio_clip_f32_size = std::mem::size_of::<AudioClip<f32>>();
        println!("audio clip f32 {}", audio_clip_f32_size);

        //------------------------------------------------------------------------

        let audio_clip_u16_size = std::mem::size_of::<RawClip<AudioClip<u16>>>();
        println!("raw clip u16 {}", audio_clip_u16_size);

        let audio_clip_i16_size = std::mem::size_of::<RawClip<AudioClip<i16>>>();
        println!("raw clip i16 {}", audio_clip_i16_size);

        let raw_clip_f32_size = std::mem::size_of::<RawClip<AudioClip<f32>>>();
        println!("raw clip f32 {}", raw_clip_f32_size);

        //------------------------------------------------------------------------

        let audio_clip_u16_size = std::mem::size_of::<RawAmbisonicClip<AudioClip<u16>>>();
        println!("raw ambisonic clip u16 {}", audio_clip_u16_size);

        let audio_clip_i16_size = std::mem::size_of::<RawAmbisonicClip<AudioClip<i16>>>();
        println!("raw ambisonic clip i16 {}", audio_clip_i16_size);

        let raw_ambisonic_clip_f32_size = std::mem::size_of::<RawAmbisonicClip<AudioClip<f32>>>();
        println!("raw ambisonic clip f32 {}", raw_ambisonic_clip_f32_size);

        //------------------------------------------------------------------------

        let audio_listener_size = std::mem::size_of::<AudioListener>();
        println!("{}", audio_listener_size);

        let sample_format_size = std::mem::size_of::<SampleFormat>();
        println!("{}", sample_format_size);
    }


    #[test]
    fn data_alignment() {
        let audio_clip_u16_alignment = std::mem::align_of::<AudioClip<u16>>();
        println!("audio clip u16 {}", audio_clip_u16_alignment);

        let audio_clip_i16_alignment = std::mem::align_of::<AudioClip<i16>>();
        println!("audio clip i16 {}", audio_clip_i16_alignment);

        let audio_clip_f32_alignment = std::mem::align_of::<AudioClip<f32>>();
        println!("audio clip f32 {}", audio_clip_f32_alignment);

        //------------------------------------------------------------------------
        let raw_clip_u16_alignment = std::mem::align_of::<RawClip<AudioClip<u16>>>();
        println!("raw clip u16 {}", raw_clip_u16_alignment);

        let raw_clip_i16_alignment = std::mem::align_of::<RawClip<AudioClip<i16>>>();
        println!("raw clip i16 {}", raw_clip_i16_alignment);

        let raw_clip_f32_alignment = std::mem::align_of::<RawClip<AudioClip<f32>>>();
        println!("raw clip f32 {}", raw_clip_f32_alignment);

        //------------------------------------------------------------------------
        let raw_ambisonic_clip_u16_alignment =
            std::mem::align_of::<RawAmbisonicClip<AudioClip<u16>>>();
        println!("raw ambisonic u16 {}", raw_ambisonic_clip_u16_alignment);

        let raw_ambisonic_clip_i16_alignment =
            std::mem::align_of::<RawAmbisonicClip<AudioClip<i16>>>();
        println!("raw ambisonic i16 {}", raw_ambisonic_clip_i16_alignment);

        let raw_ambisonic_clip_f32_alignment =
            std::mem::align_of::<RawAmbisonicClip<AudioClip<f32>>>();
        println!("raw ambisonic f32 {}", raw_ambisonic_clip_f32_alignment);

        //------------------------------------------------------------------------

        let audio_listener_alignment = std::mem::align_of::<AudioListener>();
        println!("{}", audio_listener_alignment);

        let sample_format_alignment = std::mem::align_of::<SampleFormat>();
        println!("{}", sample_format_alignment);
    }
}
