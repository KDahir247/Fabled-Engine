use crate::DeviceConfig;

use cpal::traits::{DeviceTrait, HostTrait};
use rayon::prelude::*;
use std::ops::DerefMut;

pub struct InputConfig {
    pub input_device: Option<cpal::Device>,
    pub input_config: Option<DeviceConfig>,
}

impl Default for InputConfig {
    fn default() -> Self {
        let default_host = cpal::default_host();

        let input_device = default_host.default_input_device();

        let input_config = match &input_device {
            Some(device) => {
                let supported_input_configs = device.supported_input_configs().ok();

                supported_input_configs.map(|desired_input_configs| {
                    let optimal_input_config_range = desired_input_configs
                        .max_by_key(|config_predicate| config_predicate.max_sample_rate())
                        .unwrap();

                    let desired_config_max = optimal_input_config_range.with_max_sample_rate();

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
            input_device,
            input_config,
        }
    }
}

impl InputConfig {
    pub fn retrieve_from_host() -> Vec<InputConfig> {
        let available_hosts = cpal::available_hosts();

        let input_configs = std::sync::Arc::new(parking_lot::Mutex::new(Vec::with_capacity(
            available_hosts.len(),
        )));

        available_hosts.par_iter().for_each(|host_id| {
            let host = cpal::host_from_id(*host_id);

            match host {
                Ok(host) => {
                    let input_device = host.default_input_device();

                    if let Some(device) = input_device {
                        let input_config = device.supported_input_configs().map_or(
                            None,
                            |supported_input_config| {
                                let optimal_input_config_range =
                                    supported_input_config.max_by_key(|config_predicate| {
                                        config_predicate.max_sample_rate().0
                                    });

                                if let Some(desired_config) = optimal_input_config_range {
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

                        let input_config_guard = input_configs.clone();

                        let input_device_detail = InputConfig {
                            input_device: Some(device),
                            input_config,
                        };

                        input_config_guard.lock().push(input_device_detail);
                    }
                }
                Err(err) => {
                    println!("Host is Unavailable.\nMessage : {:?}", err);
                }
            }
        });

        let mut input_config_guard = input_configs.lock();

        let result = std::mem::take(input_config_guard.deref_mut());

        result
    }

    pub fn retrieve_from_devices() -> Vec<InputConfig> {
        let host = cpal::default_host();

        let input_devices = host.devices().unwrap();

        let input_configs = std::sync::Arc::new(parking_lot::Mutex::new(Vec::new()));

        input_devices
            .par_bridge()
            .into_par_iter()
            .for_each(|device| {
                let input_config =
                    device
                        .supported_input_configs()
                        .map_or(None, |support_input_config| {
                            support_input_config
                                .max_by_key(|config_predicate| config_predicate.max_sample_rate().0)
                                .map(|optimal_input_config_range| {
                                    let desired_config_max =
                                        optimal_input_config_range.with_max_sample_rate();

                                    DeviceConfig {
                                        sample_rate: desired_config_max.sample_rate().0,
                                        channel_count: desired_config_max.channels(),
                                        sample_format: desired_config_max.sample_format().into(),
                                        buffer_size: desired_config_max.buffer_size().into(),
                                    }
                                })
                        });


                let input_config_guard = input_configs.clone();

                let input_device_detail = InputConfig {
                    input_device: Some(device),
                    input_config,
                };

                input_config_guard.lock().push(input_device_detail);
            });

        let mut input_config_guard = input_configs.lock();

        let result = std::mem::take(input_config_guard.deref_mut());

        result
    }
}

#[cfg(test)]
mod input_config_test {
    use crate::InputConfig;
    use cpal::traits::DeviceTrait;

    #[test]
    fn single_best_output() {
        let input_config = InputConfig::default();

        print!("{:?} ", input_config.input_device.unwrap().name());
        print!("{:?}", input_config.input_config);
    }


    #[test]
    fn collection_output_from_host() {
        let input_configs = InputConfig::retrieve_from_host();

        assert!(!input_configs.is_empty());

        for config in input_configs {
            print!("{:?} ", config.input_device.unwrap().name());
            println!("{:?}", config.input_config);
        }
    }

    #[test]
    fn collection_output_from_devices() {
        let input_configs = InputConfig::retrieve_from_devices();

        assert!(!input_configs.is_empty());

        for config in input_configs {
            print!("{:?} ", config.input_device.unwrap().name());
            println!("{:?}", config.input_config);
        }
    }
}
