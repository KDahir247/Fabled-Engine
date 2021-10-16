mod decode;
mod encode;

pub use decode::*;
pub use encode::*;

#[cfg(test)]
mod data_test {
    use crate::WavWriter;

    #[test]
    fn data_size() {
        let encode_waveform_size = std::mem::size_of::<WavWriter>();
        println!("{}", encode_waveform_size);
    }

    #[test]
    fn data_alignment() {
        let encode_wavform_alignment = std::mem::align_of::<WavWriter>();
        assert_eq!(encode_wavform_alignment & (encode_wavform_alignment - 1), 0);
    }
}
