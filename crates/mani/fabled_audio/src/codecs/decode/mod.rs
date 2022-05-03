mod flac;
mod mpeg;
mod vorbis;
mod waveform;

pub use flac::*;
pub use mpeg::*;
pub use vorbis::*;
pub use waveform::*;


pub struct FlacReaderOptions(u8);

impl FlacReaderOptions {
    pub const METADATA_ONLY: FlacReaderOptions = FlacReaderOptions(2);
    pub const READ_VORBIS_COMMENT: FlacReaderOptions = FlacReaderOptions(4);
    pub const ALL: FlacReaderOptions = FlacReaderOptions(6);
}

impl FlacReaderOptions {
    pub fn inner(&self) -> &u8 {
        &self.0
    }
}

impl From<FlacReaderOptions> for claxon::FlacReaderOptions {
    fn from(option_bit: FlacReaderOptions) -> Self {
        let option_bit = option_bit.inner();

        Self {
            metadata_only: (option_bit & 2) == 2,
            read_vorbis_comment: (option_bit & 4) == 4,
        }
    }
}

#[cfg(test)]
mod data_test {
    use crate::FlacReaderOptions;

    #[test]
    fn data_size() {
        let flac_option_size = std::mem::size_of::<FlacReaderOptions>();
        assert_eq!(flac_option_size & (flac_option_size - 1), 0);
    }

    #[test]
    fn data_alignment() {
        let flac_option_alignment = std::mem::align_of::<FlacReaderOptions>();
        assert_eq!(flac_option_alignment & (flac_option_alignment - 1), 0);
    }
}
