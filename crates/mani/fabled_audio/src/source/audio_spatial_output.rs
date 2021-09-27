use crate::{AudioListener, OutputConfig, RawAmbisonicClip, SpatialSource};
use ambisonic::PlaybackConfiguration;

pub struct AudioSpatialOutput {
    // we need to keep the sink and output alive for playing the audio.
    #[allow(dead_code)]
    sink: Option<ambisonic::rodio::SpatialSink>,
    #[allow(dead_code)]
    output_stream: Option<ambisonic::rodio::OutputStream>,
    pub composer: std::sync::Arc<ambisonic::BmixerComposer>,
}

impl Default for AudioSpatialOutput {
    fn default() -> Self {
        Self::new([0.; 3], AudioListener::default())
    }
}

impl AudioSpatialOutput {
    pub fn new(init_pos: [f32; 3], audio_listener: AudioListener) -> Self {
        let OutputConfig {
            device,
            output_config,
        } = OutputConfig::default();

        let (b_mixer, b_controller) = ambisonic::bmixer(output_config.sample_rate);

        let mut spatial_sink = None;
        let mut out_stream = None;

        if let Ok((output_stream, output_handle)) =
            ambisonic::rodio::OutputStream::try_from_device(&device)
        {
            let sink = ambisonic::rodio::SpatialSink::try_new(
                &output_handle,
                init_pos,
                audio_listener.stereo_left_position,
                audio_listener.stereo_right_position,
            )
            .unwrap();

            match ambisonic::PlaybackConfiguration::default() {
                PlaybackConfiguration::Stereo(cfg) => {
                    let output = ambisonic::BstreamStereoRenderer::new(b_mixer, cfg);
                    sink.append(output);
                }
                PlaybackConfiguration::Hrtf(cfg) => {
                    let output = ambisonic::BstreamHrtfRenderer::new(b_mixer, cfg);
                    sink.append(output);
                }
            };

            out_stream = Some(output_stream);
            spatial_sink = Some(sink);
        }

        Self {
            sink: spatial_sink,
            output_stream: out_stream,
            composer: b_controller,
        }
    }

    // pick up sound equally from all directions, so we can combine input to a
    // single mono source. one channel so what ever you hear from the left speaker
    // you will also hear from the right equally
    pub fn play_omni<T>(&self, clip: RawAmbisonicClip<T>, volume: f32) -> SpatialSource
    where
        T: ambisonic::rodio::Source<Item = f32> + Send + 'static, {
        let inner = clip.get();
        let channel_count = inner.channels();

        let channel_volume = ambisonic::rodio::source::ChannelVolume::new(
            inner,
            vec![volume.max(1.0); channel_count as usize],
        );

        let sound_controller = self
            .composer
            .play(channel_volume, ambisonic::BstreamConfig::new());

        SpatialSource::new(sound_controller)
    }


    pub fn play_at<T>(
        &self,
        clip: RawAmbisonicClip<T>,
        volume: f32,
        init_pos: [f32; 3],
    ) -> SpatialSource
    where
        T: ambisonic::rodio::Source<Item = f32> + Send + 'static, {
        let sound_controller = self.composer.play(
            clip.get().amplify(volume.max(1.0)),
            ambisonic::BstreamConfig::new().with_position(init_pos),
        );

        SpatialSource::new(sound_controller)
    }

    pub fn global_volume(&self, volume: f32) {
        if let Some(sink) = self.sink.as_ref() {
            sink.set_volume(volume);
        }
    }

    pub fn get_volume(&self) -> f32 {
        self.sink
            .as_ref()
            .map_or(0.0, |valid_sink| valid_sink.volume())
    }
}
