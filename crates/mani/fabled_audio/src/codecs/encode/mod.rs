mod wav;

pub use wav::*;


#[cfg(test)]
mod data_test {
    use crate::WavSpecification;

    #[test]
    fn data_size() {
        let wav_spec_size = std::mem::size_of::<WavSpecification>();

        println!("{:?}", wav_spec_size);
    }

    #[test]
    fn data_alignment() {
        let wav_spec_alignment = std::mem::align_of::<WavSpecification>();

        println!("{:?}", wav_spec_alignment);
    }
}
