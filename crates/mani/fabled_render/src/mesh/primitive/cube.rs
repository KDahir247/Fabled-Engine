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

#[derive(Debug)]
pub struct Cube {
    #[allow(dead_code)]
    model: Model,
}

impl Default for Cube {
    fn default() -> Self {
        Self::new(0.5)
    }
}

impl Cube {
    pub fn new(size: f32) -> Cube {
        const CUBE_DATA: &CubeData = &CUBE_FACE_DATA;

        let mut container = Vec::with_capacity(24);
        let mut temp_vert_storage = [Vertex::default(); 4];
        for chunk in 0..6 {
            let normal = CUBE_DATA.normal[chunk];
            let tangent = CUBE_DATA.tangent[chunk];
            let bi_tangent = CUBE_DATA.bi_tangent[chunk];

            let corners = [
                (normal - bi_tangent - tangent) * size,
                (normal - bi_tangent + tangent) * size,
                (normal + bi_tangent + tangent) * size,
                (normal + bi_tangent - tangent) * size,
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

            for (index, corner) in corners.iter().enumerate() {
                let tex_coord = [corner.x.signum() * 0.5 + 0.5, corner.y.signum() * 0.5 + 0.5];

                temp_vert_storage[index] = Vertex {
                    position: corner.to_array(),
                    tex_coord,
                    normal: normal_result,
                    tangent: tangent_result,
                    bi_tangent: bi_tangent_result,
                }
            }

            container.push(Mesh {
                vertices: temp_vert_storage.to_vec(),
                material_id: 0,
                indices,
            });
        }

        Cube {
            model: Model { meshes: container },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::mesh::primitive::cube::Cube;

    #[test]
    fn test() {
        Cube::new(0.7);
    }
}
