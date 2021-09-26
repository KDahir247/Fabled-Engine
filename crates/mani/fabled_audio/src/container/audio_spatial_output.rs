use crate::{AudioListener, Clip, OutputConfig, SpatialSource};
use ambisonic::rodio::Source;
use ambisonic::PlaybackConfiguration;

pub struct AudioSpatialOutput {
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
        // handle option
        let OutputConfig {
            device,
            output_config,
        } = OutputConfig::default();


        let (output_stream, output_handle) =
            ambisonic::rodio::OutputStream::try_from_device(&device).unwrap();

        // handle un-recognized file format, handle NoDevice (Device losted)
        let spatial_sink = ambisonic::rodio::SpatialSink::try_new(
            &output_handle,
            init_pos,
            audio_listener.stereo_left_position,
            audio_listener.stereo_right_position,
        )
        .unwrap();

        let (b_mixer, b_controller) = ambisonic::bmixer(output_config.sample_rate);

        match ambisonic::PlaybackConfiguration::default() {
            PlaybackConfiguration::Stereo(cfg) => {
                let output = ambisonic::BstreamStereoRenderer::new(b_mixer, cfg);
                spatial_sink.append(output);
            }
            PlaybackConfiguration::Hrtf(cfg) => {
                let output = ambisonic::BstreamHrtfRenderer::new(b_mixer, cfg);
                spatial_sink.append(output);
            }
        };

        Self {
            sink: Some(spatial_sink),
            output_stream: Some(output_stream),
            composer: b_controller,
        }
    }

    pub fn play_omni<T>(&self, clip: T) -> SpatialSource
    where
        T: ambisonic::rodio::Source<Item = f32> + Send + 'static, {
        let sound_controller = self.composer.play(clip, ambisonic::BstreamConfig::new());


        SpatialSource::new(sound_controller)
    }


    // todo this should handle source, since we want the user to effect the clip
    //  such as fade_in. Got to swap Clip trait for something better.
    pub fn play_at<T>(&self, clip: T, init_pos: [f32; 3]) -> SpatialSource
    where
        T: ambisonic::rodio::Source<Item = f32> + Send + 'static, {
        let sound_controller = self.composer.play(
            clip,
            ambisonic::BstreamConfig::new().with_position(init_pos),
        );

        SpatialSource::new(sound_controller)
    }
}
