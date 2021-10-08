use crate::SampleFormat;

#[repr(align(8))]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct WavSpecification {
    pub channels: u16,
    pub sample_rate: u32,
    pub bit_per_sample: u16,
    pub sample_format: SampleFormat,
}

impl From<WavSpecification> for hound::WavSpec {
    fn from(wav_spec: WavSpecification) -> Self {
        hound::WavSpec {
            channels: wav_spec.channels,
            sample_rate: wav_spec.sample_rate,
            bits_per_sample: wav_spec.bit_per_sample,
            sample_format: wav_spec.sample_format.into(),
        }
    }
}
