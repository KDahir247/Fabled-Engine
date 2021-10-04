use crate::{RawAmbisonicClip, RawClip};
use rodio::Source;
use std::time::Duration;

// Rodio spawns a background thread that is dedicated to reading from the
// sources and sending the output to the device and I want user to have control
// over customizing the audio clip data I will wrap it in a Mutex. Might have
// to create a separate thread for audio since I dont want Mutex to block the
// main thread which will handle core logic for read when write is happening,
// but a dedicated thread.

// Mutex will be faster if you mostly only ever have one reader at a time.
// There will only be on reader for this type since it is moved to a raw_clip
// type. and from there it is moved to an audio output and played.
// writing can still happen after reading though so we will lock this if it is
// reading or writing and treat reader and writer the same and lock on either
// case. Mutex also support more 3rd tier support and prevent writer starvation.

// A Rust object that represents a sound should implement the `Source` trait.

pub type Standard<D> = RawClip<AudioClip<D>>;
pub type Ambisonic<D> = RawAmbisonicClip<AudioClip<D>>;


// We don't know if the data is f32, i16, or u16 and want to support current
// data types and future types deriving from rodio::Sample. Not just restrict it
// to one type.
#[derive(Debug)]
pub struct AudioClip<D> {
    pub data: parking_lot::Mutex<std::vec::IntoIter<D>>,
    pub channel: u16,
    pub sample: u32,
    pub duration: Option<std::time::Duration>,
    pub current_frame_len: Option<usize>,
}

impl<D> Default for AudioClip<D> {
    fn default() -> Self {
        Self {
            data: parking_lot::Mutex::new(vec![].into_iter()),
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
    pub fn from_file(buffer: Vec<u8>, play_on_awake: bool) -> Self {
        // todo better way to read file. should I pass a buffer?
        let decoder = rodio::Decoder::new(std::io::Cursor::new(buffer));

        match decoder {
            Ok(source) => {
                let source = source
                    .pausable(!play_on_awake)
                    .stoppable()
                    .convert_samples::<D>(); // todo maybe remove this and convert to float in Iterator next.

                Self {
                    channel: source.channels(),
                    sample: source.sample_rate(),
                    duration: source.total_duration(),
                    current_frame_len: source.current_frame_len(),
                    data: parking_lot::Mutex::new(source.collect::<Vec<_>>().into_iter()),
                }
            }
            Err(err) => {
                println!("Error from decoding audio file.\nMessage : {:?}", err);
                AudioClip::default()
            }
        }
    }

    pub fn create_clip(
        data: Vec<D>,
        channel: u16,
        sample: u32,
        duration: Option<std::time::Duration>,
        play_on_awake: bool,
    ) -> AudioClip<D> {
        let current = Self {
            data: parking_lot::Mutex::new(data.into_iter()),
            channel,
            sample,
            duration,
            current_frame_len: None,
        }
        .pausable(!play_on_awake)
        .stoppable();


        Self {
            channel: current.channels(),
            sample: current.sample_rate(),
            duration: current.total_duration(),
            current_frame_len: current.current_frame_len(),
            data: parking_lot::Mutex::new(
                current.convert_samples().collect::<Vec<_>>().into_iter(),
            ),
        }
    }
}

// Standard clip
impl<D> From<AudioClip<D>> for Standard<D>
where
    D: rodio::Sample,
{
    fn from(clip: AudioClip<D>) -> Self {
        RawClip::new(clip)
    }
}

// Ambisonic clip
impl<D> From<AudioClip<D>> for Ambisonic<D>
where
    D: ambisonic::rodio::Sample,
{
    fn from(clip: AudioClip<D>) -> Self {
        RawAmbisonicClip::new(clip)
    }
}

impl<D> Iterator for AudioClip<D> {
    type Item = D;

    fn next(&mut self) -> Option<Self::Item> {
        // if data is getting changed block the next audio till change completed.
        // rather then returning a zero and play the newly changed value/s
        let mut lock = self.data.lock();
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

    fn total_duration(&self) -> Option<Duration> {
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

    fn total_duration(&self) -> Option<Duration> {
        self.duration
    }
}
