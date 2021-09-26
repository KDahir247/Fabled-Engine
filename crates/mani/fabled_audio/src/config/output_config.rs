use crate::SampleFormat;
use cpal::traits::{DeviceTrait, HostTrait};

// todo handle unwrap errors.

pub struct OutputDeviceConfig {
    pub sample_rate: u32,
    pub channel_count: u16,
    pub sample_format: SampleFormat,
}

pub struct OutputConfig {
    pub device: cpal::Device,
    pub output_config: OutputDeviceConfig,
}

impl Default for OutputConfig {
    fn default() -> Self {
        let default_host = cpal::default_host();

        let device = default_host.default_output_device().unwrap();

        let supported_configs = device.supported_output_configs().unwrap();

        let desired_config = supported_configs
            .max_by_key(|config_pred| config_pred.max_sample_rate())
            .unwrap();

        let desired_config_max = desired_config.with_max_sample_rate();

        let output_config = OutputDeviceConfig {
            sample_rate: desired_config_max.sample_rate().0,
            channel_count: desired_config_max.channels(),
            sample_format: desired_config_max.sample_format().into(),
        };


        Self {
            device,
            output_config,
        }
    }
}

impl OutputDeviceConfig {
    pub fn retrieve_all() -> Vec<(cpal::Device, OutputDeviceConfig)> {
        let available_hosts = cpal::available_hosts();

        let mut devices = Vec::with_capacity(available_hosts.len());

        for host_id in available_hosts {
            let host = cpal::host_from_id(host_id);

            if let Ok(host) = host {
                let default_out = host.default_output_device();

                if let Some(device) = default_out {
                    if let Ok(configs) = device.supported_output_configs() {
                        let config = configs
                            .max_by_key(|config_pred| config_pred.max_sample_rate().0)
                            .unwrap();

                        let config = config.with_max_sample_rate();

                        let output_config = OutputDeviceConfig {
                            channel_count: config.channels(),
                            sample_rate: config.sample_rate().0,
                            sample_format: config.sample_format().into(),
                        };

                        devices.push((device, output_config))
                    };
                };
            }
        }
        devices
    }
}
