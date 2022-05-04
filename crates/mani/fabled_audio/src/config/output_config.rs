// There will only be one OutputConfig, so all mostly all if not all function
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
        let default_host = cpal::default_host();

        let output_device = default_host.default_output_device();

        let output_cfg = match &output_device {
            Some(device) => {
                let supported_output_configs = device.supported_output_configs().unwrap();

                let optimal_output_config_range = supported_output_configs
                    .max_by(|curr, next| curr.max_sample_rate().cmp(&next.max_sample_rate()))
                    .unwrap();

                let desired_config = optimal_output_config_range.with_max_sample_rate();

                DeviceConfig {
                    sample_rate: desired_config.sample_rate().0,
                    channel_count: desired_config.channels(),
                    buffer_size: desired_config.buffer_size().into(),
                    sample_format: desired_config.sample_format().into(),
                }
            }
            None => DeviceConfig::default(),
        };

        Self {
            output_device,
            output_config: output_cfg,
        }
    }
}

impl OutputConfig {
    #[cold]
    pub fn retrieve_from_host() -> Vec<OutputConfig> {
        let available_hosts = cpal::available_hosts();

        let mut output_configs: Vec<OutputConfig> = Vec::with_capacity(available_hosts.len());

        for host_id in available_hosts {
            let host = cpal::host_from_id(host_id);

            if let Ok(valid_host) = host {
                // should be at least one device right.. monitor audio?
                let output_device = valid_host.default_output_device().unwrap();

                let supported_output_config = output_device.supported_output_configs().unwrap();

                let optimal_output_config_range = supported_output_config
                    .max_by(|curr, next| curr.max_sample_rate().cmp(&next.max_sample_rate()));

                if let Some(desired_config_range) = optimal_output_config_range {
                    let desired_config = desired_config_range.with_max_sample_rate();

                    let output_config = OutputConfig {
                        output_device: Some(output_device),
                        output_config: DeviceConfig {
                            sample_rate: desired_config.sample_rate().0,
                            channel_count: desired_config.channels(),
                            buffer_size: desired_config.buffer_size().into(),
                            sample_format: desired_config.sample_format().into(),
                        },
                    };

                    output_configs.push(output_config);
                }
            }
        }

        output_configs
    }


    #[cold]
    pub fn retrieve_from_devices() -> Vec<OutputConfig> {
        todo!()

        // let host = cpal::default_host();
        //
        // let output_devices = host.devices().unwrap();
        //
        // let output_configs =
        // std::sync::Arc::new(parking_lot::Mutex::new(Vec::new()));
        //
        // output_devices
        //     .par_bridge()
        //     .into_par_iter()
        //     .for_each(|device| {
        //         let output_config =
        //             device
        //                 .supported_output_configs()
        //                 .map_or(None, |support_output_config| {
        //                     support_output_config
        //                         .max_by_key(|config_predicate|
        // config_predicate.max_sample_rate().0)
        // .map(|optimal_output_config_range| {
        // let desired_config_max =
        // optimal_output_config_range.with_max_sample_rate();
        //
        //                             DeviceConfig {
        //                                 sample_rate:
        // desired_config_max.sample_rate().0,
        // channel_count: desired_config_max.channels(),
        // sample_format: desired_config_max.sample_format().into(),
        //                                 buffer_size:
        // desired_config_max.buffer_size().into(),
        // }                         })
        //                 });
        //
        //         let output_config_guard = output_configs.clone();
        //
        //         let output_device_detail = OutputConfig {
        //             output_device: Some(device),
        //             output_config : output_config.unwrap(),
        //         };
        //
        //         output_config_guard.lock().push(output_device_detail);
        //     });
        //
        // let mut output_config_guard = output_configs.lock();
        //
        // let result = std::mem::take(output_config_guard.deref_mut());
        // result
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
