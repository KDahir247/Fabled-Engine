// Audio Death Is Just Another Path.mp3 Made by Otto Halmén
// https://opengameart.org/content/death-is-just-another-path

mod codecs;
mod config;
mod container;
mod contract;
mod dsp;
mod error;
mod ext;
mod mixer;
mod source;

pub use codecs::*;
pub use config::*;
pub use container::*;
pub use contract::*;
pub use dsp::*;
pub use error::*;
pub use ext::*;
pub use mixer::*;
pub use source::*;

#[cfg(test)]
mod tests {
    use crate::{AudioClip, AudioOutput, AudioSpatialOutput, Standard};

    use std::io::Read;

    #[test]
    fn it_works() {
        // ------------------- initial set up-------------------

        //--------------------- Loading the File ----------------

        let mut dir_path = String::new();
        let cargo_dir = env!("CARGO_MANIFEST_DIR");
        dir_path.push_str(cargo_dir);
        dir_path.push_str("/src/audio/epic.mp3");

        let mut file = std::fs::File::open(dir_path.as_str()).unwrap();
        let file_length = file.metadata().unwrap();

        let mut buffer1 = Vec::with_capacity(file_length.len() as usize);
        file.read_to_end(&mut buffer1).unwrap();


        //---------------------- Creating the Clip ------------------

        let spatial_output = AudioSpatialOutput::default();
        let standard_output = AudioOutput::default();

        let audio_clip1 = AudioClip::from_file(buffer1);

        let raw_clip = Standard::from(audio_clip1);


        standard_output.play(raw_clip, 0.6);

        // let sound = spatial_output.play_omni(raw_clip, 1.);


        std::thread::sleep(std::time::Duration::from_secs(100000));
    }
}
