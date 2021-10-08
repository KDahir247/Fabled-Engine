use crate::SampleFormat;
use cpal::traits::{DeviceTrait, HostTrait};
use rayon::prelude::*;
use std::ops::DerefMut;

#[derive(Clone, Debug)]
pub struct InputDeviceConfig {
    pub sample_rate: u32,
    pub channel_count: u16,
    pub sample_format: SampleFormat,
    pub buffer_size: cpal::SupportedBufferSize,
}

#[repr(align(16))]
pub struct InputConfig {
    pub device: Option<cpal::Device>,
    pub input_config: Option<InputDeviceConfig>,
}

impl Default for InputConfig {
    fn default() -> Self {
        let default_host = cpal::default_host();
        let input_device = default_host.default_input_device();

        let desired_input_config = match &input_device {
            Some(device) => {
                let supported_configs = device.supported_input_configs().ok();

                supported_configs.map(|desired_configs| {
                    let optimal_config_range = desired_configs
                        .max_by_key(|config_pred| config_pred.max_sample_rate())
                        .unwrap();

                    let desired_config_max = optimal_config_range.with_max_sample_rate();

                    InputDeviceConfig {
                        sample_rate: desired_config_max.sample_rate().0,
                        channel_count: desired_config_max.channels(),
                        sample_format: desired_config_max.sample_format().into(),
                        buffer_size: desired_config_max.buffer_size().to_owned(),
                    }
                })
            }
            None => None,
        };

        Self {
            device: input_device,
            input_config: desired_input_config,
        }
    }
}

impl InputConfig {
    pub fn retrieve_all_host() -> Vec<InputConfig> {
        let available_hosts = cpal::available_hosts();

        let devices = std::sync::Arc::new(parking_lot::Mutex::new(Vec::with_capacity(
            available_hosts.len(),
        )));

        available_hosts.par_iter().for_each(|host_id| {
            let host = cpal::host_from_id(*host_id);

            match host {
                Ok(host) => {
                    let input_device = host.default_input_device();

                    if let Some(device) = input_device {
                        let input_config =
                            device
                                .supported_input_configs()
                                .map_or(None, |supported_in_config| {
                                    let optimal_config_range = supported_in_config
                                        .max_by_key(|config_pred| config_pred.max_sample_rate().0);

                                    if let Some(desired_config) = optimal_config_range {
                                        let desired_config_max =
                                            desired_config.with_max_sample_rate();

                                        Some(InputDeviceConfig {
                                            sample_rate: desired_config_max.sample_rate().0,
                                            channel_count: desired_config_max.channels(),
                                            sample_format: desired_config_max
                                                .sample_format()
                                                .into(),
                                            buffer_size: desired_config_max
                                                .buffer_size()
                                                .to_owned(),
                                        })
                                    } else {
                                        None
                                    }
                                });

                        let clone = devices.clone();

                        let input_device_detail = InputConfig {
                            device: Some(device),
                            input_config,
                        };

                        clone.lock().push(input_device_detail);
                    }
                }
                Err(err) => {
                    println!("Host is Unavailable.\nMessage : {:?}", err);
                }
            }
        });

        let mut lock = devices.lock();

        let result = std::mem::take(lock.deref_mut());
        result
    }

    pub fn retrieve_all_devices() -> Vec<InputConfig> {
        // retrieve all input devices
        let host = cpal::default_host();

        let input_devices = host.devices().unwrap();

        let devices = std::sync::Arc::new(parking_lot::Mutex::new(Vec::new()));

        input_devices
            .par_bridge()
            .into_par_iter()
            .for_each(|device| {
                //
                let input_value =
                    device
                        .supported_input_configs()
                        .map_or(None, |support_in_config| {
                            support_in_config
                                .max_by_key(|confg_pred| confg_pred.max_sample_rate().0)
                                .map(|optimal_conf_range| {
                                    let desired_config_max =
                                        optimal_conf_range.with_max_sample_rate();


                                    InputDeviceConfig {
                                        sample_rate: desired_config_max.sample_rate().0,
                                        channel_count: desired_config_max.channels(),
                                        sample_format: desired_config_max.sample_format().into(),
                                        buffer_size: desired_config_max.buffer_size().to_owned(),
                                    }
                                })
                        });


                let clone = devices.clone();

                let input_device_detail = InputConfig {
                    device: Some(device),
                    input_config: input_value,
                };

                clone.lock().push(input_device_detail);
            });

        let mut lock = devices.lock();

        let result = std::mem::take(lock.deref_mut());
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

        print!("{:?} ", input_config.device.unwrap().name());
        print!("{:?}", input_config.input_config);
    }


    #[test]
    fn collection_output_from_host() {
        let input_configs = InputConfig::retrieve_all_host();

        assert!(!input_configs.is_empty());

        for config in input_configs {
            print!("{:?} ", config.device.unwrap().name());
            println!("{:?}", config.input_config);
        }
    }

    #[test]
    fn collection_output_from_devices() {
        let input_configs = InputConfig::retrieve_all_devices();

        assert!(!input_configs.is_empty());

        for config in input_configs {
            print!("{:?} ", config.device.unwrap().name());
            println!("{:?}", config.input_config);
        }
    }
}
