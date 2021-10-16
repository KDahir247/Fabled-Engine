use crate::{SampleFormat, SupportedBufferSize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeviceConfig {
    pub sample_rate: u32,
    pub channel_count: u16,
    pub buffer_size: SupportedBufferSize,
    pub sample_format: SampleFormat,
}
