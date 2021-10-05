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
        let path = &[env!("CARGO_MANIFEST_DIR"), "/src/audio/epic.mp3"].join("");
        let mut file = std::fs::File::open(path).unwrap();
        let mut audio_buffer = vec![0; file.metadata().unwrap().len() as usize];
        file.read_exact(&mut audio_buffer).unwrap();

        //---------------------- Creating the Clip ------------------
        let standard_output = AmbisonicOutput::default();

        let audio_clip: AudioClip<f32> = AudioClip::from_file(audio_buffer, true);

        let raw_clip = Ambisonic::from(audio_clip).speed(1.3);

        standard_output.play_omni(raw_clip, 1.0);

        std::thread::sleep(std::time::Duration::from_secs(10000));
    }
}
