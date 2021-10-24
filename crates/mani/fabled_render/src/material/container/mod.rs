pub use blending::*;
pub use material_param::*;
pub use material_type::*;
pub use pbr_material::*;
pub use rma::*;
pub use standard_material::*;
pub use tex_option::*;
pub use tex_type::*;

mod blending;
mod material_param;
mod material_type;
mod pbr_material;
mod rma;
mod standard_material;
mod tex_option;
mod tex_type;

#[cfg(test)]
mod data_test {
    use crate::material::{
        MaterialParameter, MaterialType, PBRStandardMaterial, StandardMaterial, SupportRMA,
        TextureBlending, TextureOptions, TextureType,
    };

    #[test]
    fn data_size() {
        let blending_size = std::mem::size_of::<TextureBlending>();
        assert_eq!(blending_size & (blending_size - 1), 0);

        let rma_size = std::mem::size_of::<SupportRMA>();
        assert_eq!(rma_size & (rma_size - 1), 0);

        let standard_mat_size = std::mem::size_of::<StandardMaterial>();
        println!("{}", standard_mat_size);

        let pbr_standard_mat_size = std::mem::size_of::<PBRStandardMaterial>();
        assert_eq!(pbr_standard_mat_size & (pbr_standard_mat_size - 1), 0);

        let texture_options_size = std::mem::size_of::<TextureOptions>();
        println!("{}", texture_options_size);

        let texture_ty_size = std::mem::size_of::<TextureType>();
        assert_eq!(texture_ty_size & (texture_ty_size - 1), 0);

        let material_type_size = std::mem::size_of::<MaterialType>();
        assert_eq!(material_type_size & (material_type_size - 1), 0);

        let material_param_size = std::mem::size_of::<MaterialParameter>();
        assert_eq!(material_param_size & (material_param_size - 1), 0);
    }

    #[test]
    fn data_alignment() {
        let blending_alignment = std::mem::align_of::<TextureBlending>();
        assert_eq!(blending_alignment & (blending_alignment - 1), 0);

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

        let texture_ty_alignment = std::mem::align_of::<TextureType>();
        assert_eq!(texture_ty_alignment & (texture_ty_alignment - 1), 0);

        let material_type_alignment = std::mem::align_of::<MaterialType>();
        assert_eq!(material_type_alignment & (material_type_alignment - 1), 0);

        let material_param_alignment = std::mem::align_of::<MaterialParameter>();
        assert_eq!(material_param_alignment & (material_param_alignment - 1), 0);
    }
}
