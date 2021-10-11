use crate::{AudioDecodingError, AudioSpecification, SampleFormat};

#[derive(Default)]
pub struct Mp3Reader;

impl Mp3Reader {
    pub fn read_mp3<P: AsRef<std::path::Path>>(
        &self,
        mp3_path: P,
    ) -> Result<AudioSpecification, AudioDecodingError> {
        let path_dir = mp3_path
            .as_ref()
            .to_str()
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::InvalidData))?;

        let file = std::fs::File::open(path_dir)?;
        let buf_reader = std::io::BufReader::new(file);
        let mut reader = minimp3::Decoder::new(buf_reader);

        let mp3_duration = mp3_duration::from_path(path_dir).unwrap_or_default();

        // All the frame should have the same sample rate and channel
        let frame_detail = reader.next_frame().map_err(AudioDecodingError::Mp3Error)?;

        Ok(AudioSpecification {
            channel_count: frame_detail.channels as u16,
            sample_rate: frame_detail.sample_rate as u32,
            // Internally, MP3 uses Huffman symbols to store the processed audio data. As such,
            // there's no real "bit depth" to report.
            bit_per_sample: 24,
            // since frame_detail.data is a collection of i16.
            sample_format: SampleFormat::I16,
            duration: mp3_duration.as_secs() as u32,
        })
    }
}


#[cfg(test)]
mod mp3_decoding_test {
    use crate::Mp3Reader;
    use rodio::Source;

    #[test]
    fn decoding_file() {
        let mp3_path = [env!("CARGO_MANIFEST_DIR"), "/src/audio/epic.mp3"].join("");

        let mp3_reader = Mp3Reader::default();
        let mp3_spec = mp3_reader.read_mp3(mp3_path).unwrap();

        println!("{:?}", mp3_spec);
    }

    #[test]
    fn compare_with_rodio() {
        let mp3_path = [env!("CARGO_MANIFEST_DIR"), "/src/audio/epic.mp3"].join("");

        let file = std::fs::File::open(mp3_path.as_str()).unwrap();

        let rodio_decoder = rodio::Decoder::new_mp3(file).unwrap();
        let mp3_reader = Mp3Reader::default();
        let audio_spec = mp3_reader.read_mp3(mp3_path.as_str()).unwrap();

        assert_eq!(audio_spec.channel_count, rodio_decoder.channels());
        assert_eq!(audio_spec.sample_rate, rodio_decoder.sample_rate());

        // duration isn't calculated in rodio for mp3 and return None.
    }
}
