use crate::Clip;
use ambisonic::rodio::buffer::SamplesBuffer;
use ambisonic::rodio::source::SamplesConverter;
use ambisonic::rodio::Source as AmbientSource;
use rodio::Source;


// Rodio spawns a background thread that is dedicated to reading from the
// sources and sending the output to the device and I want user to have control
// over customizing the audio clip data I will wrap it in a RwLock. Might have
// to create a separate thread for audio since I dont want RwLock to block the
// main thread which will handle core logic for read when write is happening,
// but a dedicated thread.

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
        let sample_rate = decoder.sample_rate();
        let data = decoder.collect::<Vec<_>>();

        Self {
            data: std::sync::RwLock::new(data.into()),
            channel,
            sample: sample_rate,
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


impl Clip for AudioClip {
    fn to_buffer(&self) -> rodio::buffer::SamplesBuffer<i16> {
        // panic from poisoning or lock is already held by the current thread.

        let data = self.data.read().unwrap();

        rodio::buffer::SamplesBuffer::new(self.channel, self.sample, data.to_vec())
    }

    fn to_ambisonic_buffer(&self) -> SamplesConverter<SamplesBuffer<i16>, f32> {
        // panic from poisoning or lock is already held by the current thread.

        let data = self.data.read().unwrap();

        ambisonic::rodio::buffer::SamplesBuffer::new(self.channel, self.sample, data.to_vec())
            .convert_samples::<f32>()
    }
}
