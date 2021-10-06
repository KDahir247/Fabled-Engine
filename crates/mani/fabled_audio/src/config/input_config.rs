use crate::SampleFormat;
use cpal::traits::{DeviceTrait, HostTrait};


#[derive(Clone, Debug)]
pub struct InputDeviceConfig {
    pub sample_rate: u32,
    pub channel_count: u16,
    pub sample_format: SampleFormat,
}


#[repr(align(16))]
pub struct InputConfig {
    pub device: Option<cpal::Device>,
    pub output_config: Option<InputDeviceConfig>,
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
                    }
                })
            }
            None => None,
        };


        Self {
            device: input_device,
            output_config: desired_input_config,
        }
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
        print!("{:?}", input_config.output_config);
    }
}
