use crate::{RawAmbisonicClip, RawClip};

pub struct AudioCollection<T> {
    audio_collection_output: rodio::queue::SourcesQueueOutput<T>,
    audio_collection_input: std::sync::Arc<rodio::queue::SourcesQueueInput<T>>,
}

impl<T: 'static> AudioCollection<T>
where
    T: rodio::Sample + Send,
{
    pub fn new(keep_alive: bool) -> Self {
        let (queue_input, queue_output) = rodio::queue::queue::<T>(keep_alive);

        Self {
            audio_collection_input: queue_input,
            audio_collection_output: queue_output,
        }
    }

    pub fn append(&self, raw_clip: RawClip<T>) {
        self.audio_collection_input.append(raw_clip.dyn_clip);
    }

    pub fn retrieve_output(self) -> RawClip<T> {
        RawClip::new(self.audio_collection_output)
    }
}

pub struct AmbisonicCollection {
    input: std::sync::Arc<ambisonic::rodio::queue::SourcesQueueInput<f32>>,
    output: ambisonic::rodio::queue::SourcesQueueOutput<f32>,
}

impl AmbisonicCollection {
    pub fn new(keep_alive: bool) -> Self {
        let (queue_input, queue_output) = ambisonic::rodio::queue::queue::<f32>(keep_alive);
        Self {
            input: queue_input,
            output: queue_output,
        }
    }

    pub fn append(&self, raw_ambisonic_clip: RawAmbisonicClip) {
        self.input.append(raw_ambisonic_clip.dyn_clip);
    }

    pub fn retrieve_output(self) -> RawAmbisonicClip {
        RawAmbisonicClip::new(self.output)
    }
}


#[cfg(test)]
mod audio_collection_test {
    use crate::{
        AmbisonicCollection, AmbisonicOutput, AudioCollection, FadeFilter, RawAmbisonicClip,
        RawClip, StandardOutput,
    };

    #[test]
    fn audio_collection() {
        let audio_collection = AudioCollection::new(true);

        let first_clip =
            RawClip::new(rodio::source::SineWave::new(28000)).take_duration(5, 0, FadeFilter::FADE);

        let second_clip = RawClip::new(rodio::source::SineWave::new(1100)).take_duration(
            4,
            0,
            FadeFilter::ABRUPT,
        );

        audio_collection.append(first_clip);
        audio_collection.append(second_clip);

        let combined_clip = audio_collection.retrieve_output();

        let output = StandardOutput::default();

        output.play_omni(combined_clip, 0.1);

        std::thread::sleep(std::time::Duration::from_secs(12));
    }

    #[test]
    fn ambisonic_collection() {
        let ambisonic_collection = AmbisonicCollection::new(true);

        let first_clip = RawAmbisonicClip::new(ambisonic::sources::Noise::new(10000))
            .take_duration(5, 0, FadeFilter::FADE);

        let second_clip = RawAmbisonicClip::new(ambisonic::sources::Noise::new(48000))
            .take_duration(5, 0, FadeFilter::ABRUPT);

        ambisonic_collection.append(first_clip);
        ambisonic_collection.append(second_clip);

        let combined_clip = ambisonic_collection.retrieve_output();

        let output = AmbisonicOutput::default();

        output.play_omni(combined_clip, 0.1);

        std::thread::sleep(std::time::Duration::from_secs(13));
    }
}
