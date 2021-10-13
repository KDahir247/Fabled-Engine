use crate::{SampleFormat, SupportedBufferSize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeviceConfig {
    pub sample_rate: u32,
    pub channel_count: u16,
    pub sample_format: SampleFormat,
    pub buffer_size: SupportedBufferSize,
}
