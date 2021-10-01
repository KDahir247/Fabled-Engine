use crate::{RawAmbisonicClip, RawClip};
use rodio::Source;
use std::time::Duration;

// Rodio spawns a background thread that is dedicated to reading from the
// sources and sending the output to the device and I want user to have control
// over customizing the audio clip data I will wrap it in a RwLock. Might have
// to create a separate thread for audio since I dont want RwLock to block the
// main thread which will handle core logic for read when write is happening,
// but a dedicated thread.

// A Rust object that represents a sound should implement the `Source` trait.

pub type Standard = RawClip<AudioClip>;
pub type Ambisonic = RawAmbisonicClip<AudioClip>;

#[derive(Debug, Clone)]
pub struct AudioClip {
    // std::sync::RwLock<std::sync::Arc<??>>
    pub data: std::vec::IntoIter<f32>,
    pub channel: u16,
    pub sample: u32,
    pub duration: Option<std::time::Duration>,
    pub current_frame_len: Option<usize>,
}

impl Default for AudioClip {
    fn default() -> Self {
        Self {
            data: vec![].into_iter(),
            channel: 0,
            sample: 0,
            duration: None,
            current_frame_len: None,
        }
    }
}

impl AudioClip {
    pub fn from_file(buffer: Vec<u8>, play_on_awake: bool) -> Self {
        let decoder = rodio::Decoder::new(std::io::Cursor::new(buffer));

        match decoder {
            Ok(source) => {
                let source = source
                    .pausable(!play_on_awake)
                    .stoppable()
                    .convert_samples::<f32>();

                let channel = source.channels();
                let sample = source.sample_rate();
                let duration = source.total_duration();
                let current_frame_len = source.current_frame_len();
                let data = source.collect::<Vec<_>>().into_iter();

                Self {
                    channel,
                    sample,
                    duration,
                    current_frame_len,
                    data,
                }
            }
            Err(err) => {
                println!("Error from decoding audio file.\nMessage : {:?}", err);
                Self::default()
            }
        }
    }

    // todo can refactor and optimize this function.
    pub fn create_clip(
        data: Vec<f32>,
        channel: u16,
        sample: u32,
        duration: Option<std::time::Duration>,
        play_on_awake: bool,
    ) -> AudioClip {
        let current = Self {
            data: data.into_iter(),
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
            data: current.convert_samples().collect::<Vec<_>>().into_iter(),
        }
    }
}

// Standard clip
impl From<AudioClip> for Standard {
    fn from(source: AudioClip) -> Self {
        RawClip::new(source)
    }
}

// Ambisonic clip
impl From<AudioClip> for Ambisonic {
    fn from(clip: AudioClip) -> Self {
        RawAmbisonicClip::new(clip)
    }
}

impl Iterator for AudioClip {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next()
    }
}


impl rodio::Source for AudioClip {
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

impl ambisonic::rodio::Source for AudioClip {
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
