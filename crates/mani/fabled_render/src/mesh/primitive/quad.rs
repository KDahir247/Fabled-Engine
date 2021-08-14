use crate::mesh::{Mesh, Model, Vertex};

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
    pub depth: f32,
}

impl Default for Quad {
    fn default() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
}

impl Quad {
    pub fn new(width: f32, height: f32, depth: f32) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }
}

impl From<Quad> for Model {
    fn from(quad: Quad) -> Self {
        let mut vertices = [Vertex::default(); 4];
        let vertex = &mut [
            0.5, -0.5, 0.0, //Bottom Right
            -0.5, 0.5, 0.0, // Top Left
            -0.5, -0.5, 0.0, // Bottom Left
            0.5, 0.5, 0.0, //Top Right
        ];

        for (index, vert) in vertex.chunks_exact_mut(3).enumerate() {
            let x = vert[0];
            let y = vert[1];
            let z = vert[2];

            vert[0] *= quad.width;
            vert[1] *= quad.height;
            vert[2] *= quad.depth;

            vertices[index] = Vertex {
                position: [x, y, z],
                tex_coord: [x + 0.5, y + 0.5],
                normal: NORMAL,
                tangent: TANGENT,
                bi_tangent: BI_TANGENT,
            };
        }
        let mesh = Mesh {
            vertices: vertices.to_vec(),
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
        let quad = Quad::new(1.0, 1.0, 1.0);
        let quad_model: Model = quad.into();
        for mesh in &quad_model.meshes {
            println!("{:?}", mesh.vertices);
        }
        println!("{:?}", quad_model.meshes[0].indices);
    }
}
