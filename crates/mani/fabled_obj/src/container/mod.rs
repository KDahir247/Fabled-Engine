pub use illumination_model::*;
pub use imfchan::*;
pub use load_flag::*;
pub use material::*;
pub use material_metadata::*;
pub use model_metadata::*;

mod illumination_model;
mod imfchan;
mod load_flag;
mod material;
mod material_metadata;
mod model_metadata;

#[cfg(test)]
mod data_test {
    use crate::{IlluminationModel, LoadFlags, Material, MaterialMetadata, ModelMetadata, IMFCHAN};

    #[test]
    fn data_size() {
        let load_flag_size = std::mem::size_of::<LoadFlags>();
        assert_eq!(load_flag_size & (load_flag_size - 1), 0);

        let illum_model_size = std::mem::size_of::<IlluminationModel>();
        assert_eq!(illum_model_size & (illum_model_size - 1), 0);

        let model_metadata = std::mem::size_of::<ModelMetadata>();
        assert_eq!(model_metadata & (model_metadata - 1), 0);

        let material_size = std::mem::size_of::<Material>();
        println!("{}", material_size);

        let material_metadata_size = std::mem::size_of::<MaterialMetadata>();
        assert_eq!(material_metadata_size & (material_metadata_size - 1), 0);

        let imfchan_size = std::mem::size_of::<IMFCHAN>();
        assert_eq!(imfchan_size & (imfchan_size - 1), 0);
    }

    #[test]
    fn data_alignment() {
        let load_flag_alignment = std::mem::align_of::<LoadFlags>();
        assert_eq!(load_flag_alignment & (load_flag_alignment - 1), 0);

        let illum_model_alignment = std::mem::align_of::<IlluminationModel>();
        assert_eq!(illum_model_alignment & (illum_model_alignment - 1), 0);

        let model_metadata_alignment = std::mem::align_of::<ModelMetadata>();
        assert_eq!(model_metadata_alignment & (model_metadata_alignment - 1), 0);

        let material_alignment = std::mem::align_of::<Material>();
        assert_eq!(material_alignment & (material_alignment - 1), 0);

        let material_metadata_alignment = std::mem::align_of::<MaterialMetadata>();
        assert_eq!(
            material_metadata_alignment & (material_metadata_alignment - 1),
            0
        );

        let imfchan_alignment = std::mem::align_of::<IMFCHAN>();
        assert_eq!(imfchan_alignment & (imfchan_alignment - 1), 0);
    }
}
