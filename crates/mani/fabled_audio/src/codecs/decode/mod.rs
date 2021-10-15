mod flac;
mod mpeg;
mod vorbis;
mod waveform;

pub use flac::*;
pub use mpeg::*;
pub use vorbis::*;
pub use waveform::*;

use bitflags::bitflags;

bitflags! {
    pub struct FlacReaderOptions : u8{

        const METADATA_ONLY  = 2;
        const READ_VORBIS_COMMENT  = 4;
    }
}


impl From<FlacReaderOptions> for claxon::FlacReaderOptions {
    fn from(option_bit: FlacReaderOptions) -> Self {
        let option_bit = option_bit.bits;

        let meta_data_bit = FlacReaderOptions::METADATA_ONLY.bits;

        let vorbis_comment_bit = FlacReaderOptions::READ_VORBIS_COMMENT.bits;

        Self {
            metadata_only: (option_bit & meta_data_bit) == 2,
            read_vorbis_comment: (option_bit & vorbis_comment_bit) == 4,
        }
    }
}


#[cfg(test)]
mod flac_options_test {
    use crate::FlacReaderOptions;

    #[test]
    fn bit_set() {
        let flac_option_bit_1 = FlacReaderOptions::from_bits(2).unwrap();
        assert_eq!(flac_option_bit_1, FlacReaderOptions::METADATA_ONLY);

        let flac_option_bit_2 = FlacReaderOptions::from_bits(4).unwrap();
        assert_eq!(flac_option_bit_2, FlacReaderOptions::READ_VORBIS_COMMENT);

        let flac_option_bit_3 = FlacReaderOptions::from_bits(6).unwrap();
        assert!(flac_option_bit_3.is_all());
    }


    #[test]
    fn bit_conversion() {
        let flac_empty_bit = FlacReaderOptions::empty();
        let flac_empty_option: claxon::FlacReaderOptions = flac_empty_bit.into();

        assert!(!flac_empty_option.read_vorbis_comment);
        assert!(!flac_empty_option.metadata_only);

        let flac_meta_bit = FlacReaderOptions::from_bits(2).unwrap();
        let flac_meta_option: claxon::FlacReaderOptions = flac_meta_bit.into();

        assert!(!flac_meta_option.read_vorbis_comment);
        assert!(flac_meta_option.metadata_only);

        let flac_read_bit = FlacReaderOptions::from_bits(4).unwrap();
        let flac_read_option: claxon::FlacReaderOptions = flac_read_bit.into();

        assert!(flac_read_option.read_vorbis_comment);
        assert!(!flac_read_option.metadata_only);

        let flac_all_bit = FlacReaderOptions::from_bits(6).unwrap();
        let flac_all_option: claxon::FlacReaderOptions = flac_all_bit.into();

        assert!(flac_all_option.read_vorbis_comment);
        assert!(flac_all_option.metadata_only);
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
