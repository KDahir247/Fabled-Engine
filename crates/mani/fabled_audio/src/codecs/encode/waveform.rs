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
        spec: InputConfig,
    ) -> Result<(), AudioEncodingError> {
        let InputConfig {
            device,
            input_config,
        } = spec;


        let device = device.ok_or(AudioEncodingError::NoDeviceError)?;
        let input_config = input_config.ok_or(AudioEncodingError::NoInputConfigError)?;

        let wav_spec = hound::WavSpec {
            channels: input_config.channel_count,
            sample_rate: input_config.sample_rate,
            bits_per_sample: (input_config.sample_format.sample_size() * 8) as _,
            sample_format: input_config.sample_format.into(),
        };

        let writer = hound::WavWriter::create(path, wav_spec).unwrap();
        let writer = std::sync::Arc::new(parking_lot::Mutex::new(Some(writer)));

        // Starting recording below....
        let writer_2 = writer.clone();

        let buffer_detail = match input_config.buffer_size {
            cpal::SupportedBufferSize::Range { min: _, max } => cpal::BufferSize::Fixed(max),
            cpal::SupportedBufferSize::Unknown => cpal::BufferSize::Default,
        };

        let input_stream_config = cpal::StreamConfig {
            channels: input_config.channel_count,
            sample_rate: cpal::SampleRate(input_config.sample_rate),
            buffer_size: buffer_detail,
        };

        let error_pred = move |err| println!("an error has occurred on stream: {}", err);

        // data is a slice of cpal::Sample
        let stream = match input_config.sample_format {
            SampleFormat::I16 => device
                .build_input_stream(
                    &input_stream_config,
                    move |data, &_| Self::write_input_data::<i16, i16>(data, &writer_2),
                    error_pred,
                )
                .map_err(AudioEncodingError::BuildStreamError)?,
            SampleFormat::U16 => device
                .build_input_stream(
                    &input_stream_config,
                    move |data, &_| Self::write_input_data::<u16, i16>(data, &writer_2),
                    error_pred,
                )
                .map_err(AudioEncodingError::BuildStreamError)?,
            SampleFormat::F32 => device
                .build_input_stream(
                    &input_stream_config,
                    move |data, &_| Self::write_input_data::<f32, f32>(data, &writer_2),
                    error_pred,
                )
                .map_err(AudioEncodingError::BuildStreamError)?,
        };

        stream.play().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(5));
        drop(stream);
        writer.lock().take().unwrap().finalize().unwrap();

        println!("Saved");


        Ok(())
    }


    fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
    where
        T: cpal::Sample,
        U: cpal::Sample + hound::Sample, {
        if let Some(mut guard) = writer.try_lock() {
            if let Some(writer) = guard.as_mut() {
                for &sample in input.iter() {
                    let sample: U = cpal::Sample::from(&sample);
                    writer.write_sample(sample).ok();
                }
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

        const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/recorded.wav");
        WavWriter::default().create_wav(PATH, input_config).unwrap();
    }
}
