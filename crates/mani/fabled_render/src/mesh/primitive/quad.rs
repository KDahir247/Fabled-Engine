use crate::mesh::{Mesh, Model, RenderInstruction, Vertex};

const NORMAL: [f32; 3] = [0.0, 0.0, 1.0];
const INDICES_SINGLE: [usize; 6] = [0, 2, 1, 0, 3, 2];
const INDICES_DOUBLE: [usize; 6] = [1, 2, 0, 2, 3, 0];

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(align(16))]
pub struct Quad {
    pub width: f32,
    pub height: f32,
    pub quad_instruction: RenderInstruction,
}

impl Default for Quad {
    fn default() -> Self {
        Self::new(1.0, 1.0, RenderInstruction::SingleSided)
    }
}

impl Quad {
    pub fn new(width: f32, height: f32, instruction: RenderInstruction) -> Self {
        Self {
            width,
            height,
            quad_instruction: instruction,
        }
    }
}

impl From<Quad> for Model {
    fn from(quad: Quad) -> Self {
        let mut vertices = [Vertex::default(); 4];

        let vertex = &mut [
            -0.5, -0.5, 0.0, -0.5, 0.5, 0.0, 0.5, 0.5, 0.0, 0.5, -0.5, 0.0,
        ];

        for (index, vert) in vertex.chunks_exact(3).enumerate() {
            let x = vert[0];
            let y = vert[1];

            vertices[index] = Vertex {
                position: [x * quad.width, y * quad.height, 0.0],
                tex_coord: [x + 0.5, y + 0.5],
                normal: NORMAL,
                tangent: [0.0; 4],
                bi_tangent: [0.0; 4],
            };
        }

        let mut indices = INDICES_SINGLE.to_vec();

        for d_index in 0..6 * quad.quad_instruction as usize {
            indices.push(INDICES_DOUBLE[d_index]);
        }

        let mesh = Mesh {
            vertices: vertices.to_vec(),
            material_id: 0,
            indices : indices.into(),
        };

        Model { meshes: vec![mesh] }
    }
}

#[cfg(test)]
mod test {
    use crate::mesh::primitive::quad::Quad;
    use crate::mesh::{Model, RenderInstruction};

    #[test]
    fn test() {
        let quad = Quad::new(3.0, 3.0, RenderInstruction::DoubleSided);
        let quad_model: Model = quad.into();
        for vertex in &quad_model.meshes[0].vertices {
            println!(
                "new Vector3({}f, {}f, {}f),",
                vertex.normal[0], vertex.normal[1], vertex.normal[2]
            );
        }
        println!("{:?}", quad_model.meshes[0].indices);
    }
}
