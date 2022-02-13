mod geometry_mesh;
mod geometry_mesh_index;
mod material;
mod material_index;
mod mesh;
mod mesh_index;
mod scene;
mod shading_model;
mod texture_index;

pub use geometry_mesh::*;
pub use geometry_mesh_index::*;
pub use material::*;
pub use material_index::*;
pub use mesh::*;
pub use mesh_index::*;
pub use scene::*;
pub use shading_model::*;
pub use texture_index::*;

#[cfg(test)]
mod data_test {
    use crate::MaterialData;

    #[test]
    fn data_size() {
        let full_data_size = std::mem::size_of::<MaterialData>();
        println!("{}", full_data_size);
    }


    #[test]
    fn data_alignment() {
        let full_data_alignment = std::mem::align_of::<MaterialData>();
        println!("{}", full_data_alignment);
    }
}
