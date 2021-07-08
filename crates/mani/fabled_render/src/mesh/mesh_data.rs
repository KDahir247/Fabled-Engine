use crate::mesh::Vertex;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub material_id: u32,
    pub indices: u32,
}
