use crate::mesh::Indices::U32;
use crate::mesh::{Indices, Mesh, Vertex};
use fabled_math::Vector3;
use smallvec::{SmallVec, ToSmallVec};

struct CubeData {
    pub normal: [Vector3; 6],
    pub tangent: [Vector3; 6],
    pub bi_tangent: [Vector3; 6],
}

const CUBE_FACE_DATA: CubeData = CubeData {
    normal: [
        Vector3::set(0.0, 0.0, 1.0),  // Front
        Vector3::set(0.0, 0.0, -1.0), // Back
        Vector3::set(1.0, 0.0, 0.0),  // Right
        Vector3::set(-1.0, 0.0, 0.0), // Left,
        Vector3::set(0.0, 1.0, 0.0),  // UP
        Vector3::set(0.0, -1.0, 0.0), // Down
    ],
    tangent: [
        Vector3::set(-1.0, 0.0, 0.0), // Front
        Vector3::set(1.0, 0.0, 0.0),  // Back
        Vector3::set(0.0, 0.0, 1.0),  // Right
        Vector3::set(0.0, 0.0, -1.0), // Left
        Vector3::set(1.0, 0.0, 0.0),  // Up
        Vector3::set(-1.0, 0.0, 0.0), // Down
    ],
    bi_tangent: [
        Vector3::set(0.0, 1.0, 0.0), // Front
        Vector3::set(0.0, 1.0, 0.0), // Back
        Vector3::set(0.0, 1.0, 0.0), // Right,
        Vector3::set(0.0, 1.0, 0.0), // Left
        Vector3::set(0.0, 0.0, 1.0), // UP
        Vector3::set(0.0, 0.0, 1.0), // Down
    ],
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cube {
    pub size: f32,
}

impl Default for Cube {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl Cube {
    pub fn new(size: f32) -> Cube {
        Self { size }
    }
}

impl From<Cube> for Mesh {
    #[inline(always)]
    fn from(cube: Cube) -> Self {
        const CUBE_DATA: CubeData = CUBE_FACE_DATA;

        const DEFAULT_VERTEX: Vertex = Vertex::init();

        let mut vertices = [DEFAULT_VERTEX; 24];
        let mut indices = [0; 36];

        for face_index in 0..6 {
            let face_index_u32: u32 = face_index as u32;

            let normal = CUBE_DATA.normal[face_index];
            let tangent = CUBE_DATA.tangent[face_index];
            let bi_tangent = CUBE_DATA.bi_tangent[face_index];

            {
                let intermediate_indices_step = face_index_u32 << 2;

                let indices_face = [
                    intermediate_indices_step + 2,
                    intermediate_indices_step + 1,
                    intermediate_indices_step,
                    intermediate_indices_step,
                    intermediate_indices_step + 3,
                    intermediate_indices_step + 2,
                ];

                let offset = face_index * 6;
                let (target_left, _) = indices[offset..].split_array_mut::<6>();
                target_left.copy_from_slice(&indices_face);
            }

            let normal_primitive = normal.to_primitive();

            let first_vertex_chunk = Vertex {
                position: ((normal - bi_tangent - tangent) * cube.size).to_primitive(),
                tex_coord: [1.0, 0.0],
                normal: normal_primitive,
                tangent: [0.0; 4],
                bi_tangent: [0.0; 4],
            };

            let second_vertex_chunk = Vertex {
                position: ((normal - bi_tangent + tangent) * cube.size).to_primitive(),
                tex_coord: [1.0, 1.0],
                normal: normal_primitive,
                tangent: [0.0; 4],
                bi_tangent: [0.0; 4],
            };

            let third_vertex_chunk = Vertex {
                position: ((normal + bi_tangent + tangent) * cube.size).to_primitive(),
                tex_coord: [0.0, 1.0],
                normal: normal_primitive,
                tangent: [0.0; 4],
                bi_tangent: [0.0; 4],
            };

            let fourth_vertex_chunk = Vertex {
                position: ((normal + bi_tangent - tangent) * cube.size).to_primitive(),
                tex_coord: [0.0, 0.0],
                normal: normal_primitive,
                tangent: [0.0; 4],
                bi_tangent: [0.0; 4],
            };

            {
                let face_vertices = [
                    first_vertex_chunk,
                    second_vertex_chunk,
                    third_vertex_chunk,
                    fourth_vertex_chunk,
                ];

                let offset = face_index << 2;
                let (target_left, _) = vertices[offset..].split_array_mut::<4>();
                target_left.copy_from_slice(&face_vertices);
            }
        }

        Mesh {
            vertices: vertices.to_vec(),
            indices: U32(indices.to_smallvec()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::mesh::primitive::cube::Cube;
    use crate::mesh::Mesh;

    #[test]
    fn test() {
        let cube = Cube::new(1.0);
        let cube_model: Mesh = cube.into();
        for vertex in &cube_model.vertices {
            println!(
                "new Vector3({}f, {}f, {}f),",
                vertex.position[0], vertex.position[1], vertex.position[2]
            );
        }
        println!("{:?}", cube_model.indices);
        println!("{:?}", cube_model.vertices.len());
    }
}
