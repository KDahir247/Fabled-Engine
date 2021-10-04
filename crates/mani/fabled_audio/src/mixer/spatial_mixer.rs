use crate::{FadeFilter, RawAmbisonicClip};
use ambisonic::rodio::Source;

type Reverb<T> = RawAmbisonicClip<
    ambisonic::rodio::source::Mix<
        ambisonic::rodio::source::Buffered<T>,
        ambisonic::rodio::source::Delay<
            ambisonic::rodio::source::Amplify<ambisonic::rodio::source::Buffered<T>>,
        >,
    >,
>;

impl<T> RawAmbisonicClip<T>
where
    T: ambisonic::rodio::Source<Item = f32>,
{
    pub fn buffered(self) -> RawAmbisonicClip<ambisonic::rodio::source::Buffered<T>> {
        RawAmbisonicClip::new(self.get().buffered())
    }

    pub fn low_pass(
        self,
        frequency: u32,
    ) -> RawAmbisonicClip<ambisonic::rodio::source::BltFilter<T>> {
        RawAmbisonicClip::new(self.get().low_pass(frequency))
    }


    pub fn mix<U: ambisonic::rodio::source::Source<Item = f32>>(
        self,
        raw_clip: RawAmbisonicClip<U>,
    ) -> RawAmbisonicClip<ambisonic::rodio::source::Mix<T, U>> {
        RawAmbisonicClip::new(self.get().mix(raw_clip.get()))
    }

    pub fn take_duration(
        self,
        seconds: u64,
        micro_seconds: u32,
        filter: FadeFilter,
    ) -> RawAmbisonicClip<ambisonic::rodio::source::TakeDuration<T>> {
        let mut take = self
            .get()
            .take_duration(std::time::Duration::new(seconds, micro_seconds * 1000));

        take.clear_filter();

        if let FadeFilter::FADE = filter {
            take.set_filter_fadeout();
        }

        RawAmbisonicClip::new(take)
    }


    pub fn delay(
        self,
        seconds: u64,
        micro_seconds: u32,
    ) -> RawAmbisonicClip<ambisonic::rodio::source::Delay<T>> {
        let delay = self
            .get()
            .delay(std::time::Duration::new(seconds, micro_seconds * 1000));

        RawAmbisonicClip::new(delay)
    }

    pub fn fade_in(
        self,
        seconds: u64,
        micro_seconds: u32,
    ) -> RawAmbisonicClip<ambisonic::rodio::source::FadeIn<T>> {
        let fade = self
            .get()
            .fade_in(std::time::Duration::new(seconds, micro_seconds * 1000));

        RawAmbisonicClip::new(fade)
    }

    pub fn amplify(self, factor: f32) -> RawAmbisonicClip<ambisonic::rodio::source::Amplify<T>> {
        RawAmbisonicClip::new(self.get().amplify(factor))
    }

    pub fn take_crossfade_with<U: ambisonic::rodio::source::Source<Item = f32>>(
        self,
        seconds: u64,
        micro_seconds: u32,
        raw_clip: RawAmbisonicClip<U>,
    ) -> RawAmbisonicClip<ambisonic::rodio::source::Crossfade<T, U>> {
        let cross_fade = self.get().take_crossfade_with(
            raw_clip.get(),
            std::time::Duration::new(seconds, micro_seconds * 1000),
        );

        RawAmbisonicClip::new(cross_fade)
    }


    pub fn reverb(self, seconds: u64, micro_seconds: u32, amplitude: f32) -> Reverb<T> {
        let reverb = self.buffered().get().reverb(
            std::time::Duration::new(seconds, micro_seconds * 1000),
            amplitude,
        );

        RawAmbisonicClip::new(reverb)
    }

    pub fn periodic_access<F>(
        self,
        seconds: u64,
        micro_seconds: u32,
        access: F,
    ) -> RawAmbisonicClip<ambisonic::rodio::source::PeriodicAccess<T, F>>
    where
        F: FnMut(&mut T), {
        let access = self.get().periodic_access(
            std::time::Duration::new(seconds, micro_seconds * 1000),
            access,
        );

        RawAmbisonicClip::new(access)
    }

    pub fn repeat(self) -> RawAmbisonicClip<ambisonic::rodio::source::Repeat<T>> {
        RawAmbisonicClip::new(self.get().repeat_infinite())
    }

    pub fn speed(self, factor: f32) -> RawAmbisonicClip<ambisonic::rodio::source::Speed<T>> {
        RawAmbisonicClip::new(self.get().speed(factor))
    }
}
