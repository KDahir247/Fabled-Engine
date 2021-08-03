use crate::mesh::container::Vertex;

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub material_id: u32,
    pub indices: Vec<usize>,
}
