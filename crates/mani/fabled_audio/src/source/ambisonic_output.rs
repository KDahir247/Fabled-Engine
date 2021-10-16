use crate::{OutputConfig, RawAmbisonicClip, SpatialAmbisonicSource};
use ambisonic::rodio::Source;

pub struct AmbisonicOutput {
    #[allow(dead_code)]
    sink: ambisonic::rodio::Sink,
    #[allow(dead_code)]
    output_stream: ambisonic::rodio::OutputStream,
    composer: std::sync::Arc<ambisonic::BmixerComposer>,
}

impl Default for AmbisonicOutput {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

impl AmbisonicOutput {
    fn new() -> Option<Self> {
        let OutputConfig {
            output_device,
            output_config,
        } = OutputConfig::default();

        match (output_device, output_config) {
            (Some(device), Some(config)) => {
                let (b_mixer, b_controller) = ambisonic::bmixer(config.sample_rate);

                let (output_stream, output_handle) =
                    ambisonic::rodio::OutputStream::try_from_device(&device).unwrap();

                let sink = ambisonic::rodio::Sink::try_new(&output_handle).unwrap();

                match ambisonic::PlaybackConfiguration::default() {
                    ambisonic::PlaybackConfiguration::Stereo(cfg) => {
                        let output = ambisonic::BstreamStereoRenderer::new(b_mixer, cfg);

                        sink.append(output);
                    }
                    ambisonic::PlaybackConfiguration::Hrtf(cfg) => {
                        let output = ambisonic::BstreamHrtfRenderer::new(b_mixer, cfg);

                        sink.append(output);
                    }
                }

                Some(Self {
                    sink,
                    output_stream,
                    composer: b_controller,
                })
            }
            _ => None,
        }
    }


    pub fn play_omni(&self, clip: RawAmbisonicClip, volume: f32) -> SpatialAmbisonicSource {
        let dyn_clip = clip.dyn_clip;

        let channels = dyn_clip.channels();

        let channel_volume =
            ambisonic::rodio::source::ChannelVolume::new(dyn_clip, vec![volume; channels as usize]);

        let sound_controller = self
            .composer
            .play(channel_volume, ambisonic::BstreamConfig::default());

        SpatialAmbisonicSource::new(sound_controller)
    }


    pub fn play_at(
        &self,
        clip: RawAmbisonicClip,
        volume: f32,
        init_pos: [f32; 3],
    ) -> SpatialAmbisonicSource {
        let sound_controller = self.composer.play(
            clip.dyn_clip.amplify(volume),
            ambisonic::BstreamConfig::new().with_position(init_pos),
        );

        SpatialAmbisonicSource::new(sound_controller)
    }

    pub fn set_global_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }

    pub fn get_global_volume(&self) -> f32 {
        self.sink.volume()
    }
}


#[cfg(test)]
mod ambisonic_output_test {
    use crate::{AmbisonicOutput, RawAmbisonicClip};
    use std::io::Read;

    fn retrieve_audio_buffer() -> Vec<u8> {
        let path = &[env!("CARGO_MANIFEST_DIR"), "/src/audio/test.mp3"].join("");
        let mut file = std::fs::File::open(path).unwrap();
        let mut audio_buffer = vec![0; file.metadata().unwrap().len() as usize];
        file.read_exact(&mut audio_buffer).unwrap();

        audio_buffer
    }

    #[test]
    fn creation_test() {
        let _ambisonic_output = AmbisonicOutput::default();

        let another_ambisonic_output = AmbisonicOutput::new();
        assert!(another_ambisonic_output.is_some());
    }


    #[test]
    fn volume_test() {
        let ambisonic_output = AmbisonicOutput::default();
        assert!(ambisonic_output.get_global_volume().eq(&1.0));

        ambisonic_output.set_global_volume(0.5);
        assert!(ambisonic_output.get_global_volume().eq(&0.5));
    }


    #[test]
    fn omni_test() {
        use crate::AudioClip;

        let audio_buffer = retrieve_audio_buffer();

        let standard_output = AmbisonicOutput::default();

        let audio_clip: AudioClip<f32> = AudioClip::from_raw(audio_buffer, true).unwrap();
        let raw_clip = RawAmbisonicClip::from(audio_clip);

        standard_output.play_omni(raw_clip, 1.0);

        std::thread::sleep(std::time::Duration::from_secs(2));
    }


    #[test]
    fn spatial_test() {
        use crate::AudioClip;

        let audio_buffer = retrieve_audio_buffer();

        let standard_output = AmbisonicOutput::default();

        let audio_clip: AudioClip<f32> = AudioClip::from_raw(audio_buffer, true).unwrap();
        let raw_clip = RawAmbisonicClip::from(audio_clip).repeat();

        let mut sound = standard_output.play_at(raw_clip, 2.0, [50.0, 1.0, 0.0]);

        sound.set_velocity([-10.0, 0.0, 0.0]);
        for i in 0..1000 {
            sound.set_position([50.0 - i as f32 / 10.0, 1.0, 0.0]);
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        sound.set_velocity([0.0, 0.0, 0.0]);
    }
}
