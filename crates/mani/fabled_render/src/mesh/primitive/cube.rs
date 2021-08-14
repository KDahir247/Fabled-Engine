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

        let mut container = Vec::with_capacity(24);
        let mut face_vert_storage = [Vertex::default(); 4];

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

            let indices = vec![
                chunk * 3,
                chunk * 3 + 1,
                chunk * 3 + 2,
                chunk * 3 + 2,
                chunk * 3 + 3,
                chunk * 3,
            ];

            let normal_result = normal.to_array();
            let tangent_result = tangent.extend(1.0).to_array();
            let bi_tangent_result = bi_tangent.extend(1.0).to_array();

            for face in corners.chunks_exact(4) {
                let vert_1 = face[0];
                let vert_2 = face[1];
                let vert_3 = face[2];
                let vert_4 = face[3];

                let tex_coord_1 = [vert_1.signum().x * 0.5 + 0.5, vert_1.signum().y * 0.5 + 0.5];
                let tex_coord_2 = [vert_2.signum().x * 0.5 + 0.5, vert_2.signum().y * 0.5 + 0.5];
                let tex_coord_3 = [vert_3.signum().x * 0.5 + 0.5, vert_3.signum().y * 0.5 + 0.5];
                let tex_coord_4 = [vert_4.signum().x * 0.5 + 0.5, vert_4.signum().y * 0.5 + 0.5];

                face_vert_storage[0] = Vertex {
                    position: vert_1.to_array(),
                    tex_coord: tex_coord_1,
                    normal: normal_result,
                    tangent: tangent_result,
                    bi_tangent: bi_tangent_result,
                };

                face_vert_storage[1] = Vertex {
                    position: vert_2.to_array(),
                    tex_coord: tex_coord_2,
                    normal: normal_result,
                    tangent: tangent_result,
                    bi_tangent: bi_tangent_result,
                };

                face_vert_storage[2] = Vertex {
                    position: vert_3.to_array(),
                    tex_coord: tex_coord_3,
                    normal: normal_result,
                    tangent: tangent_result,
                    bi_tangent: bi_tangent_result,
                };

                face_vert_storage[3] = Vertex {
                    position: vert_4.to_array(),
                    tex_coord: tex_coord_4,
                    normal: normal_result,
                    tangent: tangent_result,
                    bi_tangent: bi_tangent_result,
                };
            }

            container.push(Mesh {
                vertices: face_vert_storage.to_vec(),
                material_id: 0,
                indices,
            });
        }

        Model { meshes: container }
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
        for mesh in cube_model.meshes {
            println!("{:?}", mesh.vertices);
        }
    }
}
