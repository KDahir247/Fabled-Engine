use crate::mesh::container::Vertex;
use crate::mesh::Indices;

#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Indices,
}
