use crate::{FadeFilter, RawAmbisonicClip};
use ambisonic::rodio::Source;

impl RawAmbisonicClip {
    pub fn low_pass(self, frequency: u32) -> RawAmbisonicClip {
        RawAmbisonicClip::new(self.data.low_pass(frequency))
    }

    pub fn buffered(self) -> RawAmbisonicClip {
        RawAmbisonicClip::new(self.data.buffered())
    }

    pub fn mix<U>(self, raw_clip: RawAmbisonicClip) -> RawAmbisonicClip
    where
        U: ambisonic::rodio::Source<Item = f32>, {
        RawAmbisonicClip::new(self.data.mix(raw_clip.data))
    }

    pub fn take_duration(
        self,
        seconds: u64,
        micro_seconds: u32,
        filter: FadeFilter,
    ) -> RawAmbisonicClip {
        let mut take = self
            .data
            .take_duration(std::time::Duration::new(seconds, micro_seconds * 1000));

        take.clear_filter();

        if let FadeFilter::FADE = filter {
            take.set_filter_fadeout();
        }

        RawAmbisonicClip::new(take)
    }

    pub fn delay(self, seconds: u64, micro_seconds: u32) -> RawAmbisonicClip {
        let delay = self
            .data
            .delay(std::time::Duration::new(seconds, micro_seconds * 1000));

        RawAmbisonicClip::new(delay)
    }

    pub fn fade_in(self, seconds: u64, micro_seconds: u32) -> RawAmbisonicClip {
        let fade = self
            .data
            .fade_in(std::time::Duration::new(seconds, micro_seconds * 1000));

        RawAmbisonicClip::new(fade)
    }

    pub fn amplify(self, factor: f32) -> RawAmbisonicClip {
        RawAmbisonicClip::new(self.data.amplify(factor))
    }

    pub fn take_crossfade_with<U>(
        self,
        seconds: u64,
        micro_seconds: u32,
        raw_clip: RawAmbisonicClip,
    ) -> RawAmbisonicClip
    where
        U: ambisonic::rodio::Source<Item = f32>, {
        let cross_fade = self.data.take_crossfade_with(
            raw_clip.data,
            std::time::Duration::new(seconds, micro_seconds * 1000),
        );

        RawAmbisonicClip::new(cross_fade)
    }


    pub fn reverb(self, seconds: u64, micro_seconds: u32, amplitude: f32) -> RawAmbisonicClip {
        let reverb = self.data.buffered().reverb(
            std::time::Duration::new(seconds, micro_seconds * 1000),
            amplitude,
        );

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
        let access = self.data.periodic_access(
            std::time::Duration::new(seconds, micro_seconds * 1000),
            access,
        );

        RawAmbisonicClip::new(access)
    }

    pub fn repeat(self) -> RawAmbisonicClip {
        RawAmbisonicClip::new(self.data.repeat_infinite())
    }

    pub fn speed(self, factor: f32) -> RawAmbisonicClip {
        RawAmbisonicClip::new(self.data.speed(factor))
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