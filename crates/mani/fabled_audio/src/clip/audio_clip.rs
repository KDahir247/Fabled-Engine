use crate::{AudioDecodingError, RawAmbisonicClip, RawClip};
use rodio::Source;
use std::io::Read;

// A Rust object that represents a sound should implement the `Source` trait.

// We don't know if the data is f32, i16, or u16 and want to support current
// data types and future types deriving from rodio::Sample. Not just restrict it
// to one type.
#[derive(Debug)]
pub struct AudioClip<D> {
    pub audio_data: parking_lot::Mutex<std::vec::IntoIter<D>>,
    pub duration: Option<std::time::Duration>,
    pub current_frame_len: Option<usize>,
    pub sample: u32,
    pub channel: u16,
}

impl<D> Default for AudioClip<D> {
    fn default() -> Self {
        Self {
            audio_data: parking_lot::Mutex::new(vec![].into_iter()),
            channel: 0,
            sample: 0,
            duration: None,
            current_frame_len: None,
        }
    }
}

impl<D> AudioClip<D>
where
    D: rodio::Sample,
{
    pub fn from_raw(buffer: Vec<u8>, play_on_awake: bool) -> Result<Self, AudioDecodingError> {
        let audio_decoder = rodio::Decoder::new(std::io::Cursor::new(buffer))
            .map_err(AudioDecodingError::DecoderError)?;

        let audio_clip = audio_decoder
            .pausable(!play_on_awake)
            .stoppable()
            .convert_samples::<D>();

        Ok(Self {
            channel: audio_clip.channels(),
            sample: audio_clip.sample_rate(),
            duration: audio_clip.total_duration(),
            current_frame_len: audio_clip.current_frame_len(),
            audio_data: parking_lot::Mutex::new(audio_clip.collect::<Vec<_>>().into_iter()),
        })
    }

    pub fn from_file(file: std::fs::File, play_on_awake: bool) -> Result<Self, AudioDecodingError> {
        let mut file = file;

        let meta_data = file.metadata()?;

        let mut audio_buffer = vec![0; meta_data.len() as usize];

        file.read_exact(&mut audio_buffer)?;

        let audio_decoder = rodio::Decoder::new(std::io::Cursor::new(audio_buffer))
            .map_err(AudioDecodingError::DecoderError)?;

        let audio_clip = audio_decoder
            .pausable(!play_on_awake)
            .stoppable()
            .convert_samples::<D>();


        Ok(Self {
            channel: audio_clip.channels(),
            sample: audio_clip.sample_rate(),
            duration: audio_clip.total_duration(),
            current_frame_len: audio_clip.current_frame_len(),
            audio_data: parking_lot::Mutex::new(audio_clip.collect::<Vec<_>>().into_iter()),
        })
    }


    pub fn create_clip(
        data: Vec<D>,
        channel: u16,
        sample: u32,
        duration: Option<std::time::Duration>,
        play_on_awake: bool,
    ) -> AudioClip<D> {
        let audio_clip = Self {
            audio_data: parking_lot::Mutex::new(data.into_iter()),
            channel,
            sample,
            duration,
            current_frame_len: None,
        }
        .pausable(!play_on_awake)
        .stoppable();

        Self {
            channel: audio_clip.channels(),
            sample: audio_clip.sample_rate(),
            duration: audio_clip.total_duration(),
            current_frame_len: audio_clip.current_frame_len(),
            audio_data: parking_lot::Mutex::new(
                audio_clip.convert_samples().collect::<Vec<_>>().into_iter(),
            ),
        }
    }
}

impl<D: 'static> From<AudioClip<D>> for RawClip<D>
where
    D: rodio::Sample + Send,
{
    fn from(audio_clip: AudioClip<D>) -> Self {
        RawClip::new(audio_clip)
    }
}

impl From<AudioClip<f32>> for RawAmbisonicClip {
    fn from(clip: AudioClip<f32>) -> Self {
        RawAmbisonicClip::new(clip)
    }
}


impl<D> Iterator for AudioClip<D> {
    type Item = D;

