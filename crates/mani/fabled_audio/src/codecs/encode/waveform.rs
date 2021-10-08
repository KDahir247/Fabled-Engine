use crate::{AudioEncodingError, InputConfig, SampleFormat};
use cpal::traits::{DeviceTrait, StreamTrait};

type WavWriterHandle = std::sync::Arc<
    parking_lot::lock_api::Mutex<
        parking_lot::RawMutex,
        Option<hound::WavWriter<std::io::BufWriter<std::fs::File>>>,
    >,
>;

#[derive(Default)]
pub struct WavWriter;

impl WavWriter {
    pub fn create_wav<P: AsRef<std::path::Path>>(
        &self,
        path: P,
        config: InputConfig,
    ) -> Result<(), AudioEncodingError> {
        // Creating Data.

        let InputConfig {
            device,
            input_config,
        } = config;


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

        let error_pred = move |err| println!("an error has occurred on stream: {}", err);

        let writer =
            hound::WavWriter::create(path, wav_spec).map_err(AudioEncodingError::WavError)?;

        let concurrent_writer = std::sync::Arc::new(parking_lot::Mutex::new(Some(writer)));

        let writer_2 = concurrent_writer.clone();

        // todo don't like how this is repeatedly calling device.build_input_stream.
        let stream = match input_config.sample_format {
            SampleFormat::I16 => device.build_input_stream(
                &input_stream_config,
                move |data: &[i16], &_| Self::write_input_data::<_, i16>(data, &writer_2),
                error_pred,
            ),
            SampleFormat::U16 => device.build_input_stream(
                &input_stream_config,
                move |data: &[u16], &_| Self::write_input_data::<_, i16>(data, &writer_2),
                error_pred,
            ),
            SampleFormat::F32 => device.build_input_stream(
                &input_stream_config,
                move |data: &[f32], &_| Self::write_input_data::<_, f32>(data, &writer_2),
                error_pred,
            ),
        };

        let stream = stream.map_err(AudioEncodingError::BuildStreamError)?;

        {
            // stream.play().unwrap();

            std::thread::sleep(std::time::Duration::from_secs(5));
        }

        concurrent_writer.lock().take().unwrap().finalize().unwrap();

        println!("Saved");


        Ok(())
    }


    fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
    where
        T: cpal::Sample,
        U: cpal::Sample + hound::Sample, {
        if let Some(mut guard) = writer.try_lock() {
            if let Some(writer) = guard.as_mut() {
                // 960 data len.

                for &sample in input.iter() {
                    let sample: U = cpal::Sample::from(&sample);
                    writer.write_sample(sample).ok();
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
        WavWriter::default().create_wav(PATH, input_config).unwrap();
    }
}
