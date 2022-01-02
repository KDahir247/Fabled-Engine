mod shading_model;

pub use shading_model::*;


#[cfg(test)]
mod data_test {
    use crate::{FullData, ShaderModel, StandardData};

    #[test]
    fn data_size() {
        let shader_model_size = std::mem::size_of::<ShaderModel>();
        println!("{}", shader_model_size);

        let standard_data_size = std::mem::size_of::<StandardData>();
        println!("{}", standard_data_size);

        let full_data_size = std::mem::size_of::<FullData>();
        println!("{}", full_data_size);
    }


    #[test]
    fn data_alignment() {
        let shader_model_alignment = std::mem::align_of::<ShaderModel>();
        println!("{}", shader_model_alignment);

        let standard_data_alignment = std::mem::align_of::<StandardData>();
        println!("{}", standard_data_alignment);

        let full_data_alignment = std::mem::align_of::<FullData>();
        println!("{}", full_data_alignment);
    }
}
