use crate::DeviceConfig;

use cpal::traits::{DeviceTrait, HostTrait};
use rayon::prelude::*;
use std::ops::DerefMut;

pub struct OutputConfig {
    pub output_device: Option<cpal::Device>,
    pub output_config: Option<DeviceConfig>,
}

impl Default for OutputConfig {
    fn default() -> Self {
        let default_host = cpal::default_host();

        let output_device = default_host.default_output_device();

        let output_config = match &output_device {
            Some(device) => {
                let supported_output_configs = device.supported_output_configs().ok();

                supported_output_configs.map(|desired_output_configs| {
                    let optimal_output_config_range = desired_output_configs
                        .max_by_key(|config_predicate| config_predicate.max_sample_rate())
                        .unwrap();

                    let desired_config_max = optimal_output_config_range.with_max_sample_rate();

                    DeviceConfig {
                        sample_rate: desired_config_max.sample_rate().0,
                        channel_count: desired_config_max.channels(),
                        sample_format: desired_config_max.sample_format().into(),
                        buffer_size: desired_config_max.buffer_size().into(),
                    }
                })
            }
            None => None,
        };


        Self {
            output_device,
            output_config,
        }
    }
}

impl OutputConfig {
    pub fn retrieve_from_host() -> Vec<OutputConfig> {
        let available_hosts = cpal::available_hosts();

        let output_configs = std::sync::Arc::new(parking_lot::Mutex::new(Vec::with_capacity(
            available_hosts.len(),
        )));

        available_hosts.par_iter().for_each(|host_id| {
            let host = cpal::host_from_id(*host_id);

            match host {
                Ok(host) => {
                    let output_device = host.default_output_device();

                    if let Some(device) = output_device {
                        let output_config = device.supported_output_configs().map_or(
                            None,
                            |supported_output_config| {
                                let optimal_output_config_range = supported_output_config
                                    .max_by_key(|config_predicate| {
                                        config_predicate.max_sample_rate().0
                                    });

                                if let Some(desired_config) = optimal_output_config_range {
                                    let desired_config_max = desired_config.with_max_sample_rate();

                                    Some(DeviceConfig {
                                        sample_rate: desired_config_max.sample_rate().0,
                                        channel_count: desired_config_max.channels(),
                                        sample_format: desired_config_max.sample_format().into(),
                                        buffer_size: desired_config_max.buffer_size().into(),
                                    })
                                } else {
                                    None
                                }
                            },
                        );

                        let output_config_guard = output_configs.clone();

                        let output_device_detail = OutputConfig {
                            output_device: Some(device),
                            output_config,
                        };

                        output_config_guard.lock().push(output_device_detail);
                    }
                }
                Err(err) => {
                    println!("Host is Unavailable.\nMessage : {:?}", err);
                }
            }
        });

        let mut output_config_guard = output_configs.lock();

        let result = std::mem::take(output_config_guard.deref_mut());
        result
    }


    pub fn retrieve_from_devices() -> Vec<OutputConfig> {
        let host = cpal::default_host();

        let output_devices = host.devices().unwrap();

        let output_configs = std::sync::Arc::new(parking_lot::Mutex::new(Vec::new()));

        output_devices
            .par_bridge()
            .into_par_iter()
            .for_each(|device| {
                let output_config =
                    device
                        .supported_output_configs()
                        .map_or(None, |support_output_config| {
                            support_output_config
                                .max_by_key(|config_predicate| config_predicate.max_sample_rate().0)
                                .map(|optimal_output_config_range| {
                                    let desired_config_max =
                                        optimal_output_config_range.with_max_sample_rate();

                                    DeviceConfig {
                                        sample_rate: desired_config_max.sample_rate().0,
                                        channel_count: desired_config_max.channels(),
                                        sample_format: desired_config_max.sample_format().into(),
                                        buffer_size: desired_config_max.buffer_size().into(),
                                    }
                                })
                        });

                let output_config_guard = output_configs.clone();

                let output_device_detail = OutputConfig {
                    output_device: Some(device),
                    output_config,
                };

                output_config_guard.lock().push(output_device_detail);
            });

        let mut output_config_guard = output_configs.lock();

        let result = std::mem::take(output_config_guard.deref_mut());
        result
    }
}


#[cfg(test)]
mod output_config_test {
    use crate::OutputConfig;
    use cpal::traits::DeviceTrait;

    #[test]
    fn single_best_output() {
        let output_config = OutputConfig::default();
        print!("{:?} ", output_config.output_device.unwrap().name());
        print!("{:?}", output_config.output_config);
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
