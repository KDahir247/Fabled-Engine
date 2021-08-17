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
        glam::const_vec3a!([-1.0, 0.0, 0.0]), // Back
        glam::const_vec3a!([0.0, 0.0, -1.0]), // Right
        glam::const_vec3a!([0.0, 0.0, 1.0]),  // Left
        glam::const_vec3a!([-1.0, 0.0, 0.0]), // Up
        glam::const_vec3a!([-1.0, 0.0, 0.0]), // Down
    ],
    bi_tangent: [
        glam::const_vec3a!([0.0, -1.0, 0.0]), // Front
        glam::const_vec3a!([0.0, 1.0, 0.0]),  // Back
        glam::const_vec3a!([0.0, 1.0, 0.0]),  // Right
        glam::const_vec3a!([0.0, 1.0, 0.0]),  // Left
        glam::const_vec3a!([0.0, 0.0, 1.0]),  // Up
        glam::const_vec3a!([0.0, 0.0, -1.0]), // Down
    ],
};

#[derive(Debug, Copy, Clone)]
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
        const CUBE_DATA: &CubeData = &CUBE_FACE_DATA;

        let mut vertex_storage = vec![Vertex::default(); 24];
        let mut indices_storage = vec![0; 36];

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

            let indices = [
                chunk * 3,
                chunk * 3 + 1,
                chunk * 3 + 2,
                chunk * 3 + 2,
                chunk * 3 + 3,
                chunk * 3,
            ];

            {
                let offset = chunk * 6;
                let (target_left, _) = indices_storage[offset..].split_at_mut(6);
                target_left.copy_from_slice(&indices);
            }

            for (index, face) in corners.chunks_exact(4).enumerate() {
                let vert_1 = face[0];
                let vert_2 = face[1];
                let vert_3 = face[2];
                let vert_4 = face[3];

                let tex_coord_1 = [vert_1.signum().x * 0.5 + 0.5, vert_1.signum().y * 0.5 + 0.5];
                let tex_coord_2 = [vert_2.signum().x * 0.5 + 0.5, vert_2.signum().y * 0.5 + 0.5];
                let tex_coord_3 = [vert_3.signum().x * 0.5 + 0.5, vert_3.signum().y * 0.5 + 0.5];
                let tex_coord_4 = [vert_4.signum().x * 0.5 + 0.5, vert_4.signum().y * 0.5 + 0.5];

                let first_vertex_chunk = Vertex {
                    position: vert_1.to_array(),
                    tex_coord: tex_coord_1,
                    normal: normal.to_array(),
                    tangent: [0.0; 4],
                    bi_tangent: [0.0; 4],
                };

                let second_vertex_chunk = Vertex {
                    position: vert_2.to_array(),
                    tex_coord: tex_coord_2,
                    normal: normal.to_array(),
                    tangent: [0.0; 4],
                    bi_tangent: [0.0; 4],
                };

                let third_vertex_chunk = Vertex {
                    position: vert_3.to_array(),
                    tex_coord: tex_coord_3,
                    normal: normal.to_array(),
                    tangent: [0.0; 4],
                    bi_tangent: [0.0; 4],
                };

                let fourth_vertex_chunk = Vertex {
                    position: vert_4.to_array(),
                    tex_coord: tex_coord_4,
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
                    let (target_left, _) = vertex_storage[offset..].split_at_mut(4);
                    target_left.copy_from_slice(&face_vertices);
                }
            }
        }

        let mesh = Mesh {
            vertices: vertex_storage,
            material_id: 0,
            indices: indices_storage,
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
        let cube = Cube::new(0.7);
        let cube_model: Model = cube.into();
        for mesh in &cube_model.meshes[0].vertices {
            println!("{:?}", mesh);
        }
    }
}
