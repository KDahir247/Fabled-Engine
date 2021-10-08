mod wav;
pub use wav::*;


#[cfg(test)]
mod data_test {
    use crate::WavSpecification;

    #[test]
    fn data_size() {
        let wav_spec_size = std::mem::size_of::<WavSpecification>();
        assert_eq!(wav_spec_size & (wav_spec_size - 1), 0);
    }

    #[test]
    fn data_alignment() {
        let wav_spec_alignment = std::mem::align_of::<WavSpecification>();
        assert_eq!(wav_spec_alignment & (wav_spec_alignment - 1), 0);
    }
}
