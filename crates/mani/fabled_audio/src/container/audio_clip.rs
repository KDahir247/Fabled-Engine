use crate::{RawAmbisonicClip, RawClip};
use rodio::Source;
use std::time::Duration;

// Rodio spawns a background thread that is dedicated to reading from the
// sources and sending the output to the device and I want user to have control
// over customizing the audio clip data I will wrap it in a RwLock. Might have
// to create a separate thread for audio since I dont want RwLock to block the
// main thread which will handle core logic for read when write is happening,
// but a dedicated thread.

pub type Standard = RawClip<AudioClip>;
pub type Ambisonic = RawAmbisonicClip<AudioClip>;

#[derive(Debug, Clone)]
pub struct AudioClip {
    // std::sync::RwLock<std::sync::Arc<??>>
    pub data: std::vec::IntoIter<i16>,
    channel: u16,
    sample: u32,
    duration: Option<std::time::Duration>,
    current_frame_len: Option<usize>,
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
    pub fn from_file(buffer: Vec<u8>) -> Self {
        let decoder = rodio::Decoder::new(std::io::Cursor::new(buffer));

        match decoder {
            Ok(source) => Self {
                channel: source.channels(),
                sample: source.sample_rate(),
                duration: source.total_duration(),
                current_frame_len: source.current_frame_len(),
                data: source.collect::<Vec<_>>().into_iter(),
            },
            Err(err) => {
                println!("Error from decoding audio file.\nMessage : {:?}", err);
                Self::default()
            }
        }
    }

    pub fn create_clip(
        data: Vec<i16>,
        channel: u16,
        sample: u32,
        duration: Option<std::time::Duration>,
    ) -> AudioClip {
        Self {
            data: data.into_iter(),
            channel,
            sample,
            duration,
            current_frame_len: None,
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
        self.data.next().map(|x| cpal::Sample::to_f32(&x))
    }
}

// Standard support
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

// Ambisonic Support
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
