pub use audio_listener::*;
pub use audio_spec::*;
pub use device_config::*;
pub use fade_filter::*;
pub use sample_format::*;
pub use supported_buffer::*;

mod audio_listener;
mod audio_spec;
mod device_config;
mod fade_filter;
mod sample_format;
mod supported_buffer;


#[cfg(test)]
mod data_test {
    use crate::{
        AudioListener, AudioSpecification, DeviceConfig, FadeFilter, FlacReaderOptions,
        SampleFormat, SupportedBufferSize,
    };

    #[test]
    fn data_size() {
        let audio_listener_size = std::mem::size_of::<AudioListener>();
        assert_eq!(audio_listener_size & (audio_listener_size - 1), 0);

        let fade_filter_size = std::mem::size_of::<FadeFilter>();
        assert_eq!(fade_filter_size & (fade_filter_size - 1), 0);

        let sample_format_size = std::mem::size_of::<SampleFormat>();
        assert_eq!(sample_format_size & (sample_format_size - 1), 0);

        let supported_buffer_size = std::mem::size_of::<SupportedBufferSize>();
        assert_eq!(supported_buffer_size & (supported_buffer_size - 1), 0);

        let audio_spec_size = std::mem::size_of::<AudioSpecification>();
        assert_eq!(audio_spec_size & (audio_spec_size - 1), 0);

        let flac_option_size = std::mem::size_of::<FlacReaderOptions>();
        assert_eq!(flac_option_size & (flac_option_size - 1), 0);

        let device_config_size = std::mem::size_of::<DeviceConfig>();
        assert_eq!(device_config_size & (device_config_size - 1), 0);
    }


    #[test]
    fn data_alignment() {
        let audio_listener_alignment = std::mem::align_of::<AudioListener>();
        assert_eq!(audio_listener_alignment & (audio_listener_alignment - 1), 0);

        let fade_filter_alignment = std::mem::align_of::<FadeFilter>();
        assert_eq!(fade_filter_alignment & (fade_filter_alignment - 1), 0);

        let sample_format_alignment = std::mem::align_of::<SampleFormat>();
        assert_eq!(sample_format_alignment & (sample_format_alignment - 1), 0);

        let supported_buffer_alignment = std::mem::align_of::<SupportedBufferSize>();
        assert_eq!(
            supported_buffer_alignment & (supported_buffer_alignment - 1),
            0
        );

        let audio_spec_alignment = std::mem::align_of::<AudioSpecification>();
        assert_eq!(audio_spec_alignment & (audio_spec_alignment - 1), 0);

        let flac_option_alignment = std::mem::align_of::<FlacReaderOptions>();
        assert_eq!(flac_option_alignment & (flac_option_alignment - 1), 0);

        let device_config_alignment = std::mem::align_of::<DeviceConfig>();
        assert_eq!(device_config_alignment & (device_config_alignment - 1), 0);
    }
}
