use crate::{AudioListener, OutputConfig, RawClip};

pub struct StandardOutput<D> {
    #[allow(dead_code)]
    sink: ambisonic::rodio::SpatialSink,
    #[allow(dead_code)]
    output_stream: ambisonic::rodio::OutputStream,
    composer: std::sync::Arc<ambisonic::rodio::dynamic_mixer::DynamicMixerController<D>>,
}

impl<D: 'static> Default for StandardOutput<D>
where
    D: ambisonic::rodio::Sample + Send + std::fmt::Debug,
{
    fn default() -> Self {
        Self::new(AudioListener::default())
    }
}

impl<D: 'static> StandardOutput<D>
where
    D: ambisonic::rodio::Sample + Send + std::fmt::Debug,
{
    pub fn new(audio_listener: AudioListener) -> Self {
        let OutputConfig {
            output_device,
            output_config,
        } = OutputConfig::default();

        assert_eq!(
            output_config.len(),
            1,
            "there should be only one device config on call to default"
        );

        let output_config = output_config[0];

        let (dyn_controller, dyn_mixer) = ambisonic::rodio::dynamic_mixer::mixer(
            output_config.channel_count,
            output_config.sample_rate,
        );

        let (output_stream, output_handle) =
            ambisonic::rodio::OutputStream::try_from_device(&output_device).unwrap();

        let AudioListener {
            position,
            stereo_left_position,
            stereo_right_position,
        } = audio_listener;

        let sink = ambisonic::rodio::SpatialSink::try_new(
            &output_handle,
            position,
            stereo_left_position,
            stereo_right_position,
        )
        .unwrap();

        let zeroed = ambisonic::rodio::source::Zero::new(
            output_config.channel_count,
            output_config.sample_rate,
        );

        dyn_controller.add(zeroed);

        sink.append(dyn_mixer);

        Self {
            sink,
            output_stream,
            composer: dyn_controller,
        }
    }

    pub fn play_omni(&self, clip: RawClip<D>, volume: f32) {
        let dyn_clip = clip.dyn_clip;

        self.sink.set_volume(volume);

        self.composer.add(dyn_clip);
    }


    pub fn play_at(&self, clip: RawClip<D>, volume: f32, init_pos: [f32; 3]) {
        self.sink.set_emitter_position(init_pos);

        let dyn_clip = clip.dyn_clip;

        self.sink.set_volume(volume);

        self.composer.add(dyn_clip);
    }

    pub fn set_global_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }

    pub fn get_global_volume(&self) -> f32 {
        self.sink.volume()
    }

    pub fn stop(&self) {
        self.sink.stop();
    }

    pub fn pause(&self) {
        self.sink.pause()
    }

    pub fn resume(&self) {
        self.sink.play();
    }

    pub fn set_position(&mut self, target_position: [f32; 3]) {
        self.sink.set_emitter_position(target_position);
    }

    pub fn set_left_ear_position(&mut self, left_ear_position: [f32; 3]) {
        self.sink.set_left_ear_position(left_ear_position);
    }

    pub fn set_right_ear_position(&mut self, right_ear_position: [f32; 3]) {
        self.sink.set_right_ear_position(right_ear_position);
    }
}

#[cfg(test)]
mod standard_output_test {
    use crate::{RawClip, StandardOutput};
    use std::io::Read;

    fn retrieve_audio_buffer() -> Vec<u8> {
        let path = &[env!("CARGO_MANIFEST_DIR"), "/src/audio/test.mp3"].join("");
        let mut file = std::fs::File::open(path).unwrap();
        let mut audio_buffer = vec![0; file.metadata().unwrap().len() as usize];
        file.read_exact(&mut audio_buffer).unwrap();

        audio_buffer
    }


    #[test]
    fn volume_test() {
        let standard_output: StandardOutput<f32> = StandardOutput::default();
        assert!(standard_output.get_global_volume().eq(&1.0));

        standard_output.set_global_volume(0.5);
        assert!(standard_output.get_global_volume().eq(&0.5));
    }

    #[test]
    fn omni_test() {
        use crate::AudioClip;

        let audio_buffer = retrieve_audio_buffer();

        let standard_output = StandardOutput::default();

        let audio_clip: AudioClip<f32> = AudioClip::from_raw(audio_buffer, true).unwrap();
        let raw_clip = RawClip::from(audio_clip);

        standard_output.play_omni(raw_clip, 1.0);

        std::thread::sleep(std::time::Duration::from_secs(2));
    }


    #[test]
    fn spatial_test() {
        use crate::AudioClip;

        let audio_buffer = retrieve_audio_buffer();

        let mut standard_output = StandardOutput::default();

        let audio_clip: AudioClip<f32> = AudioClip::from_raw(audio_buffer, true).unwrap();
        let raw_clip = RawClip::from(audio_clip).repeat();

        standard_output.play_at(raw_clip, 2.0, [50.0, 1.0, 0.0]);

        for i in 0..1000 {
            standard_output.set_position([50.0 - i as f32 / 10.0, 1.0, 0.0]);
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    #[test]
    fn ear_positioning_test() {
        use crate::AudioClip;

        let audio_buffer = retrieve_audio_buffer();

        let mut standard_output = StandardOutput::default();

        let audio_clip: AudioClip<f32> = AudioClip::from_raw(audio_buffer, true).unwrap();
        let raw_clip = RawClip::from(audio_clip).repeat();

        standard_output.play_at(raw_clip, 2.0, [2.0, 1.0, 0.0]);


        for _ in 0..5 {
            for i in 0..100 {
                standard_output.set_right_ear_position([10.0 - i as f32 / 10.0, 1.0, 0.0]);
                standard_output.set_left_ear_position([-10.0 + i as f32 / 10.0, 1.0, 0.0]);

                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            for i in 0..100 {
                standard_output.set_right_ear_position([-10.0 + i as f32 / 10.0, 1.0, 0.0]);
                standard_output.set_left_ear_position([10.0 - i as f32 / 10.0, 1.0, 0.0]);
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }
    }

    #[test]
    fn pause_resume_test() {
        use crate::AudioClip;

        let audio_buffer = retrieve_audio_buffer();

        let standard_output = StandardOutput::default();

        let audio_clip: AudioClip<f32> = AudioClip::from_raw(audio_buffer, true).unwrap();
        let raw_clip = RawClip::from(audio_clip).repeat();

        standard_output.play_omni(raw_clip, 0.3);

        std::thread::sleep(std::time::Duration::from_secs(5));

        standard_output.pause();

        std::thread::sleep(std::time::Duration::from_secs(5));

        standard_output.resume();

        std::thread::sleep(std::time::Duration::from_secs(5));

        standard_output.stop();
    }
}
