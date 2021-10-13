use crate::{FadeFilter, RawClip};
use rodio::Source;

impl RawClip<f32> {
    pub fn low_pass(self, frequency: u32) -> RawClip<f32> {
        RawClip::new(self.data.low_pass(frequency))
    }
}

impl<T> RawClip<T>
where
    T: rodio::Sample + Send + Sync + 'static,
{
    pub fn buffered(self) -> RawClip<T> {
        RawClip::new(self.data.buffered())
    }

    pub fn mix(self, raw_clip: RawClip<T>) -> RawClip<T> {
        RawClip::new(self.data.mix(raw_clip.data))
    }

    pub fn take_duration(self, seconds: u64, micro_seconds: u32, filter: FadeFilter) -> RawClip<T> {
        let mut take = self
            .data
            .take_duration(std::time::Duration::new(seconds, micro_seconds * 1000));

        take.clear_filter();

        if let FadeFilter::FADE = filter {
            take.set_filter_fadeout();
        }

        RawClip::new(take)
    }

    pub fn delay(self, seconds: u64, micro_seconds: u32) -> RawClip<T> {
        let delay = self
            .data
            .delay(std::time::Duration::new(seconds, micro_seconds * 1000));

        RawClip::new(delay)
    }

    pub fn fade_in(self, seconds: u64, micro_seconds: u32) -> RawClip<T> {
        let fade = self
            .data
            .fade_in(std::time::Duration::new(seconds, micro_seconds * 1000));

        RawClip::new(fade)
    }

    pub fn amplify(self, factor: f32) -> RawClip<T> {
        RawClip::new(self.data.amplify(factor))
    }

    pub fn take_cross_fade_with(
        self,
        seconds: u64,
        micro_seconds: u32,
        raw_clip: RawClip<T>,
    ) -> RawClip<T> {
        let cross_fade = self.data.take_crossfade_with(
            raw_clip.data,
            std::time::Duration::new(seconds, micro_seconds * 1000),
        );
        RawClip::new(cross_fade)
    }

    pub fn reverb(self, seconds: u64, micro_seconds: u32, amplitude: f32) -> RawClip<T> {
        let reverb = self.data.buffered().reverb(
            std::time::Duration::new(seconds, micro_seconds * 1000),
            amplitude,
        );

        RawClip::new(reverb)
    }

    pub fn periodic_access<F: 'static>(
        self,
        seconds: u64,
        micro_seconds: u32,
        access: F,
    ) -> RawClip<T>
    where
        F: FnMut(&mut Box<dyn rodio::Source<Item = T> + Send>) + Send, {
        let access = self.data.periodic_access(
            std::time::Duration::new(seconds, micro_seconds * 1000),
            access,
        );

        RawClip::new(access)
    }

    pub fn repeat(self) -> RawClip<T> {
        RawClip::new(self.data.repeat_infinite())
    }
    pub fn speed(self, factor: f32) -> RawClip<T> {
        RawClip::new(self.data.speed(factor))
    }
}

#[cfg(test)]
mod standard_mixer_test {
    use crate::{AudioClip, RawClip};

    #[test]
    fn standard_test() {
        let clip = AudioClip::create_clip(vec![5.0; 100], 2, 48000, None, true);

        let raw_standard = RawClip::from(clip);

        // Can chain
        let _new_standard = raw_standard.amplify(5.0).fade_in(20, 1).low_pass(120);
    }
}
