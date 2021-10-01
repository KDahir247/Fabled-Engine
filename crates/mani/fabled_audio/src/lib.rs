// Audio Death Is Just Another Path.mp3 Made by Otto Halmén
// https://opengameart.org/content/death-is-just-another-path

//"The Pilgrimage" composed, performed, mixed and mastered by Viktor Kraus
mod codecs;
mod config;
mod container;
mod contract;
mod dsp;
mod error;
mod ext;
mod mixer;
mod renderer;
mod source;

pub use codecs::*;
pub use config::*;
pub use container::*;
pub use contract::*;
pub use dsp::*;
pub use error::*;
pub use ext::*;
pub use mixer::*;
pub use renderer::*;
pub use source::*;

#[cfg(test)]
mod tests {
    use crate::{Ambisonic, AudioClip, AudioOutput, AudioSpatialOutput, RawClip, Standard};

    use ambisonic::rodio::Source as OtherSource;
    use rodio::Source;
    use std::io::Read;

    #[test]
    fn it_works() {
        let mut dir_path = String::new();
        let cargo_dir = env!("CARGO_MANIFEST_DIR");
        dir_path.push_str(cargo_dir);
        dir_path.push_str(
            "/src/audio/WolfgangAmadeusMozart-SymphonyNo.40InGMinorK.550-04-AllegroAssai.flac",
        );

        let mut file = std::fs::File::open(dir_path.as_str()).unwrap();
        let file_length = file.metadata().unwrap();

        let mut buffer1 = Vec::with_capacity(file_length.len() as usize);
        file.read_to_end(&mut buffer1).unwrap();


        //---------------------- Creating the Clip ------------------
        let spatial_output = AudioSpatialOutput::default();
        let standard_output = AudioOutput::default();

        let audio_clip1 = AudioClip::from_file(buffer1, true);

        let raw_clip = Standard::from(audio_clip1);

        standard_output.play(raw_clip, 1.0);
        // let sound = spatial_output.play_at(raw_clip, 1.0, [4.0, 0., 0.]);

        std::thread::sleep(std::time::Duration::from_secs(10000));
    }


    #[test]
    fn hrtf() {
        let (stream, handle) = rodio::OutputStream::try_default().unwrap();

        // let sink = rodio::Sink::try_new(&handle).unwrap();

        let mut dir_path = String::new();

        let cargo_dir = env!("CARGO_MANIFEST_DIR");
        dir_path.push_str(cargo_dir);
        dir_path.push_str("/src/audio/IRC_1002_C.bin");


        let mut music = String::new();
        let cargo_dir = env!("CARGO_MANIFEST_DIR");
        music.push_str(cargo_dir);
        music.push_str("/src/audio/drop.wav");

        let mut file = std::fs::File::open(music.as_str()).unwrap();
        let file_length = file.metadata().unwrap();

        let mut buffer1 = Vec::with_capacity(file_length.len() as usize);
        file.read_to_end(&mut buffer1).unwrap();

        let hrir_sphere = hrtf::HrirSphere::from_file(dir_path.as_str(), 44100).unwrap();


        let decoder = rodio::Decoder::new(std::io::Cursor::new(buffer1)).unwrap();


        let source = ambisonic::sources::Noise::new(4800);
        let channel = source.channels();
        let sample = source.sample_rate();
        let duration = source.total_duration();
        let curr = source.current_frame_len();

        let data = source.take(1024).collect::<Vec<f32>>();

        let mut processor = hrtf::HrtfProcessor::new(hrir_sphere, 8, 128);

        let mut output = vec![(0.0, 0.0); data.len()];
        let mut prev_left_samples = vec![];
        let mut prev_right_samples = vec![];

        println!("{:?}", data);

        let context = hrtf::HrtfContext {
            source: &data,
            output: &mut output,
            new_sample_vector: hrtf::Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            prev_sample_vector: hrtf::Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            prev_left_samples: &mut prev_left_samples,
            prev_right_samples: &mut prev_right_samples,
            new_distance_gain: 1.0,
            prev_distance_gain: 1.0,
        };


        processor.process_samples(context);


        let audio = AudioClip {
            data: output.iter().map(|x| x.0).collect::<Vec<_>>().into_iter(),
            channel: channel,
            sample: sample,
            duration: duration,
            current_frame_len: curr,
        };

        println!("{:?}", audio.data);

        let spatial_output = AudioSpatialOutput::default();

        let a = Ambisonic::from(audio).repeat();

        let mut sound = spatial_output.play_at(a, 0.1, [50.0, 0.0, 0.0]);

        sound.set_velocity([-10.0, 0.0, 0.0]);
        for i in 0..1000 {
            sound.set_position([50.0 - i as f32 / 10.0, 1.0, 0.0]);
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        sound.set_velocity([0.0, 0.0, 0.0]);

        // std::thread::sleep(std::time::Duration::from_secs(10000));
    }
}
