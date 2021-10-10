use bitflags::*;

bitflags! {
    pub struct FlacReaderOptions : u32{

        const METADATA_ONLY  = 2;
        const READ_VORBIS_COMMENT  = 4;
    }
}


impl From<FlacReaderOptions> for claxon::FlacReaderOptions {
    fn from(option_bit: FlacReaderOptions) -> Self {
        Self {
            metadata_only: option_bit.contains(FlacReaderOptions::METADATA_ONLY),
            read_vorbis_comment: option_bit.contains(FlacReaderOptions::READ_VORBIS_COMMENT),
        }
    }
}


#[cfg(test)]
mod flac_options_test {
    use crate::FlacReaderOptions;

    #[test]
    fn bit_set() {
        let flac_option_bit_1 = FlacReaderOptions::from_bits(2).unwrap();
        let flac_option_bit_2 = FlacReaderOptions::from_bits(4).unwrap();
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
