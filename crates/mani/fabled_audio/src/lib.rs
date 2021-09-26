// Audio Death Is Just Another Path.mp3 Made by Otto Halmén
// https://opengameart.org/content/death-is-just-another-path

mod codecs;
mod config;
mod container;
mod contract;
mod dsp;
mod ext;
mod mixer;
mod source;

pub use codecs::*;
pub use config::*;
pub use container::*;
pub use contract::*;
pub use dsp::*;
pub use ext::*;
pub use mixer::*;
pub use source::*;


#[cfg(test)]
mod tests {
    use crate::{Ambisonic, AudioClip, AudioSpatialOutput, RawAmbisonicClip};
    use ambisonic::rodio::Source;
    use std::io::Read;

    #[test]
    fn it_works() {
        // ------------------- initial set up-------------------

        //--------------------- Loading the File ----------------

        ambisonic::AmbisonicBuilder::new();

        let mut dir_path = String::new();
        let cargo_dir = env!("CARGO_MANIFEST_DIR");
        dir_path.push_str(cargo_dir);
        dir_path.push_str("/src/audio/RSE_pokecenter.mp3");

        let mut file = std::fs::File::open(dir_path.as_str()).unwrap();
        let file_length = file.metadata().unwrap();

        let mut buffer = Vec::with_capacity(file_length.len() as usize);
        file.read_to_end(&mut buffer).unwrap();


        let mut dir_path = String::new();
        let cargo_dir = env!("CARGO_MANIFEST_DIR");
        dir_path.push_str(cargo_dir);
        dir_path.push_str("/src/audio/Peractorum.mp3");

        let mut file = std::fs::File::open(dir_path.as_str()).unwrap();
        let file_length = file.metadata().unwrap();

        let mut buffer1 = Vec::with_capacity(file_length.len() as usize);
        file.read_to_end(&mut buffer1).unwrap();
        //---------------------- Creating the Clip ------------------

        let output = AudioSpatialOutput::default();


        let audio_clip = AudioClip::from_file(buffer);
        let audio_clip1 = AudioClip::from_file(buffer1);


        // --------------------- Creating the source-------------------------

        let raw_clip = Ambisonic::from(audio_clip1);
        let raw_clip1 = Ambisonic::from(audio_clip);


        let spatial_source = output.play_at(raw_clip, [10.0, 0.0, 0.0]);

        std::thread::sleep(std::time::Duration::from_secs(100000));
    }
}
