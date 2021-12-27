pub use indices::*;
pub use mesh_data::*;
pub use model::*;
pub use vertex::*;

mod indices;
mod mesh_data;
mod model;
mod vertex;

#[cfg(test)]
mod data_test {
    use crate::mesh::{Indices, Mesh, Model, Vertex};

    #[test]
    fn data_size() {
        let mesh_data = std::mem::size_of::<Mesh>();
        assert_eq!(mesh_data & (mesh_data - 1), 0);

        let model = std::mem::size_of::<Model>();
        assert_eq!(model & (model - 1), 0);

        let vertex = std::mem::size_of::<Vertex>();
        assert_eq!(vertex & (vertex - 1), 0);

        let indices = std::mem::size_of::<Indices>();
        assert_eq!(indices & (indices - 1), 0);
    }

    #[test]
    fn align_size() {
        let mesh_data = std::mem::align_of::<Mesh>();
        assert_eq!(mesh_data, 16);

        let model = std::mem::align_of::<Model>();
        assert_eq!(model, 16);

        let vertex = std::mem::align_of::<Vertex>();
        assert_eq!(vertex, 16);

        let indices = std::mem::align_of::<Indices>();
        assert_eq!(indices, 8);
    }
}
