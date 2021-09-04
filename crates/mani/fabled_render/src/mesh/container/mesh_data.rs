use crate::mesh::container::Vertex;

#[derive(Debug)]
#[repr(C, align(16))]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub material_id: u32,
    pub indices: Vec<usize>, // todo convert this to indices
}
