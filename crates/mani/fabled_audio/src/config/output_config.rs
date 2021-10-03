use crate::SampleFormat;
use ambisonic::rodio::Device;
use cpal::traits::{DeviceTrait, HostTrait};

use rayon::prelude::*;
use std::ops::DerefMut;

// todo add more functionality to this struct.
//  add ConfigInput to allow user to record audio

#[derive(Clone, Debug)]
pub struct OutputDeviceConfig {
    pub sample_rate: u32,
    pub channel_count: u16,
    pub sample_format: SampleFormat,
}

pub struct OutputConfig {
    pub device: Option<cpal::Device>,
    pub output_config: OutputDeviceConfig,
}

impl Default for OutputConfig {
    fn default() -> Self {
        let default_host = cpal::default_host();
        let output_device = default_host.default_output_device();

        let desired_output_config = match &output_device {
            Some(device) => {
                let supported_configs = device.supported_output_configs().ok();

                supported_configs.map_or((0, 0, SampleFormat::F32), |desired_configs| {
                    // We must have a valid device that is not disconnect at this point a bug if
                    // unwrap fails.
                    let optimal_config_range = desired_configs
                        .max_by_key(|config_pred| config_pred.max_sample_rate())
                        .unwrap();

                    let desired_config_max = optimal_config_range.with_max_sample_rate();

                    (
                        desired_config_max.sample_rate().0,
                        desired_config_max.channels(),
                        desired_config_max.sample_format().into(),
                    )
                })
            }
            None => (0, 0, SampleFormat::F32),
        };


        let output_config = OutputDeviceConfig {
            sample_rate: desired_output_config.0,
            channel_count: desired_output_config.1,
            sample_format: desired_output_config.2,
        };


        Self {
            device: output_device,
            output_config,
        }
    }
}

impl OutputDeviceConfig {
    pub fn retrieve_all() -> Vec<(Device, OutputDeviceConfig)> {
        let available_hosts = cpal::available_hosts();

        let devices = std::sync::Arc::new(parking_lot::Mutex::new(Vec::with_capacity(
            available_hosts.len(),
        )));

        available_hosts.par_iter().for_each(|host_id| {
            let host = cpal::host_from_id(*host_id);

            match host {
                Ok(host) => {
                    let device_out = host.default_output_device();

                    if let Some(device) = device_out {
                        let output_value = device.supported_output_configs().map_or(
                            (0, 0, SampleFormat::F32),
                            |supported_out_config| {
                                // We must have a valid device that is not disconnect at this
                                // point a bug if unwrap
                                // fails.

                                // todo might change since it only get the first.
                                let optimal_config_range = supported_out_config
                                    .max_by_key(|config_pred| config_pred.max_sample_rate().0)
                                    .unwrap();

                                let desired_config_max =
                                    optimal_config_range.with_max_sample_rate();


                                (
                                    desired_config_max.channels(),
                                    desired_config_max.sample_rate().0,
                                    desired_config_max.sample_format().into(),
                                )
                            },
                        );

                        let device_output_config = OutputDeviceConfig {
                            channel_count: output_value.0,
                            sample_rate: output_value.1,
                            sample_format: output_value.2,
                        };


                        let clone = devices.clone();
                        clone.lock().push((device, device_output_config));
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
}


#[cfg(test)]
mod output_config_test {
    use crate::{OutputConfig, OutputDeviceConfig};
    use rodio::DeviceTrait;

    #[test]
    fn single_best_output() {
        let output_config = OutputConfig::default();
        print!("{:?} ", output_config.device.unwrap().name());
        println!("{:?}", output_config.output_config);
    }

    #[test]
    fn collection_output() {
        let output_configs = OutputDeviceConfig::retrieve_all();

        for config in output_configs {
            print!("{:?} ", config.0.name());
            println!("{:?}", config.1);
        }
    }
}
