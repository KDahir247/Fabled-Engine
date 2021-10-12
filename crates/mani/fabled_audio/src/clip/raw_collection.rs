use crate::{RawAmbisonicClip, RawClip};

pub struct RawCollection<T>
where
    T: rodio::Sample + Send + 'static + Copy + Clone, {
    input: std::sync::Arc<rodio::queue::SourcesQueueInput<T>>,
    output: rodio::queue::SourcesQueueOutput<T>,
}

impl<T> RawCollection<T>
where
    T: rodio::Sample + Send + 'static,
{
    pub fn new(keep_alive: bool) -> Self {
        let queue = rodio::queue::queue::<T>(keep_alive);
        Self {
            input: queue.0,
            output: queue.1,
        }
    }

    pub fn append<S>(&self, raw_clip: RawClip<S>)
    where
        S: rodio::Source<Item = T> + Send + 'static, {
        let clip = raw_clip.get();
        self.input.append(clip);
    }

    pub fn retrieve_output(self) -> RawClip<rodio::queue::SourcesQueueOutput<T>> {
        RawClip::new(self.output)
    }
}

pub struct RawAmbisonicCollection {
    #[allow(dead_code)]
    input: std::sync::Arc<ambisonic::rodio::queue::SourcesQueueInput<f32>>,
    #[allow(dead_code)]
    output: ambisonic::rodio::queue::SourcesQueueOutput<f32>,
}

impl RawAmbisonicCollection {
    pub fn new(keep_alive: bool) -> Self {
        let queue = ambisonic::rodio::queue::queue::<f32>(keep_alive);
        Self {
            input: queue.0,
            output: queue.1,
        }
    }

    pub fn append<S>(&self, raw_ambisonic_clip: RawAmbisonicClip<S>)
    where
        S: ambisonic::rodio::Source<Item = f32> + Send + 'static, {
        let clip = raw_ambisonic_clip.get();
        self.input.append(clip);
    }

    pub fn retrieve_output(
        self,
    ) -> RawAmbisonicClip<ambisonic::rodio::queue::SourcesQueueOutput<f32>> {
        RawAmbisonicClip::new(self.output)
    }
}
