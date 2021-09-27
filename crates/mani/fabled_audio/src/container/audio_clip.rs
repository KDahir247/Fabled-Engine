use crate::{RawAmbisonicClip, RawClip};
use ambisonic::rodio::Source as AmbientSource;
use rodio::Source;


// Rodio spawns a background thread that is dedicated to reading from the
// sources and sending the output to the device and I want user to have control
// over customizing the audio clip data I will wrap it in a RwLock. Might have
// to create a separate thread for audio since I dont want RwLock to block the
// main thread which will handle core logic for read when write is happening,
// but a dedicated thread.

pub type Standard = RawClip<rodio::buffer::SamplesBuffer<i16>>;
pub type Ambisonic = RawAmbisonicClip<
    ambisonic::rodio::source::SamplesConverter<ambisonic::rodio::buffer::SamplesBuffer<i16>, f32>,
>;

#[derive(Debug)]
pub struct AudioClip {
    pub data: std::sync::RwLock<std::sync::Arc<[i16]>>,
    pub channel: u16,
    pub sample: u32,
}

impl Default for AudioClip {
    fn default() -> Self {
        Self {
            data: std::sync::RwLock::new(vec![].into()),
            channel: 0,
            sample: 0,
        }
    }
}

impl AudioClip {
    pub fn from_file(buffer: Vec<u8>) -> Self {
        let decoder = rodio::Decoder::new(std::io::Cursor::new(buffer)).unwrap();


        // We should be able to guarantee that the unwrap will not be None
        let channel = decoder.channels();
        let sample = decoder.sample_rate();
        let data = decoder.collect::<Vec<_>>();

        Self {
            data: std::sync::RwLock::new(data.into()),
            channel,
            sample,
        }
    }

    pub fn create_clip(data: Vec<i16>, channel: u16, sample: u32) -> AudioClip {
        Self {
            data: std::sync::RwLock::new(data.into()),
            channel,
            sample,
        }
    }
}

// Standard clip
impl From<AudioClip> for Standard {
    fn from(clip: AudioClip) -> Self {
        // blocks
        let data = clip.data.read().unwrap();

        let sample_buffer =
            rodio::buffer::SamplesBuffer::new(clip.channel, clip.sample, data.to_vec());

        RawClip::new(sample_buffer)
    }
}

// Ambisonic clip
impl From<AudioClip> for Ambisonic {
    fn from(clip: AudioClip) -> Self {
        // blocks
        let data = clip.data.read().unwrap();

        let sample_buffer =
            ambisonic::rodio::buffer::SamplesBuffer::new(clip.channel, clip.sample, data.to_vec())
                .convert_samples::<f32>();

        RawAmbisonicClip::new(sample_buffer)
    }
}
