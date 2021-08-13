use crate::mesh::{Mesh, Model, Vertex};
#[allow(dead_code)]
const VERTEX: [f32; 12] = [
    0.5, -0.5, 0.0, //Bottom Right
    -0.5, 0.5, 0.0, // Top Left
    -0.5, -0.5, 0.0, // Bottom Left
    0.5, 0.5, 0.0, //Top Right
];

#[allow(dead_code)]
const NORMAL: [f32; 3] = [
    0.0, 0.0, 1.0, //Forward
];
const TANGENT: [f32; 4] = [-1.0, 0.0, 0.0, 1.0]; // Left
const BI_TANGENT: [f32; 4] = [0.0, -1.0, 0.0, 1.0]; // Down
const INDICES: [usize; 6] = [0, 1, 2, 0, 3, 2];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quad {
    pub width: f32,
    pub height: f32,
}

impl Default for Quad {
    fn default() -> Self {
        Self::new(1.0, 1.0)
    }
}

impl Quad {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl From<Quad> for Model {
    fn from(quad: Quad) -> Self {
        let mut vertices = Vec::with_capacity(4);
        let mut vertex = &VERTEX[0..12];

        while vertex.len() >= 3 {
            let x = vertex[0];
            let y = vertex[1];
            let z = vertex[2];

            vertices.push(Vertex {
                position: [x * quad.width, y * quad.height, z],
                tex_coord: [x + 0.5, y + 0.5],
                normal: NORMAL,
                tangent: TANGENT,
                bi_tangent: BI_TANGENT,
            });

            vertex = &vertex[3..];
        }

        let mesh = Mesh {
            vertices,
            material_id: 0,
            indices: INDICES.to_vec(),
        };

        Model { meshes: vec![mesh] }
    }
}

#[cfg(test)]
mod test {
    use crate::mesh::primitive::quad::Quad;
    use crate::mesh::Model;

    #[test]
    fn test() {
        let quad = Quad::new(1.0, 1.0);
        let quad_model: Model = quad.into();
        for mesh in &quad_model.meshes {
            println!("{:?}", mesh.vertices);
        }
        println!("{:?}", quad_model.meshes[0].indices);
    }
}
