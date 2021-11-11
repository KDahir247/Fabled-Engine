use crate::mesh::{Mesh, Model, Vertex};

#[repr(C, align(16))]
#[derive(Debug)]
struct CubeData {
    pub normal: [glam::Vec3A; 6],
    pub tangent: [glam::Vec3A; 6],
    pub bi_tangent: [glam::Vec3A; 6],
}

const CUBE_FACE_DATA: CubeData = CubeData {
    normal: [
        glam::const_vec3a!([0.0, 0.0, 1.0]),  // Front
        glam::const_vec3a!([0.0, 0.0, -1.0]), // Back
        glam::const_vec3a!([1.0, 0.0, 0.0]),  // Right
        glam::const_vec3a!([-1.0, 0.0, 0.0]), // Left
        glam::const_vec3a!([0.0, 1.0, 0.0]),  // Up
        glam::const_vec3a!([0.0, -1.0, 0.0]), // Down
    ],
    tangent: [
        glam::const_vec3a!([-1.0, 0.0, 0.0]), // Front
        glam::const_vec3a!([1.0, 0.0, 0.0]),  // Back
        glam::const_vec3a!([0.0, 0.0, 1.0]),  // Right
        glam::const_vec3a!([0.0, 0.0, -1.0]), // Left
        glam::const_vec3a!([1.0, 0.0, 0.0]),  // Up
        glam::const_vec3a!([-1.0, 0.0, 0.0]), // Down
    ],
    bi_tangent: [
        glam::const_vec3a!([0.0, 1.0, 0.0]), // Front
        glam::const_vec3a!([0.0, 1.0, 0.0]), // Back
        glam::const_vec3a!([0.0, 1.0, 0.0]), // Right
        glam::const_vec3a!([0.0, 1.0, 0.0]), // Left
        glam::const_vec3a!([0.0, 0.0, 1.0]), // Up
        glam::const_vec3a!([0.0, 0.0, 1.0]), // Down
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

impl From<Cube> for Model {
    fn from(cube: Cube) -> Self {
        const CUBE_DATA: CubeData = CUBE_FACE_DATA;

        let mut vertices = vec![Vertex::default(); 24];
        let mut indices = vec![0; 36];

        for chunk in 0..6 {
            let normal = CUBE_DATA.normal[chunk];
            let tangent = CUBE_DATA.tangent[chunk];
            let bi_tangent = CUBE_DATA.bi_tangent[chunk];

            let corners = [
                (normal - bi_tangent - tangent) * cube.size,
                (normal - bi_tangent + tangent) * cube.size,
                (normal + bi_tangent + tangent) * cube.size,
                (normal + bi_tangent - tangent) * cube.size,
            ];

            let indices_face = [
                chunk * 4 + 2,
                chunk * 4 + 1,
                chunk * 4,
                chunk * 4,
                chunk * 4 + 3,
                chunk * 4 + 2,
            ];

            {
                let offset = chunk * 6;
                let (target_left, _) = indices[offset..].split_at_mut(6);
                target_left.copy_from_slice(&indices_face);
            }

            for (index, face) in corners.chunks_exact(4).enumerate() {
                assert!(face.len().eq(&4));

                let first_vertex_chunk = Vertex {
                    position: face[0].to_array(),
                    tex_coord: [1.0, 0.0],
                    normal: normal.to_array(),
                    tangent: [0.0; 4],
                    bi_tangent: [0.0; 4],
                };

                let second_vertex_chunk = Vertex {
                    position: face[1].to_array(),
                    tex_coord: [1.0, 1.0],
                    normal: normal.to_array(),
                    tangent: [0.0; 4],
                    bi_tangent: [0.0; 4],
                };

                let third_vertex_chunk = Vertex {
                    position: face[2].to_array(),
                    tex_coord: [0.0, 1.0],
                    normal: normal.to_array(),
                    tangent: [0.0; 4],
                    bi_tangent: [0.0; 4],
                };

                let fourth_vertex_chunk = Vertex {
                    position: face[3].to_array(),
                    tex_coord: [0.0, 0.0],
                    normal: normal.to_array(),
                    tangent: [0.0; 4],
                    bi_tangent: [0.0; 4],
                };

                let face_vertices = [
                    first_vertex_chunk,
                    second_vertex_chunk,
                    third_vertex_chunk,
                    fourth_vertex_chunk,
                ];

                {
                    let offset = (index + chunk) * 4;
                    let (target_left, _) = vertices[offset..].split_at_mut(4);
                    target_left.copy_from_slice(&face_vertices);
                }
            }
        }

        let mesh = Mesh {
            vertices,
            material_id: 0,
            indices: indices.into(),
        };

        Model { meshes: vec![mesh] }
    }
}

#[cfg(test)]
mod test {
    use crate::mesh::primitive::cube::Cube;
    use crate::mesh::Model;

    #[test]
    fn test() {
        let cube = Cube::new(1.0);
        let cube_model: Model = cube.into();
        for vertex in &cube_model.meshes[0].vertices {
            println!(
                "new Vector3({}f, {}f, {}f),",
                vertex.normal[0], vertex.normal[1], vertex.normal[2]
            );
        }
        println!("{:?}", cube_model.meshes[0].indices);
    }
}
