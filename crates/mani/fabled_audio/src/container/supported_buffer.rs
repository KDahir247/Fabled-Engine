#[derive(Copy, Debug, Clone, PartialEq, Eq)]
pub enum SupportedBufferSize {
    Default,
    Fixed(u32),
}


impl Default for SupportedBufferSize {
    fn default() -> Self {
        Self::Default
    }
}

impl From<&cpal::SupportedBufferSize> for SupportedBufferSize {
    fn from(supported_buffer: &cpal::SupportedBufferSize) -> Self {
        match supported_buffer {
            cpal::SupportedBufferSize::Range { min: _, max } => Self::Fixed(*max),
            cpal::SupportedBufferSize::Unknown => Self::Default,
        }
    }
}

impl From<SupportedBufferSize> for cpal::BufferSize {
    fn from(supported_buffer: SupportedBufferSize) -> Self {
        match supported_buffer {
            SupportedBufferSize::Default => Self::Default,
            SupportedBufferSize::Fixed(frame_count) => Self::Fixed(frame_count),
        }
    }
}
