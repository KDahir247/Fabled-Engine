// There will only be one OutputConfig, so mostly all if not all function
// will be cold. OutputConfig will be added to ECS Unique

use crate::DeviceConfig;
use cpal::traits::{DeviceTrait, HostTrait};

pub struct OutputConfig {
    pub output_device: cpal::Device,
    pub output_config: Vec<DeviceConfig>,
}


impl Default for OutputConfig {
    #[cold]
    fn default() -> Self {
        Self::retrieve_from_host()
    }
}


impl OutputConfig {
    #[cold]
    pub fn retrieve_from_host() -> OutputConfig {
        // we aren't enabling cpal Asio so there will only be one host  Wasapi
        let available_host: cpal::Host = cpal::default_host();

        let output_device: cpal::Device = available_host.default_output_device().unwrap();

        let supported_output_config: cpal::SupportedOutputConfigs =
            output_device.supported_output_configs().unwrap();

        let valid_config_range: cpal::SupportedStreamConfigRange = supported_output_config
            .max_by(|curr, next| curr.cmp_default_heuristics(next))
            .unwrap();

        let desired_output_config: cpal::SupportedStreamConfig =
            valid_config_range.with_max_sample_rate();

        let device_config = vec![
            (DeviceConfig {
                sample_rate: desired_output_config.sample_rate().0,
                channel_count: desired_output_config.channels(),
                buffer_size: desired_output_config.buffer_size().into(),
                sample_format: desired_output_config.sample_format().into(),
            }),
        ];

        Self {
            output_device,
            output_config: device_config,
        }
    }

    #[cold]
    pub fn retrieve_from_devices() -> Vec<OutputConfig> {
        todo!()
        //     let host: cpal::Host = cpal::default_host();
        //
        //     let output_device: Result<cpal::OutputDevices<cpal::Devices>,
        // cpal::DevicesError> =         host.output_devices();
        //
        //     assert!(output_device.is_ok(), "There is no valid output
        // devices.");
        //
        //     let output_devices: cpal::OutputDevices<cpal::Devices> =
        // output_device.unwrap();
        //
        //     let mut output_configs: Vec<OutputConfig> =
        // Vec::with_capacity(10);
        //
        //     output_devices.for_each(|device| {
        //         let supported_output_config: cpal::SupportedOutputConfigs =
        //             device.supported_output_configs().unwrap();
        //
        //         let optimal_output_config_range:
        // cpal::SupportedStreamConfigRange =
        // supported_output_config                 .max_by(|curr, next|
        // curr.cmp_default_heuristics(next))                 .unwrap();
        //
        //         let desired_output_config: cpal::SupportedStreamConfig =
        //             optimal_output_config_range.with_max_sample_rate();
        //
        //         let output_config = OutputConfig {
        //             output_device: device,
        //             output_config: DeviceConfig {
        //                 sample_rate: desired_output_config.sample_rate().0,
        //                 channel_count: desired_output_config.channels(),
        //                 buffer_size:
        // desired_output_config.buffer_size().into(),
        // sample_format: desired_output_config.sample_format().into(),
        //             },
        //         };
        //
        //         output_configs.push(output_config);
        //     });
        //
        //     output_configs
        // }
    }
}

#[cfg(test)]

mod output_config_test {
    use crate::OutputConfig;
    use cpal::traits::DeviceTrait;

    #[test]
    fn single_best_output() {
        let output_config = OutputConfig::default();
        println!("{:?} ", output_config.output_device.name());
        println!("{:?}", output_config.output_config);

        println!("{:?}", cpal::available_hosts());
    }

    #[test]
    fn collection_output_from_host() {
        let output_config = OutputConfig::retrieve_from_host();

        assert!(!output_config.output_config.is_empty());
        print!("{:?} ", output_config.output_device.name());

        for config in output_config.output_config {
            println!("{:?}", config);
        }
    }


    #[test]
    fn collection_output_from_devices() {
        let output_configs = OutputConfig::retrieve_from_devices();

        assert!(!output_configs.is_empty());

        for config in output_configs {
            print!("{:?} ", config.output_device.name());
            println!("{:?}", config.output_config);
        }
    }
}
