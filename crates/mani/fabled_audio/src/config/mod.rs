mod input_config;
mod output_config;

pub use input_config::*;
pub use output_config::*;


#[cfg(test)]
mod data_test {
    use crate::{InputConfig, OutputConfig};

    #[test]
    fn data_size() {
        let output_config_size = std::mem::size_of::<OutputConfig>();
        assert_eq!(output_config_size & (output_config_size - 1), 0);

        let input_config_size = std::mem::size_of::<InputConfig>();
        assert_eq!(input_config_size & (input_config_size - 1), 0);
    }

    #[test]
    fn data_alignment() {
        let output_config_align = std::mem::align_of::<OutputConfig>();
        assert_eq!(output_config_align & (output_config_align - 1), 0);

        let input_config_align = std::mem::align_of::<InputConfig>();
        assert_eq!(input_config_align & (input_config_align - 1), 0);
    }
}
