use crate::{AudioDecodingError, AudioSpecification, SampleFormat};

#[derive(Default)]
pub struct WavReader;

impl WavReader {
    pub fn read_wav<P: AsRef<std::path::Path>>(
        &self,
        wav_path: P,
    ) -> Result<AudioSpecification, AudioDecodingError> {
        let path_dir = wav_path.as_ref().to_str().unwrap();

        {
            let mut file = std::fs::File::open(path_dir)?;

            hound::read_wave_header(&mut file).map_err(AudioDecodingError::WavError)?;
        }

        let file = std::fs::File::open(path_dir)?;

        let wav_reader = hound::WavReader::new(file).map_err(AudioDecodingError::WavError)?;

        let wav_spec = wav_reader.spec();

        let format_target = match wav_spec.sample_format {
            hound::SampleFormat::Float => SampleFormat::F32,
            hound::SampleFormat::Int => SampleFormat::I16,
        };

        Ok(AudioSpecification {
            channel_count: wav_spec.channels,
            sample_rate: wav_spec.sample_rate,
            bit_per_sample: wav_spec.bits_per_sample,
            sample_format: format_target,
            duration: (wav_reader.duration() as f32 / wav_spec.sample_rate as f32),
        })
    }
}


#[cfg(test)]
mod wav_decoding_test {
    use crate::WavReader;
    use rodio::Source;

    #[test]
    fn decoding_file() {
        let wav_path = [env!("CARGO_MANIFEST_DIR"), "/src/audio/recorded.wav"].join("");

        let wav_reader = WavReader::default();
        let wav_spec = wav_reader.read_wav(wav_path).unwrap();

        println!("{:?}", wav_spec);
    }

    #[test]
    fn compare_with_rodio() {
        let threshold = 0.00001;

        let wav_path = [env!("CARGO_MANIFEST_DIR"), "/src/audio/recorded.wav"].join("");

        let file = std::fs::File::open(wav_path.as_str()).unwrap();

        let rodio_decoder = rodio::Decoder::new_wav(file).unwrap();
        let wav_reader = WavReader::default();
        let audio_spec = wav_reader.read_wav(wav_path.as_str()).unwrap();

        assert_eq!(rodio_decoder.channels(), audio_spec.channel_count);
        assert_eq!(rodio_decoder.sample_rate(), audio_spec.sample_rate);
        assert!(
            (rodio_decoder.total_duration().unwrap().as_secs_f32() - audio_spec.duration)
                .abs()
                .le(&threshold)
        );
    }
}
