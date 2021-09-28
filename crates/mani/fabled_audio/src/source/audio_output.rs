// This will be similar to audio_spatial_output, except that the output audio
// will be non spatial and will not play the clip via ambisonic

use crate::{OutputConfig, RawClip};

pub struct AudioOutput {
    // we need to keep the sink and output alive for playing the audio.
    #[allow(dead_code)]
    sink: Option<rodio::Sink>,
    #[allow(dead_code)]
    output_stream: Option<rodio::OutputStream>,
    composer: std::sync::Arc<rodio::dynamic_mixer::DynamicMixerController<f32>>,
}

impl Default for AudioOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioOutput {
    pub fn new() -> Self {
        let OutputConfig {
            device,
            output_config,
        } = OutputConfig::default();

        let (d_controller, d_mixer) =
            rodio::dynamic_mixer::mixer(output_config.channel_count, output_config.sample_rate);

        let mut sink = None;
        let mut out_stream = None;

        if let Ok((output_stream, output_handle)) = rodio::OutputStream::try_from_device(&device) {
            let _sink = rodio::Sink::try_new(&output_handle).unwrap();

            let zero: rodio::source::Zero<f32> = rodio::source::Zero::new(1, 1);

            // a temporary hack.
            d_controller.add(zero);

            _sink.append(d_mixer);
            sink = Some(_sink);
            out_stream = Some(output_stream);
        }

        Self {
            sink,
            output_stream: out_stream,
            composer: d_controller,
        }
    }

    pub fn test<T>(&self, clip: T)
    where
        T: rodio::Source<Item = f32> + Send + 'static, {
        self.composer.add(clip);
    }

    pub fn play<T>(&self, clip: RawClip<T>, volume: f32)
    where
        T: rodio::Source<Item = f32> + Send + 'static, {
        let inner = clip.get();
        let channel_count = inner.channels();

        let channel_volume =
            rodio::source::ChannelVolume::new(inner, vec![volume; channel_count as usize]);


        self.composer.add(channel_volume);
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
