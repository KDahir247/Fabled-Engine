pub struct GeometryMesh<'a> {
    pub name: std::borrow::Cow<'a, str>,
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub uvs: Vec<f32>,
    pub indices_per_material: Vec<Vec<u32>>,
}
