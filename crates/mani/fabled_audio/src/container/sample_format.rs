#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SampleFormat {
    I16,
    U16,
    F32,
}

impl Default for SampleFormat {
    fn default() -> Self {
        Self::I16
    }
}

impl From<cpal::SampleFormat> for SampleFormat {
    fn from(cpal_format: cpal::SampleFormat) -> Self {
        match cpal_format {
            cpal::SampleFormat::I16 => Self::I16,
            cpal::SampleFormat::U16 => Self::U16,
            cpal::SampleFormat::F32 => Self::F32,
        }
    }
}

impl From<SampleFormat> for hound::SampleFormat {
    fn from(format: SampleFormat) -> Self {
        match format {
            SampleFormat::I16 | SampleFormat::U16 => hound::SampleFormat::Int,
            SampleFormat::F32 => hound::SampleFormat::Float,
        }
    }
}

impl SampleFormat {
    pub fn sample_size(&self) -> usize {
        match self {
            SampleFormat::I16 => std::mem::size_of::<i16>(),
            SampleFormat::U16 => std::mem::size_of::<u16>(),
            SampleFormat::F32 => std::mem::size_of::<f32>(),
        }
    }
}
