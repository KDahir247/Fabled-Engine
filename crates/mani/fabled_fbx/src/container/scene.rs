use crate::{GeometryMesh, Material, Mesh};

pub struct Scene<'a> {
    name: std::borrow::Cow<'a, str>,
    geometry_meshes: Vec<GeometryMesh<'a>>,
    materials: Vec<Material<'a>>,
    meshes: Vec<Mesh<'a>>,
}
