#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SampleFormat {
    I16,

    U16,

    F32,
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
