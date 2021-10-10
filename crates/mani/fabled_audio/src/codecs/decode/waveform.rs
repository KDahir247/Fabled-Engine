use crate::{AudioDecodingError, AudioSpecification, SampleFormat};

#[derive(Default)]
pub struct WavReader;

impl WavReader {
    pub fn open_wave<P: AsRef<std::path::Path>>(
        &self,
        wav_path: P,
    ) -> Result<AudioSpecification, AudioDecodingError> {
        let file = std::fs::File::open(wav_path)?;

        // if wave reader successfully retrieve the file size. otherwise we
        // can guarantee that this file is not a wavefront format
        // todo maybe if wav_detail is zero then we should return early, since there is
        // no audio.
        // let wav_detail =
        //    hound::read_wave_header(&mut file).map_err(AudioDecodingError::WavError)?;

        let buf_reader = std::io::BufReader::new(file);

        let reader = hound::WavReader::new(buf_reader).map_err(AudioDecodingError::WavError)?;

        let wav_spec = reader.spec();

        let format_target = match wav_spec.sample_format {
            hound::SampleFormat::Float => SampleFormat::F32,
            hound::SampleFormat::Int => SampleFormat::I16,
        };

        Ok(AudioSpecification {
            channel_count: wav_spec.channels,
            sample_rate: wav_spec.sample_rate,
            bit_per_sample: wav_spec.bits_per_sample,
            sample_format: format_target,
        })
    }
}


#[cfg(test)]
mod wav_decoding_test {
    use crate::WavReader;

    #[test]
    fn decoding_file_test() {
        let wav_reader = WavReader::default();

        let wav_path = [env!("CARGO_MANIFEST_DIR"), "/src/audio/recorded.wav"].join("");

        let wav_spec = wav_reader.open_wave(wav_path).unwrap();

        println!("channel count : {}", wav_spec.channel_count);
        println!("sample rate : {}", wav_spec.sample_rate);
        println!("bit per sample : {}", wav_spec.bit_per_sample);
        println!("sample format : {:?}", wav_spec.sample_format);
    }
}
