// Not using threading yet, since that is dedicated to Fabled_Concurrency. I
// will revisit this and implement threading rather the blocking for seconds
// when Fabled_Concurrency is supported.

use crate::{AudioEncodingError, InputConfig, SampleFormat};
use cpal::traits::{DeviceTrait, StreamTrait};

type WavWriterHandle = std::sync::Arc<
    parking_lot::lock_api::Mutex<
        parking_lot::RawMutex,
        Option<hound::WavWriter<std::io::BufWriter<std::fs::File>>>,
    >,
>;

pub struct WavWriter;

impl Default for WavWriter {
    fn default() -> Self {
        Self {}
    }
}
impl WavWriter {
    pub fn create_wav<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        config: Option<InputConfig>,
    ) -> Result<(), AudioEncodingError> {
        // Creating Data.

        let InputConfig {
            device,
            input_config,
        } = config.unwrap_or_default();

        let device = device.ok_or(AudioEncodingError::NoDeviceError)?;
        let input_config = input_config.ok_or(AudioEncodingError::NoInputConfigError)?;

        let wav_spec = hound::WavSpec {
            channels: input_config.channel_count as _,
            sample_rate: input_config.sample_rate as _,
            bits_per_sample: (input_config.sample_format.sample_size() * 8) as _,
            sample_format: input_config.sample_format.into(),
        };

        let input_stream_config = cpal::StreamConfig {
            channels: input_config.channel_count,
            sample_rate: cpal::SampleRate(input_config.sample_rate),
            buffer_size: input_config.buffer_size.into(),
        };

        let sample_format = match input_config.sample_format {
            SampleFormat::I16 => cpal::SampleFormat::I16,
            SampleFormat::U16 => cpal::SampleFormat::U16,
            SampleFormat::F32 => cpal::SampleFormat::F32,
        };

        let writer =
            hound::WavWriter::create(path, wav_spec).map_err(AudioEncodingError::WavError)?;

        let concurrent_writer = std::sync::Arc::new(parking_lot::Mutex::new(Some(writer)));

        let writer_2 = concurrent_writer.clone();

        let error_pred = move |err| println!("an error has occurred on stream: {}", err);

        let stream = device.build_input_stream_raw(
            &input_stream_config,
            sample_format,
            move |data, &_| {
                match data.sample_format() {
                    cpal::SampleFormat::I16 => {
                        let data_buffer = data.as_slice().unwrap_or_default();

                        Self::write_input_data::<i16, i16>(data_buffer, &writer_2)
                    }
                    cpal::SampleFormat::U16 => {
                        let data_buffer = data.as_slice().unwrap_or_default();

                        Self::write_input_data::<u16, i16>(data_buffer, &writer_2)
                    }
                    cpal::SampleFormat::F32 => {
                        let data_buffer = data.as_slice().unwrap_or_default();

                        Self::write_input_data::<f32, f32>(data_buffer, &writer_2)
                    }
                };
            },
            error_pred,
        );

        let stream = stream.map_err(AudioEncodingError::BuildStreamError)?;

        {
            stream.play().unwrap();

            // temp place holder when till fabled_concurrency module is complete.
            std::thread::sleep(std::time::Duration::from_secs(5));

            concurrent_writer
                .lock()
                .take()
                .unwrap()
                .finalize()
                .map_err(AudioEncodingError::WavError)?;
        }

        Ok(())
    }


    fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
    where
        T: cpal::Sample,
        U: cpal::Sample + hound::Sample, {
        if let Some(mut guard) = writer.try_lock() {
            if let Some(writer) = guard.as_mut() {
                let chunk_data = input.chunks_exact(6);

                let remaining_chunk_data = chunk_data.remainder();

                for sample in chunk_data {
                    let sample_0: U = cpal::Sample::from(&sample[0]);
                    let sample_1: U = cpal::Sample::from(&sample[1]);
                    let sample_2: U = cpal::Sample::from(&sample[2]);
                    let sample_3: U = cpal::Sample::from(&sample[3]);
                    let sample_4: U = cpal::Sample::from(&sample[4]);
                    let sample_5: U = cpal::Sample::from(&sample[5]);

                    writer.write_sample(sample_0).unwrap();
                    writer.write_sample(sample_1).unwrap();
                    writer.write_sample(sample_2).unwrap();
                    writer.write_sample(sample_3).unwrap();
                    writer.write_sample(sample_4).unwrap();
                    writer.write_sample(sample_5).unwrap();

                    writer.flush().unwrap();
                }


                for sample in remaining_chunk_data {
                    let sample: U = cpal::Sample::from(sample);
                    writer.write_sample(sample).unwrap();
                }

                writer.flush().unwrap();
            }
        }
    }
}

#[cfg(test)]
mod wav_test {
    use crate::{InputConfig, WavWriter};


    #[test]
    fn recording_voice() {
        let input_config = InputConfig::default();

        const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/src/audio/recorded.wav");
        let wav_writer = WavWriter::default();

        wav_writer.create_wav(PATH, Some(input_config)).unwrap();
    }
}
