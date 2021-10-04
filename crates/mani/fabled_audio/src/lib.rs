#![feature(thread_id_value)]
// Audio Death Is Just Another Path.mp3 Made by Otto Halmén
// https://opengameart.org/content/death-is-just-another-path

//"The Pilgrimage" composed, performed, mixed and mastered by Viktor Kraus
mod codecs;
mod config;
mod container;
mod dsp;
mod error;
mod mixer;
mod renderer;
mod source;

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
    use crate::{Ambisonic, AmbisonicOutput, AudioClip, RawClip, Standard, StandardOutput};

    use ambisonic::rodio::Source as OtherSource;
    use rodio::Source;
    use std::io::Read;

    #[test]
    fn it_works() {
        let mut dir_path = String::new();
        let cargo_dir = env!("CARGO_MANIFEST_DIR");
        dir_path.push_str(cargo_dir);
        dir_path.push_str(
            "/src/audio/epic\
        .mp3",
        );

        let mut file = std::fs::File::open(dir_path.as_str()).unwrap();
        let file_length = file.metadata().unwrap();

        let mut buffer1 = Vec::with_capacity(file_length.len() as usize);
        file.read_to_end(&mut buffer1).unwrap();

        let mut dir_path = String::new();
        let cargo_dir = env!("CARGO_MANIFEST_DIR");
        dir_path.push_str(cargo_dir);
        dir_path.push_str("/src/audio/epic1.mp3");

        let mut file = std::fs::File::open(dir_path.as_str()).unwrap();
        let file_length = file.metadata().unwrap();

        let mut buffer = Vec::with_capacity(file_length.len() as usize);
        file.read_to_end(&mut buffer).unwrap();


        //---------------------- Creating the Clip ------------------
        let standard_output = StandardOutput::default();

        let audio_clip: AudioClip<f32> = AudioClip::from_file(buffer, true);

        let raw_clip = Standard::from(audio_clip);

        standard_output.play_omni(raw_clip, 0.1);

        std::thread::sleep(std::time::Duration::from_secs(10000));
    }
}
