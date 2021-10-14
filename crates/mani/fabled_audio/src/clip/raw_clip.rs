pub struct RawClip<T>
where
    T: rodio::Sample, {
    pub data: Box<dyn rodio::Source<Item = T> + Send>,
}

impl<T> RawClip<T>
where
    T: rodio::Sample,
{
    pub fn new<U: 'static>(data: U) -> Self
    where
        U: rodio::Source<Item = T> + Send, {
        Self {
            data: Box::new(data),
        }
    }
}

pub struct RawAmbisonicClip {
    pub data: Box<dyn ambisonic::rodio::Source<Item = f32> + Send>,
}


impl RawAmbisonicClip {
    pub fn new<T: 'static>(data: T) -> Self
    where
        T: ambisonic::rodio::Source<Item = f32> + Send, {
        Self {
            data: Box::new(data),
        }
    }
}


#[cfg(test)]
mod raw_test {
    use crate::{AudioClip, AudioType, RawAmbisonicClip};
    use ambisonic::rodio::Source;
    use std::io::Read;

    #[test]
    fn get_standard_inner() {
        let path = &[env!("CARGO_MANIFEST_DIR"), "/src/audio/epic1.mp3"].join("");

        let file = std::fs::File::open(path).unwrap();


        let clip: AudioClip<f32> = AudioClip::from_file(AudioType::Loose(file), true);

        let previous_channel = clip.channels();
        let previous_sample_rate = clip.sample_rate();
        let previous_total_duration = clip.total_duration();
        let previous_curr_len = clip.current_frame_len();

        let raw_clip = RawAmbisonicClip::from(clip);

        let inner = raw_clip.data;

        assert!(previous_channel.eq(&inner.channels()));
        assert!(previous_sample_rate.eq(&inner.sample_rate()));
        assert!(previous_total_duration.eq(&inner.total_duration()));
        assert!(previous_curr_len.eq(&inner.current_frame_len()));
    }


    #[test]
    fn get_ambisonic_inner() {
        let path = &[env!("CARGO_MANIFEST_DIR"), "/src/audio/epic1.mp3"].join("");

        let file = std::fs::File::open(path).unwrap();

        let clip: AudioClip<f32> = AudioClip::from_file(AudioType::Loose(file), true);

        let previous_channel = clip.channels();
        let previous_sample_rate = clip.sample_rate();
        let previous_total_duration = clip.total_duration();
        let previous_curr_len = clip.current_frame_len();

        let raw_ambisonic = RawAmbisonicClip::from(clip);

        let inner = raw_ambisonic.data;

        assert!(inner.channels().eq(&previous_channel));
        assert!(inner.sample_rate().eq(&previous_sample_rate));
        assert!(inner.total_duration().eq(&previous_total_duration));
        assert!(inner.current_frame_len().eq(&previous_curr_len));
    }
}
