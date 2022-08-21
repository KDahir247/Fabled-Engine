use fabled_render::mesh::Mesh;

pub struct ModelMetadata {
    pub mtl_path: std::path::PathBuf,
    pub mesh: Mesh,
}
