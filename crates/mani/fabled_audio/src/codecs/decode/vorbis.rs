use crate::{AudioDecodingError, AudioSpecification};

#[derive(Default)]
pub struct OggReader;


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

        // todo not sure how to acquire bit_per_sample, sample_format, and duration.
        Ok(AudioSpecification {
            channel_count: indent_info.audio_channels as u16,
            sample_rate: indent_info.audio_sample_rate,
            bit_per_sample: 0,
            sample_format: Default::default(),
            duration: 0,
        })
    }
}


#[cfg(test)]
mod ogg_decoder_test {
    use crate::OggReader;

    #[test]
    fn decode_file() {
        let ogg_reader = OggReader::default();

        let ogg_path = [env!("CARGO_MANIFEST_DIR"), "/src/audio/Deus Ex Tempus.ogg"].join("");

        let audio_spec = ogg_reader.read_ogg(ogg_path).unwrap();

        println!("{:?}", audio_spec);
    }
}
