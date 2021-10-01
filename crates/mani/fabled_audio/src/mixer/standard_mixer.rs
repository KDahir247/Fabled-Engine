use crate::{FadeFilter, RawClip};

use rodio::Source;

type Reverb<T> = RawClip<
    rodio::source::Mix<
        rodio::source::Buffered<T>,
        rodio::source::Delay<rodio::source::Amplify<rodio::source::Buffered<T>>>,
    >,
>;

impl<T> RawClip<T>
where
    T: rodio::Source<Item = f32>,
{
    pub fn buffered(self) -> RawClip<rodio::source::Buffered<T>> {
        RawClip::new(self.get().buffered())
    }
    pub fn low_pass(self, frequency: u32) -> RawClip<rodio::source::BltFilter<T>> {
        RawClip::new(self.get().low_pass(frequency))
    }

    pub fn mix(self, raw_clip: RawClip<T>) -> RawClip<rodio::source::Mix<T, T>> {
        RawClip::new(self.get().mix(raw_clip.get()))
    }

    pub fn take_duration(
        self,
        seconds: u64,
        micro_seconds: u32,
        filter: FadeFilter,
    ) -> RawClip<rodio::source::TakeDuration<T>> {
        let mut take = self
            .get()
            .take_duration(std::time::Duration::new(seconds, micro_seconds * 1000));

        take.clear_filter();

        if let FadeFilter::FADE = filter {
            take.set_filter_fadeout();
        }

        RawClip::new(take)
    }

    pub fn delay(self, seconds: u64, micro_seconds: u32) -> RawClip<rodio::source::Delay<T>> {
        let delay = self
            .get()
            .delay(std::time::Duration::new(seconds, micro_seconds * 1000));

        RawClip::new(delay)
    }

    pub fn fade_in(self, seconds: u64, micro_seconds: u32) -> RawClip<rodio::source::FadeIn<T>> {
        let fade = self
            .get()
            .fade_in(std::time::Duration::new(seconds, micro_seconds * 1000));

        RawClip::new(fade)
    }

    pub fn amplify(self, factor: f32) -> RawClip<rodio::source::Amplify<T>> {
        RawClip::new(self.get().amplify(factor))
    }

    pub fn take_crossfade_with(
        self,
        seconds: u64,
        micro_seconds: u32,
        raw_clip: RawClip<T>,
    ) -> RawClip<rodio::source::Crossfade<T, T>> {
        let cross_fade = self.get().take_crossfade_with(
            raw_clip.get(),
            std::time::Duration::new(seconds, micro_seconds * 1000),
        );
        RawClip::new(cross_fade)
    }

    pub fn reverb(self, seconds: u64, micro_seconds: u32, amplitude: f32) -> Reverb<T> {
        let reverb = self.buffered().get().reverb(
            std::time::Duration::new(seconds, micro_seconds * 1000),
            amplitude,
        );

        RawClip::new(reverb)
    }

    pub fn periodic_access<F>(
        self,
        seconds: u64,
        micro_seconds: u32,
        access: F,
    ) -> RawClip<rodio::source::PeriodicAccess<T, F>>
    where
        F: FnMut(&mut T), {
        let access = self.get().periodic_access(
            std::time::Duration::new(seconds, micro_seconds * 1000),
            access,
        );

        RawClip::new(access)
    }

    pub fn repeat(self) -> RawClip<rodio::source::Repeat<T>> {
        RawClip::new(self.get().repeat_infinite())
    }
}
