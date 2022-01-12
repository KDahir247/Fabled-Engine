use crate::{GeometryMeshIndex, MaterialIndex};

pub struct Mesh<'a> {
    pub name: std::borrow::Cow<'a, str>,
    pub indices_per_material: Vec<MaterialIndex>,
    pub geometry: GeometryMeshIndex,
}
