use crate::SampleFormat;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioSpecification {
    pub sample_rate: u32,
    pub duration: f32,
    pub channel_count: u16,
    pub bit_per_sample: u16,
    pub sample_format: SampleFormat,
}

impl Default for AudioSpecification {
    fn default() -> Self {
        Self {
            channel_count: 2,
            sample_rate: 0,
            bit_per_sample: 32,
            sample_format: Default::default(),
            duration: 0.0,
        }
    }
}
