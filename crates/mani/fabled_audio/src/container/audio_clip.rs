use ambisonic::rodio::buffer::SamplesBuffer;
use ambisonic::rodio::source::SamplesConverter;
use ambisonic::rodio::Source as AmbientSource;
use rodio::Source;

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
            channel: 1,
            sample: 1,
        }
    }
}

impl AudioClip {
    pub fn new(buffer: Vec<u8>) -> Self {
        let decoder = rodio::Decoder::new(std::io::Cursor::new(buffer)).unwrap();

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

    pub fn to_buffer(&self) -> rodio::buffer::SamplesBuffer<i16> {
        let data = self.data.read().unwrap();
        rodio::buffer::SamplesBuffer::new(self.channel, self.sample, data.to_vec())
    }

    pub fn to_ambisonic_buffer(&self) -> SamplesConverter<SamplesBuffer<i16>, f32> {
        let data = self.data.read().unwrap();
        ambisonic::rodio::buffer::SamplesBuffer::new(self.channel, self.sample, data.to_vec())
            .convert_samples::<f32>()
    }
}
