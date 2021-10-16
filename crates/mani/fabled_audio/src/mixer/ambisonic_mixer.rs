use crate::{FadeFilter, RawAmbisonicClip};
use ambisonic::rodio::Source;

impl RawAmbisonicClip {
    pub fn low_pass(self, frequency: u32) -> RawAmbisonicClip {
        RawAmbisonicClip::new(self.dyn_clip.low_pass(frequency))
    }

    pub fn buffered(self) -> RawAmbisonicClip {
        RawAmbisonicClip::new(self.dyn_clip.buffered())
    }

    pub fn mix<U>(self, raw_clip: RawAmbisonicClip) -> RawAmbisonicClip
    where
        U: ambisonic::rodio::Source<Item = f32>, {
        RawAmbisonicClip::new(self.dyn_clip.mix(raw_clip.dyn_clip))
    }

    pub fn take_duration(
        self,
        seconds: u64,
        micro_seconds: u32,
        filter: FadeFilter,
    ) -> RawAmbisonicClip {
        let micro_seconds = micro_seconds * 1000;

        let mut take = self
            .dyn_clip
            .take_duration(std::time::Duration::new(seconds, micro_seconds));

        take.clear_filter();

        if let FadeFilter::FADE = filter {
            take.set_filter_fadeout();
        }

        RawAmbisonicClip::new(take)
    }

    pub fn delay(self, seconds: u64, micro_seconds: u32) -> RawAmbisonicClip {
        let micro_seconds = micro_seconds * 1000;

        let delay = self
            .dyn_clip
            .delay(std::time::Duration::new(seconds, micro_seconds));

        RawAmbisonicClip::new(delay)
    }

    pub fn fade_in(self, seconds: u64, micro_seconds: u32) -> RawAmbisonicClip {
        let micro_seconds = micro_seconds * 1000;

        let fade = self
            .dyn_clip
            .fade_in(std::time::Duration::new(seconds, micro_seconds));

        RawAmbisonicClip::new(fade)
    }

    pub fn amplify(self, factor: f32) -> RawAmbisonicClip {
        RawAmbisonicClip::new(self.dyn_clip.amplify(factor))
    }

    pub fn take_crossfade_with<U>(
        self,
        seconds: u64,
        micro_seconds: u32,
        raw_clip: RawAmbisonicClip,
    ) -> RawAmbisonicClip
    where
        U: ambisonic::rodio::Source<Item = f32>, {
        let micro_seconds = micro_seconds * 1000;

        let cross_fade = self.dyn_clip.take_crossfade_with(
            raw_clip.dyn_clip,
            std::time::Duration::new(seconds, micro_seconds),
        );

        RawAmbisonicClip::new(cross_fade)
    }


    pub fn reverb(self, seconds: u64, micro_seconds: u32, amplitude: f32) -> RawAmbisonicClip {
        let micro_seconds = micro_seconds * 1000;

        let reverb = self
            .dyn_clip
            .buffered()
            .reverb(std::time::Duration::new(seconds, micro_seconds), amplitude);

        RawAmbisonicClip::new(reverb)
    }

    pub fn periodic_access<T, F: 'static>(
        self,
        seconds: u64,
        micro_seconds: u32,
        access: F,
    ) -> RawAmbisonicClip
    where
        T: ambisonic::rodio::Source<Item = f32>,
        F: FnMut(&mut Box<dyn ambisonic::rodio::Source<Item = f32> + Send>) + Send, {
        let micro_seconds = micro_seconds * 1000;

        let access = self
            .dyn_clip
            .periodic_access(std::time::Duration::new(seconds, micro_seconds), access);

        RawAmbisonicClip::new(access)
    }

    pub fn repeat(self) -> RawAmbisonicClip {
        RawAmbisonicClip::new(self.dyn_clip.repeat_infinite())
    }

    pub fn speed(self, factor: f32) -> RawAmbisonicClip {
        RawAmbisonicClip::new(self.dyn_clip.speed(factor))
    }
}


#[cfg(test)]
mod ambisonic_mixer_test {
    use crate::{AudioClip, RawAmbisonicClip};

    #[test]
    fn ambisonic_test() {
        let clip = AudioClip::create_clip(vec![5.0; 100], 2, 48000, None, true);

        let raw_ambisonic = RawAmbisonicClip::from(clip);

        // Can chain
        let _new_ambisonic = raw_ambisonic.amplify(5.0).fade_in(20, 1).low_pass(120);
    }
}
