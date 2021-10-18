pub use illumination_model::*;
pub use load_flag::*;
pub use parse_command::*;
mod illumination_model;
mod load_flag;
mod parse_command;

#[cfg(test)]
mod data_test {
    use crate::{IlluminationModel, LoadFlags};

    #[test]
    fn data_size() {
        let load_flag_size = std::mem::size_of::<LoadFlags>();
        assert_eq!(load_flag_size & (load_flag_size - 1), 0);

        let illum_model_size = std::mem::size_of::<IlluminationModel>();
        assert_eq!(illum_model_size & (illum_model_size - 1), 0);
    }

    #[test]
    fn data_alignment() {
        let load_flag_alignment = std::mem::align_of::<LoadFlags>();
        assert_eq!(load_flag_alignment & (load_flag_alignment - 1), 0);

        let illum_model_alignment = std::mem::align_of::<IlluminationModel>();
        assert_eq!(illum_model_alignment & (illum_model_alignment - 1), 0);
    }
}
