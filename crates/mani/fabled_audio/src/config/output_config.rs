// There will only be one OutputConfig, so mostly all if not all function
// will be cold. OutputConfig will be added to ECS Unique

use crate::DeviceConfig;
use cpal::traits::{DeviceTrait, HostTrait};

pub struct OutputConfig {
    pub output_device: Option<cpal::Device>,
    pub output_config: DeviceConfig,
}


impl Default for OutputConfig {
    #[cold]
    fn default() -> Self {
        let default_host: cpal::Host = cpal::default_host();

        let output_device: Option<cpal::Device> = default_host.default_output_device();

        assert!(output_device.is_some(), "There is no valid output device. \n failed to retrieve output device from monitor or headphone");

        let device: cpal::Device = output_device.unwrap();

        let supported_output_configs: cpal::SupportedOutputConfigs =
            device.supported_output_configs().unwrap();

        let optimal_output_config_range: cpal::SupportedStreamConfigRange =
            supported_output_configs
                .max_by(|curr, next| curr.cmp_default_heuristics(next))
                .unwrap();

        let desired_output_config: cpal::SupportedStreamConfig =
            optimal_output_config_range.with_max_sample_rate();

        Self {
            output_device: Some(device),
            output_config: DeviceConfig {
                sample_rate: desired_output_config.sample_rate().0,
                channel_count: desired_output_config.channels(),
                buffer_size: desired_output_config.buffer_size().into(),
                sample_format: desired_output_config.sample_format().into(),
            },
        }
    }
}

impl OutputConfig {
    #[cold]
    pub fn retrieve_from_host() -> Vec<OutputConfig> {
        let available_hosts: Vec<cpal::HostId> = cpal::available_hosts();

        let mut output_configs: Vec<OutputConfig> = Vec::with_capacity(available_hosts.len() + 5);

        available_hosts
            .iter()
            .filter_map(|&host_id: &cpal::HostId| cpal::host_from_id(host_id).ok())
            .for_each(|valid_host: cpal::Host| {
                let output_device: cpal::Device = valid_host.default_output_device().unwrap();

                let supported_output_config: cpal::SupportedOutputConfigs =
                    output_device.supported_output_configs().unwrap();

                let optimal_output_config_range: Option<cpal::SupportedStreamConfigRange> =
                    supported_output_config.max_by(|curr, next| curr.cmp_default_heuristics(next));

                if let Some(valid_config_range) = optimal_output_config_range {
                    let desired_output_config: cpal::SupportedStreamConfig =
                        valid_config_range.with_max_sample_rate();

                    let output_config = OutputConfig {
                        output_device: Some(output_device),
                        output_config: DeviceConfig {
                            sample_rate: desired_output_config.sample_rate().0,
                            channel_count: desired_output_config.channels(),
                            buffer_size: desired_output_config.buffer_size().into(),
                            sample_format: desired_output_config.sample_format().into(),
                        },
                    };

                    output_configs.push(output_config);
                }
            });

        output_configs
    }


    #[cold]
    pub fn retrieve_from_devices() -> Vec<OutputConfig> {
        let host: cpal::Host = cpal::default_host();

        let output_device: Result<cpal::Devices, cpal::DevicesError> = host.devices();

        assert!(output_device.is_ok(), "There is no valid output devices.");

        let devices: cpal::Devices = output_device.unwrap();

        let mut output_configs: Vec<OutputConfig> = Vec::with_capacity(10);

        devices.for_each(|device| {
            let supported_output_config: cpal::SupportedOutputConfigs =
                device.supported_output_configs().unwrap();

            let optimal_output_config_range: Option<cpal::SupportedStreamConfigRange> =
                supported_output_config.max_by(|curr, next| curr.cmp_default_heuristics(next));

            if let Some(valid_config_range) = optimal_output_config_range {
                let desired_output_config: cpal::SupportedStreamConfig =
                    valid_config_range.with_max_sample_rate();

                let output_config = OutputConfig {
                    output_device: Some(device),
                    output_config: DeviceConfig {
                        sample_rate: desired_output_config.sample_rate().0,
                        channel_count: desired_output_config.channels(),
                        buffer_size: desired_output_config.buffer_size().into(),
                        sample_format: desired_output_config.sample_format().into(),
                    },
                };

                output_configs.push(output_config);
            }
        });

        output_configs
    }
}


#[cfg(test)]

mod output_config_test {
    use crate::OutputConfig;
    use cpal::traits::DeviceTrait;

    #[test]
    fn single_best_output() {
        let output_config = OutputConfig::default();
        println!("{:?} ", output_config.output_device.unwrap().name());
        println!("{:?}", output_config.output_config);

        println!("{:?}", cpal::available_hosts());
    }

    #[test]
    fn collection_output_from_host() {
        let output_configs = OutputConfig::retrieve_from_host();

        assert!(!output_configs.is_empty());

        for config in output_configs {
            print!("{:?} ", config.output_device.unwrap().name());
            println!("{:?}", config.output_config);
        }
    }


    #[test]
    fn collection_output_from_devices() {
        let output_configs = OutputConfig::retrieve_from_devices();

        assert!(!output_configs.is_empty());

        for config in output_configs {
            print!("{:?} ", config.output_device.unwrap().name());
            println!("{:?}", config.output_config);
        }
    }
}
