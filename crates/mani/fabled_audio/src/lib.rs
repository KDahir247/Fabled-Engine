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
    use crate::{AudioClip, Standard, StandardOutput};
    use std::io::Read;

    #[test]
    fn sequence_player_test() {
        // fast show it works implementation.
        // Both audio must be the same audio sample type.

        // 0
        let path = &[env!("CARGO_MANIFEST_DIR"), "/src/audio/test.mp3"].join("");
        let mut file = std::fs::File::open(path).unwrap();
        let mut audio_buffer = vec![0; file.metadata().unwrap().len() as usize];
        file.read_exact(&mut audio_buffer).unwrap();

        // 1
        let path = &[env!("CARGO_MANIFEST_DIR"), "/src/audio/epic.mp3"].join("");
        let mut file = std::fs::File::open(path).unwrap();
        let mut audio_buffer1 = vec![0; file.metadata().unwrap().len() as usize];
        file.read_exact(&mut audio_buffer1).unwrap();

        //---------------------- Creating the Clip ------------------
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();

        let audio_clip: AudioClip<i16> = AudioClip::from_file(audio_buffer, true);
        let raw_clip = Standard::from(audio_clip);

        let audio_clip: AudioClip<i16> = AudioClip::from_file(audio_buffer1, true);
        let raw_clip1 = Standard::from(audio_clip);

        let audio_queue = rodio::queue::queue::<i16>(true);

        audio_queue.0.append(raw_clip.get());
        audio_queue.0.append(raw_clip1.get());

        sink.append(audio_queue.1);
        sink.sleep_until_end();
    }

    #[test]
    fn it_works() {
        let path = &[env!("CARGO_MANIFEST_DIR"), "/src/audio/recorded.wav"].join("");
        let mut file = std::fs::File::open(path).unwrap();
        let mut audio_buffer = vec![0; file.metadata().unwrap().len() as usize];
        file.read_exact(&mut audio_buffer).unwrap();

        //---------------------- Creating the Clip ------------------
        let standard_output = StandardOutput::default();

        let audio_clip: AudioClip<f32> = AudioClip::from_file(audio_buffer, true);

        // Put low pass so it remove background noise
        let raw_clip = Standard::from(audio_clip);

        standard_output.play_omni(raw_clip, 0.2);

        std::thread::sleep(std::time::Duration::from_secs(10000));
    }
}
