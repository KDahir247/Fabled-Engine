pub struct RawClip<T>
where
    T: rodio::Source,
    T::Item: rodio::Sample, {
    data: T,
}

impl<T> AsRef<T> for RawClip<T>
where
    T: rodio::Source,
    T::Item: rodio::Sample,
{
    fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<T> RawClip<T>
where
    T: rodio::Source,
    T::Item: rodio::Sample,
{
    pub fn new(data: T) -> Self {
        Self { data }
    }

    pub fn get(self) -> T {
        self.data
    }
}

pub struct RawAmbisonicClip<T>
where
    T: ambisonic::rodio::Source<Item = f32>, {
    data: T,
}

impl<T> AsRef<T> for RawAmbisonicClip<T>
where
    T: ambisonic::rodio::Source<Item = f32>,
{
    fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<T> RawAmbisonicClip<T>
where
    T: ambisonic::rodio::Source<Item = f32>,
{
    pub fn new(data: T) -> Self {
        Self { data }
    }

    pub fn get(self) -> T {
        self.data
    }
}


#[cfg(test)]
mod raw_clip_test {
    use crate::{AudioClip, Standard};
    use ambisonic::rodio::Source;
    use std::io::Read;

    #[test]
    fn get_inner() {
        let path = &[env!("CARGO_MANIFEST_DIR"), "/src/audio/epic1.mp3"].join("");

        let mut file = std::fs::File::open(path).unwrap();
        let mut audio_buffer = vec![0; file.metadata().unwrap().len() as usize];
        file.read_exact(&mut audio_buffer).unwrap();


        let clip: AudioClip<f32> = AudioClip::from_file(audio_buffer, true);

        let previous_channel = clip.channels();
        let previous_sample_rate = clip.sample_rate();
        let previous_total_duration = clip.total_duration();
        let previous_curr_len = clip.current_frame_len();

        let raw_clip = Standard::from(clip);

        let _ref_inner = raw_clip.as_ref();

        let inner = raw_clip.get();

        assert!(previous_channel.eq(&inner.channels()));
        assert!(previous_sample_rate.eq(&inner.sample_rate()));
        assert!(previous_total_duration.eq(&inner.total_duration()));
        assert!(previous_curr_len.eq(&inner.current_frame_len()));
    }
}


#[cfg(test)]
mod raw_ambisonic_clip_test {
    use crate::{Ambisonic, AudioClip};
    use ambisonic::rodio::Source;
    use std::io::Read;

    #[test]
    fn get_inner() {
        let path = &[env!("CARGO_MANIFEST_DIR"), "/src/audio/epic1.mp3"].join("");

        let mut file = std::fs::File::open(path).unwrap();
        let mut audio_buffer = vec![0; file.metadata().unwrap().len() as usize];
        file.read_exact(&mut audio_buffer).unwrap();

        let clip: AudioClip<f32> = AudioClip::from_file(audio_buffer, true);

        let previous_channel = clip.channels();
        let previous_sample_rate = clip.sample_rate();
        let previous_total_duration = clip.total_duration();
        let previous_curr_len = clip.current_frame_len();

        let raw_ambisonic = Ambisonic::from(clip);

        let _ref_inner = raw_ambisonic.as_ref();

        let inner = raw_ambisonic.get();

        assert!(inner.channels().eq(&previous_channel));
        assert!(inner.sample_rate().eq(&previous_sample_rate));
        assert!(inner.total_duration().eq(&previous_total_duration));
        assert!(inner.current_frame_len().eq(&previous_curr_len));
    }
}
