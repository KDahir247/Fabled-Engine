use crate::{AudioDecodingError, AudioSpecification, SampleFormat};

#[derive(Default)]
pub struct OggReader;

// todo not sure how to retrieve the duration.

impl OggReader {
    pub fn read_ogg<P: AsRef<std::path::Path>>(
        &self,
        ogg_path: P,
    ) -> Result<AudioSpecification, AudioDecodingError> {
        let file = std::fs::File::open(ogg_path)?;

        let packet_reader = ogg::PacketReader::new(file);

        let reader = lewton::inside_ogg::OggStreamReader::from_ogg_reader(packet_reader)
            .map_err(AudioDecodingError::OggError)?;

        let indent_info = reader.ident_hdr;

        println!("{:?}", indent_info.bitrate_nominal);

        Ok(AudioSpecification {
            channel_count: indent_info.audio_channels as u16,
            sample_rate: indent_info.audio_sample_rate,
            bit_per_sample: 24,
            // rodio current_data is of type i16
            sample_format: SampleFormat::I16,
            duration: 0,
        })
    }
}


#[cfg(test)]
mod ogg_decoder_test {
    use crate::OggReader;
    use rodio::Source;

    #[test]
    fn decode_file() {
        let ogg_path = [env!("CARGO_MANIFEST_DIR"), "/src/audio/Deus Ex Tempus.ogg"].join("");

        let ogg_reader = OggReader::default();
        let audio_spec = ogg_reader.read_ogg(ogg_path).unwrap();

        println!("{:?}", audio_spec);
    }


    #[test]
    fn compare_with_rodio() {
        let ogg_path = [env!("CARGO_MANIFEST_DIR"), "/src/audio/Deus Ex Tempus.ogg"].join("");

        let file = std::fs::File::open(ogg_path.as_str()).unwrap();

        let ogg_reader = OggReader::default();
        let rodio_decoder = rodio::Decoder::new_vorbis(file).unwrap();
        let audio_spec = ogg_reader.read_ogg(ogg_path.as_str()).unwrap();

        assert_eq!(audio_spec.channel_count, rodio_decoder.channels());
        assert_eq!(audio_spec.sample_rate, rodio_decoder.sample_rate());

        println!("{:?}", rodio_decoder.total_duration());
    }
}
