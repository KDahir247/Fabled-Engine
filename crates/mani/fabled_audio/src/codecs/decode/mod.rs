mod flac;
mod mpeg;
mod vorbis;
mod waveform;

pub use flac::*;
pub use mpeg::*;
pub use vorbis::*;
pub use waveform::*;

#[cfg(test)]
mod data_test {

    #[test]
    fn data_size() {}

    #[test]
    fn data_alignment() {}
}
