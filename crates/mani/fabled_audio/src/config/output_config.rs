use crate::SampleFormat;
use cpal::traits::{DeviceTrait, HostTrait};
use rayon::prelude::*;
use std::ops::DerefMut;

#[derive(Clone, Debug)]
pub struct OutputDeviceConfig {
    pub sample_rate: u32,
    pub channel_count: u16,
    pub sample_format: SampleFormat,
}

pub struct OutputConfig {
    pub device: Option<cpal::Device>,
    pub output_config: Option<OutputDeviceConfig>,
}

impl Default for OutputConfig {
    fn default() -> Self {
        let default_host = cpal::default_host();
        let output_device = default_host.default_output_device();

        let desired_output_config = match &output_device {
            Some(device) => {
                let supported_configs = device.supported_output_configs().ok();

                supported_configs.map(|desired_configs| {
                    // We must have a valid device that is not disconnect at this point a bug if
                    // unwrap fails.
                    let optimal_config_range = desired_configs
                        .max_by_key(|config_pred| config_pred.max_sample_rate())
                        .unwrap();

                    let desired_config_max = optimal_config_range.with_max_sample_rate();

                    OutputDeviceConfig {
                        sample_rate: desired_config_max.sample_rate().0,
                        channel_count: desired_config_max.channels(),
                        sample_format: desired_config_max.sample_format().into(),
                    }
                })
            }
            None => None,
        };


        Self {
            device: output_device,
            output_config: desired_output_config,
        }
    }
}

impl OutputConfig {
    pub fn retrieve_all_host() -> Vec<OutputConfig> {
        let available_hosts = cpal::available_hosts();

        let devices = std::sync::Arc::new(parking_lot::Mutex::new(Vec::with_capacity(
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
                            |supported_out_config| {
                                let optimal_config_range = supported_out_config
                                    .max_by_key(|config_pred| config_pred.max_sample_rate().0);

                                if let Some(desired_config) = optimal_config_range {
                                    let desired_config_max = desired_config.with_max_sample_rate();

                                    Some(OutputDeviceConfig {
                                        sample_rate: desired_config_max.sample_rate().0,
                                        channel_count: desired_config_max.channels(),
                                        sample_format: desired_config_max.sample_format().into(),
                                    })
                                } else {
                                    None
                                }
                            },
                        );

                        let clone = devices.clone();

                        let output_device_detail = OutputConfig {
                            device: Some(device),
                            output_config,
                        };

                        clone.lock().push(output_device_detail);
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


    pub fn retrieve_all_devices() -> Vec<OutputConfig> {
        let host = cpal::default_host();

        let output_devices = host.devices().unwrap();

        let devices = std::sync::Arc::new(parking_lot::Mutex::new(Vec::new()));

        output_devices
            .par_bridge()
            .into_par_iter()
            .for_each(|device| {
                let output_value =
                    device
                        .supported_output_configs()
                        .map_or(None, |supported_out_config| {
                            supported_out_config
                                .max_by_key(|config_pred| config_pred.max_sample_rate().0)
                                .map(|optimal_config_range| {
                                    let desired_config_max =
                                        optimal_config_range.with_max_sample_rate();

                                    OutputDeviceConfig {
                                        sample_rate: desired_config_max.sample_rate().0,
                                        channel_count: desired_config_max.channels(),
                                        sample_format: desired_config_max.sample_format().into(),
                                    }
                                })
                        });

                let clone = devices.clone();

                let output_device_detail = OutputConfig {
                    device: Some(device),
                    output_config: output_value,
                };

                clone.lock().push(output_device_detail);
            });


        let mut lock = devices.lock();

        let result = std::mem::take(lock.deref_mut());
        result
    }
}


#[cfg(test)]
mod output_config_test {
    use crate::OutputConfig;
    use rodio::DeviceTrait;

    #[test]
    fn single_best_output() {
        let output_config = OutputConfig::default();
        print!("{:?} ", output_config.device.unwrap().name());
        println!("{:?}", output_config.output_config);
    }

    #[test]
    fn collection_output_from_host() {
        let output_configs = OutputConfig::retrieve_all_host();

        assert!(!output_configs.is_empty());

        for config in output_configs {
            print!("{:?} ", config.device.unwrap().name());
            println!("{:?}", config.output_config);
        }
    }


    #[test]
    fn collection_output_from_devices() {
        let output_configs = OutputConfig::retrieve_all_devices();

        assert!(!output_configs.is_empty());

        for config in output_configs {
            print!("{:?} ", config.device.unwrap().name());
            println!("{:?}", config.output_config);
        }
    }
}
