mod audio_clip;
mod audio_collection;
mod raw_clip;


pub use audio_clip::*;
pub use audio_collection::*;
pub use raw_clip::*;

#[cfg(test)]
mod data_test {
    use crate::{AmbisonicCollection, AudioClip, AudioCollection, RawAmbisonicClip, RawClip};

    #[test]
    fn data_size() {
        let audio_clip_u16_size = std::mem::size_of::<AudioClip<u16>>();
        println!("audio clip u16 {}", audio_clip_u16_size);

        let audio_clip_i16_size = std::mem::size_of::<AudioClip<i16>>();
        println!("audio clip i16 {}", audio_clip_i16_size);

        let audio_clip_f32_size = std::mem::size_of::<AudioClip<f32>>();
        println!("audio clip f32 {}", audio_clip_f32_size);

        //------------------------------------------------------------------------

        let raw_clip_u16_size = std::mem::size_of::<RawClip<u16>>();
        assert_eq!(raw_clip_u16_size & (raw_clip_u16_size - 1), 0);

        let raw_clip_i16_size = std::mem::size_of::<RawClip<i16>>();
        assert_eq!(raw_clip_i16_size & (raw_clip_i16_size - 1), 0);

        let raw_clip_f32_size = std::mem::size_of::<RawClip<f32>>();
        assert_eq!(raw_clip_f32_size & (raw_clip_f32_size - 1), 0);

        //------------------------------------------------------------------------

        let raw_collection_u16_size = std::mem::size_of::<AudioCollection<u16>>();
        println!("raw clip collection u16 {}", raw_collection_u16_size);

        let raw_collection_i16_size = std::mem::size_of::<AudioCollection<i16>>();
        println!("raw clip collection i16 {}", raw_collection_i16_size);

        let raw_collection_f32_size = std::mem::size_of::<AudioCollection<f32>>();
        println!("raw clip collection f32 {}", raw_collection_f32_size);

        //------------------------------------------------------------------------

        let raw_ambisonic_clip_f32_size = std::mem::size_of::<RawAmbisonicClip>();
        assert_eq!(
            raw_ambisonic_clip_f32_size & (raw_ambisonic_clip_f32_size - 1),
            0
        );

        //------------------------------------------------------------------------

        let raw_ambisonic_collection_f32_size = std::mem::size_of::<AmbisonicCollection>();
        println!(
            "raw ambisonic clip collection f32 {}",
            raw_ambisonic_collection_f32_size
        );

        //------------------------------------------------------------------------
    }


    #[test]
    fn data_alignment() {
        let audio_clip_u16_alignment = std::mem::align_of::<AudioClip<u16>>();
        assert_eq!(audio_clip_u16_alignment & (audio_clip_u16_alignment - 1), 0);

        let audio_clip_i16_alignment = std::mem::align_of::<AudioClip<i16>>();
        assert_eq!(audio_clip_i16_alignment & (audio_clip_i16_alignment - 1), 0);

        let audio_clip_f32_alignment = std::mem::align_of::<AudioClip<f32>>();
        assert_eq!(audio_clip_f32_alignment & (audio_clip_f32_alignment - 1), 0);


        //------------------------------------------------------------------------

        let raw_clip_u16_alignment = std::mem::align_of::<RawClip<u16>>();
        assert_eq!(raw_clip_u16_alignment & (raw_clip_u16_alignment - 1), 0);

        let raw_clip_i16_alignment = std::mem::align_of::<RawClip<i16>>();
        assert_eq!(raw_clip_i16_alignment & (raw_clip_i16_alignment - 1), 0);

        let raw_clip_f32_alignment = std::mem::align_of::<RawClip<f32>>();
        assert_eq!(raw_clip_f32_alignment & (raw_clip_f32_alignment - 1), 0);

        //------------------------------------------------------------------------

        let raw_collection_u16_alignment = std::mem::align_of::<AudioCollection<u16>>();
        assert_eq!(
            raw_collection_u16_alignment & (raw_collection_u16_alignment - 1),
            0
        );

        let raw_collection_i16_alignment = std::mem::align_of::<AudioCollection<i16>>();
        assert_eq!(
            raw_collection_i16_alignment & (raw_collection_i16_alignment - 1),
            0
        );

        let raw_collection_f32_alignment = std::mem::align_of::<AudioCollection<f32>>();
        assert_eq!(
            raw_collection_f32_alignment & (raw_collection_f32_alignment - 1),
            0
        );

        //------------------------------------------------------------------------

        let raw_ambisonic_clip_f32_alignment = std::mem::align_of::<RawAmbisonicClip>();
        assert_eq!(
            raw_ambisonic_clip_f32_alignment & (raw_ambisonic_clip_f32_alignment - 1),
            0
        );

        //------------------------------------------------------------------------

        let raw_ambisonic_collection_f32_alignment = std::mem::align_of::<AmbisonicCollection>();

        assert_eq!(
            raw_ambisonic_collection_f32_alignment & (raw_ambisonic_collection_f32_alignment - 1),
            0
        );

        //------------------------------------------------------------------------
    }
}