    fn next(&mut self) -> Option<Self::Item> {
        let mut lock = self.audio_data.lock();
        lock.next()
    }
}

impl<D> rodio::Source for AudioClip<D>
where
    D: rodio::Sample,
{
    fn current_frame_len(&self) -> Option<usize> {
        self.current_frame_len
    }

    fn channels(&self) -> u16 {
        self.channel
    }

    fn sample_rate(&self) -> u32 {
        self.sample
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.duration
    }
}


impl<D> ambisonic::rodio::Source for AudioClip<D>
where
    D: ambisonic::rodio::Sample,
{
    fn current_frame_len(&self) -> Option<usize> {
        self.current_frame_len
    }

    fn channels(&self) -> u16 {
        self.channel
    }

    fn sample_rate(&self) -> u32 {
        self.sample
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.duration
    }
}


#[cfg(test)]
mod audio_clip_test {
    use crate::{AudioClip, RawAmbisonicClip, RawClip};
    use ambisonic::rodio::Source;
    use std::io::Read;

    #[test]
    fn default_clip_test() {
        let mut empty_clip: AudioClip<u16> = AudioClip::default();

        assert!(empty_clip.sample.eq(&0));
        assert!(rodio::Source::sample_rate(&empty_clip).eq(&0));
        assert!(ambisonic::rodio::Source::sample_rate(&empty_clip).eq(&0));

        assert!(empty_clip.duration.is_none());
        assert!(rodio::Source::total_duration(&empty_clip).is_none());
        assert!(ambisonic::rodio::Source::total_duration(&empty_clip).is_none());

        assert!(empty_clip.channel.eq(&0));
        assert!(rodio::Source::channels(&empty_clip).eq(&0));
        assert!(ambisonic::rodio::Source::channels(&empty_clip).eq(&0));

        assert!(empty_clip.current_frame_len.is_none());
        assert!(rodio::Source::current_frame_len(&empty_clip).is_none());
        assert!(ambisonic::rodio::Source::current_frame_len(&empty_clip).is_none());

        assert!(empty_clip.next().is_none());

        let lock = empty_clip.audio_data.lock();
        assert!(lock.len().eq(&0));
    }

    #[test]
    fn create_clip_test() {
        let buffer = vec![5.0; 10];
        let channel = 2;
        let sample_rate = 4800;
        let duration = None;

        let mut custom_clip =
            AudioClip::create_clip(buffer.clone(), channel, sample_rate, duration, true);

        assert!(custom_clip.channels().eq(&channel));
        assert!(custom_clip.sample_rate().eq(&sample_rate));
        assert!(custom_clip.total_duration().is_none());
        assert!(custom_clip.current_frame_len().is_none());

        for _ in 0..buffer.len() {
            let elem = custom_clip.next();
            assert!(elem.eq(&Some(5.0)));
        }

        assert!(custom_clip.next().is_none());
    }


    #[test]
    fn load_clip_from_file() {
        let path = &[env!("CARGO_MANIFEST_DIR"), "/src/audio/epic1.mp3"].join("");

        let mut file = std::fs::File::open(path).unwrap();

        let mut audio_buffer = vec![0; file.metadata().unwrap().len() as usize];
        file.read_exact(&mut audio_buffer).unwrap();


        let audio_clip: AudioClip<f32> = AudioClip::from_raw(audio_buffer, true).unwrap();

        assert!(audio_clip.channels().gt(&0));
        assert!(audio_clip.sample_rate().gt(&0));
        assert!(audio_clip.total_duration().is_none()); // unknown duration
        assert!(audio_clip.current_frame_len().is_some());
    }

    #[test]
    fn convert_clip_to_standard_test() {
        let audio_clip: AudioClip<f32> = AudioClip::default();

        let _audio = RawClip::new(audio_clip);
    }

    #[test]
    fn convert_clip_to_ambisonic_test() {
        let audio_clip: AudioClip<f32> = AudioClip::default();

        let _ambisonic_audio = RawAmbisonicClip::new(audio_clip);
    }
}
