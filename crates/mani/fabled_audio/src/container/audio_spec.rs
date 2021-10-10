use crate::SampleFormat;

#[derive(Clone, Debug)]
pub struct AudioSpecification {
    pub channel_count: u16,

    pub sample_rate: u32,

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
        }
    }
}
