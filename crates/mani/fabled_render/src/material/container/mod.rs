mod blending;
mod illumination_model;
mod pbr_standard;
mod rma;
mod standard;
mod tex_option;

pub use blending::*;
pub use illumination_model::*;
pub use pbr_standard::*;
pub use rma::*;
pub use standard::*;
pub use tex_option::*;

#[cfg(test)]
mod data_test {
    use crate::material::{
        IlluminationModel, PBRStandardMaterial, StandardMaterial, SupportRMA, TextureBlending,
        TextureOptions,
    };

    #[test]
    fn data_size() {
        let blending_size = std::mem::size_of::<TextureBlending>();
        assert_eq!(blending_size & (blending_size - 1), 0);

        let illum_model_size = std::mem::size_of::<IlluminationModel>();
        assert_eq!(illum_model_size & (illum_model_size - 1), 0);

        let rma_size = std::mem::size_of::<SupportRMA>();
        assert_eq!(rma_size & (rma_size - 1), 0);

        let standard_mat_size = std::mem::size_of::<StandardMaterial>();
        assert_eq!(standard_mat_size & (standard_mat_size - 1), 0);

        let pbr_standard_mat_size = std::mem::size_of::<PBRStandardMaterial>();
        assert_eq!(pbr_standard_mat_size & (pbr_standard_mat_size - 1), 0);

        let texture_options_size = std::mem::size_of::<TextureOptions>();
        println!("{}", texture_options_size);
    }

    #[test]
    fn data_alignment() {
        let blending_alignment = std::mem::align_of::<TextureBlending>();
        assert_eq!(blending_alignment & (blending_alignment - 1), 0);

        let illum_model_alignment = std::mem::align_of::<IlluminationModel>();
        assert_eq!(illum_model_alignment & (illum_model_alignment - 1), 0);

        let rma_alignment = std::mem::align_of::<SupportRMA>();
        assert_eq!(rma_alignment & (rma_alignment - 1), 0);

        let standard_mat_alignment = std::mem::align_of::<StandardMaterial>();
        assert_eq!(standard_mat_alignment & (standard_mat_alignment - 1), 0);

        let pbr_standard_mat_alignment = std::mem::align_of::<PBRStandardMaterial>();
        assert_eq!(
            pbr_standard_mat_alignment & (pbr_standard_mat_alignment - 1),
            0
        );

        let texture_options_alignment = std::mem::align_of::<TextureOptions>();
        assert_eq!(
            texture_options_alignment & (texture_options_alignment - 1),
            0
        );
    }
}
