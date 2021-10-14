mod clip;
mod codecs;
mod config;
mod container;
mod dsp;
mod error;
mod mixer;
mod renderer;
mod source;

pub use clip::*;
pub use codecs::*;
pub use config::*;
pub use container::*;
pub use dsp::*;
pub use error::*;
pub use mixer::*;
pub use renderer::*;
pub use source::*;


#[cfg(test)]
mod tests {
    use crate::{
        AmbisonicCollection, AmbisonicOutput, AudioClip, AudioType, RawAmbisonicClip, RawClip,
        StandardOutput,
    };

    use std::io::Read;

    #[test]
    fn sequence_player_test() {
        // 0
        let path = &[env!("CARGO_MANIFEST_DIR"), "/src/audio/test.mp3"].join("");
        let file = std::fs::File::open(path).unwrap();

        // 1
        let path = &[env!("CARGO_MANIFEST_DIR"), "/src/audio/epic.mp3"].join("");
        let file1 = std::fs::File::open(path).unwrap();

        //---------------------- Creating the Clip ------------------
        let standard_output = AmbisonicOutput::default();

        let audio_clip: AudioClip<f32> = AudioClip::from_file(AudioType::Loose(file), true);
        let raw_clip = RawAmbisonicClip::from(audio_clip);

        let audio_clip: AudioClip<f32> = AudioClip::from_file(AudioType::Loose(file1), true);
        // you can transform the audio before putting them in a collection.
        let raw_clip1 = RawAmbisonicClip::from(audio_clip).speed(1.3);

        //--------------- Creating the Clip Collection --------------

        let audio_collection = AmbisonicCollection::new(true);

        audio_collection.append(raw_clip);
        audio_collection.append(raw_clip1);

        let combined_clip = audio_collection.retrieve_output();

        //---------------------- Playing the Clip ------------------

        standard_output.play_omni(combined_clip, 0.3);
        std::thread::sleep(std::time::Duration::from_secs(10000));
    }

    #[test]
    fn it_works() {
        let path = &[env!("CARGO_MANIFEST_DIR"), "/src/audio/epic1.mp3"].join("");
        let file = std::fs::File::open(path).unwrap();
        //---------------------- Creating the Clip ------------------
        let standard_output = StandardOutput::default();

        let audio_clip: AudioClip<f32> = AudioClip::from_file(AudioType::Loose(file), true);

        let raw_clip = RawClip::from(audio_clip);

        standard_output.play_omni(raw_clip, 0.2);

        std::thread::sleep(std::time::Duration::from_secs(10000));
    }
}
