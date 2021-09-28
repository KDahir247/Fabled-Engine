mod output_config;

pub use output_config::*;


#[cfg(test)]
mod data_test {
    use crate::OutputConfig;

    #[test]
    fn data_size() {
        let output_config_size = std::mem::size_of::<OutputConfig>();
        println!("{}", output_config_size);
    }

    #[test]
    fn data_alignment() {
        let output_config_align = std::mem::align_of::<OutputConfig>();
        println!("{}", output_config_align);
    }
}
