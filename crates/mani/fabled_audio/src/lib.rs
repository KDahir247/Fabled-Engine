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
        let mut standard_output = StandardOutput::default();

        let spatial_output = AmbisonicOutput::default();

        let audio_clip1 = AudioClip::from_file(buffer1, true);
        let audio_clip = AudioClip::from_file(buffer, true);

        let raw_clip = Standard::from(audio_clip1);

        standard_output.play_omni(raw_clip, 1.0);
        // let mut a = spatial_output.play_at(raw_clip, 1.0, [10.0, 0.0, 0.0]);


        // a.set_doppler(0.3);
        // for _ in 0..5 {
        // a.set_velocity([-10.0, 0.0, 0.0]);
        // for i in 1..1001 {
        // std::thread::sleep(std::time::Duration::from_millis(10));
        // standard_output.set_position([50. - i as f32 / 10.0, 0.0, 0.0]);
        // }
        //
        // a.set_velocity([10.0, 0.0, 0.0]);
        // for i in 1..1001 {
        // std::thread::sleep(std::time::Duration::from_millis(10));
        // standard_output.set_position([-50. + i as f32 / 10.0, 0.0, 0.0]);
        // }
        // }

        // standard_output.set_velocity([0.0; 3]);


        // let sound = spatial_output.play_omni(raw_clip, 1.0);

        std::thread::sleep(std::time::Duration::from_secs(1000));
    }
}
