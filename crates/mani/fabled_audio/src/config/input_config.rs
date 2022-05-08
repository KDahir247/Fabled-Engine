// There will only be one InputConfig, so mostly all if not all function
// will be cold. InputConfig will be added to ECS Unique

use crate::DeviceConfig;
use cpal::traits::{DeviceTrait, HostTrait};


pub struct InputConfig {
    pub input_device: Option<cpal::Device>,
    pub input_config: DeviceConfig,
}

impl Default for InputConfig {
    #[cold]
    fn default() -> Self {
        let default_host: cpal::Host = cpal::default_host();

        let input_device: Option<cpal::Device> = default_host.default_input_device();

        let input_config: DeviceConfig = match &input_device {
            None => DeviceConfig::default(),
            Some(device) => {
                let supported_input_configs: cpal::SupportedInputConfigs =
                    device.supported_input_configs().unwrap();

                let optimal_input_config_range: cpal::SupportedStreamConfigRange =
                    supported_input_configs
                        .max_by(|curr, next| curr.cmp_default_heuristics(next))
                        .unwrap();

                let desired_input_config: cpal::SupportedStreamConfig =
                    optimal_input_config_range.with_max_sample_rate();

                DeviceConfig {
                    sample_rate: desired_input_config.sample_rate().0,
                    channel_count: desired_input_config.channels(),
                    buffer_size: desired_input_config.buffer_size().into(),
                    sample_format: desired_input_config.sample_format().into(),
                }
            }
        };

        Self {
            input_device,
            input_config,
        }
    }
}

impl InputConfig {
    #[cold]
    pub fn retrieve_from_host() -> Vec<InputConfig> {
        let available_hosts: Vec<cpal::HostId> = cpal::available_hosts();

        let mut input_configs: Vec<InputConfig> = Vec::with_capacity(available_hosts.len() + 5);

        available_hosts
            .iter()
            .filter_map(|&host_id: &cpal::HostId| cpal::host_from_id(host_id).ok())
            .for_each(|valid_host: cpal::Host| {
                let input_device: Option<cpal::Device> = valid_host.default_input_device();

                if let Some(valid_device) = input_device {
                    let supported_input_config: cpal::SupportedInputConfigs =
                        valid_device.supported_input_configs().unwrap();

                    let optimal_input_config_range: Option<cpal::SupportedStreamConfigRange> =
                        supported_input_config
                            .max_by(|curr, next| curr.cmp_default_heuristics(next));

                    if let Some(valid_config_range) = optimal_input_config_range {
                        let desired_input_config = valid_config_range.with_max_sample_rate();

                        let input_config = InputConfig {
                            input_device: Some(valid_device),
                            input_config: DeviceConfig {
                                sample_rate: desired_input_config.sample_rate().0,
                                channel_count: desired_input_config.channels(),
                                buffer_size: desired_input_config.buffer_size().into(),
                                sample_format: desired_input_config.sample_format().into(),
                            },
                        };

                        input_configs.push(input_config);
                    }
                }
            });

        input_configs
    }

    #[cold]
    pub fn retrieve_from_devices() -> Vec<InputConfig> {
        let host: cpal::Host = cpal::default_host();

        let input_devices: Result<cpal::InputDevices<cpal::Devices>, cpal::DevicesError> =
            host.input_devices();

        let input_configs: Vec<InputConfig> = Vec::with_capacity(10);


        todo!()
        // let host = cpal::default_host();
        //
        // let input_devices = host.devices().unwrap();
        //
        // let input_configs =
        // std::sync::Arc::new(parking_lot::Mutex::new(Vec::new()));
        //
        // input_devices
        //     .par_bridge()
        //     .into_par_iter()
        //     .for_each(|device| {
        //         let input_config =
        //             device
        //                 .supported_input_configs()
        //                 .map_or(None, |support_input_config| {
        //                     support_input_config
        //                         .max_by_key(|config_predicate|
        // config_predicate.max_sample_rate().0)
        // .map(|optimal_input_config_range| {
        // let desired_config_max =
        // optimal_input_config_range.with_max_sample_rate();
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
        //
        //         let input_config_guard = input_configs.clone();
        //
        //         let input_device_detail = InputConfig {
        //             input_device: Some(device),
        //             input_config,
        //         };
        //
        //         input_config_guard.lock().push(input_device_detail);
        //     });
        //
        // let mut input_config_guard = input_configs.lock();
        //
        // let result = std::mem::take(input_config_guard.deref_mut());
        //
        // result
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
