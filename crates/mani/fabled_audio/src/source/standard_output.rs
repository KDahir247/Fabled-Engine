// This will be similar to audio_spatial_output, except that the output audio
// will be non spatial and will not play the clip via ambisonic

use crate::{AudioListener, OutputConfig, RawClip};

pub struct StandardOutput {
    // we need to keep the sink and output alive for playing the audio.
    #[allow(dead_code)]
    sink: Option<rodio::SpatialSink>,
    #[allow(dead_code)]
    output_stream: Option<rodio::OutputStream>,
    composer: std::sync::Arc<rodio::dynamic_mixer::DynamicMixerController<f32>>,
}

impl Default for StandardOutput {
    fn default() -> Self {
        Self::new([0.; 3], AudioListener::default())
    }
}

impl StandardOutput {
    fn new(init_pos: [f32; 3], audio_listener: AudioListener) -> Self {
        let OutputConfig {
            device,
            output_config,
        } = OutputConfig::default();

        let (d_controller, d_mixer) =
            rodio::dynamic_mixer::mixer(output_config.channel_count, output_config.sample_rate);

        let mut out_sink = None;
        let mut out_stream = None;

        if let Ok((output_stream, output_handle)) =
            rodio::OutputStream::try_from_device(&device.unwrap())
        {
            let sink = rodio::SpatialSink::try_new(
                &output_handle,
                init_pos,
                audio_listener.stereo_left_position,
                audio_listener.stereo_right_position,
            )
            .unwrap();

            let zero: rodio::source::Zero<f32> = rodio::source::Zero::new(1, 1);

            // a temporary hack.
            d_controller.add(zero);

            sink.append(d_mixer);
            out_sink = Some(sink);
            out_stream = Some(output_stream);
        }

        Self {
            sink: out_sink,
            output_stream: out_stream,
            composer: d_controller,
        }
    }

    // todo got to handle case where we set a emit position and left and right ear
    // position for the sink
    pub fn play_omni<T>(&self, clip: RawClip<T>, volume: f32)
    where
        T: rodio::Source<Item = f32> + Send + 'static, {
        let inner = clip.get();
        let channel_count = inner.channels();

        let channel_volume =
            rodio::source::ChannelVolume::new(inner, vec![volume; channel_count as usize]);

        self.composer.add(channel_volume);
    }

    // todo currently in progress. Got to return a controller that will let me
    //  change position and update the sink position.
    // we want a function to move the emitter position and the left ear position and
    // the right ear position.
    pub fn play_at<T>(&self, clip: RawClip<T>, volume: f32, init_pos: [f32; 3])
    where
        T: rodio::Source<Item = f32> + Send + 'static, {
        self.sink.as_ref().unwrap().set_emitter_position(init_pos);

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


    // todo temp solution.
    pub fn stop(&self) {
        self.sink.as_ref().unwrap().stop();
    }

    pub fn pause(&self) {
        self.sink.as_ref().unwrap().pause()
    }

    pub fn resume(&self) {
        self.sink.as_ref().unwrap().play();
    }

    pub fn set_position(&mut self, target_position: [f32; 3]) {
        self.sink
            .as_ref()
            .unwrap()
            .set_emitter_position(target_position);
    }

    pub fn set_left_ear_position(&mut self, left_ear_position: [f32; 3]) {
        self.sink
            .as_ref()
            .unwrap()
            .set_left_ear_position(left_ear_position);
    }

    pub fn set_right_ear_position(&mut self, right_ear_position: [f32; 3]) {
        self.sink
            .as_ref()
            .unwrap()
            .set_right_ear_position(right_ear_position);
    }
}
