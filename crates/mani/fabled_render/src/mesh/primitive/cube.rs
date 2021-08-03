use crate::mesh::{Mesh, Model, Vertex};

#[repr(C)]
#[derive(Debug)]
pub struct CubeData {
    pub normal: [glam::Vec3A; 6],
    pub tangent: [glam::Vec3A; 6],
    pub bi_tangent: [glam::Vec3A; 6],
}

const CUBE_DATA: CubeData = CubeData {
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
        let cube_data = &CUBE_DATA;

        let mut container = Vec::with_capacity(24);

        for index in 0..6 {
            let normal = cube_data.normal[index];
            let tangent = cube_data.tangent[index];
            let bi_tangent = cube_data.bi_tangent[index];

            let corner_0 = (normal - bi_tangent - tangent) * size;
            let corner_1 = (normal - bi_tangent + tangent) * size;
            let corner_2 = (normal + bi_tangent + tangent) * size;
            let corner_3 = (normal + bi_tangent - tangent) * size;

            let indices = [
                index * 3,
                index * 3 + 1,
                index * 3 + 2,
                index * 3 + 2,
                index * 3 + 3,
                index * 3,
            ];

            let normal_result = normal.to_array();
            let tangent_result = tangent.extend(1.0).to_array();
            let bi_tangent_result = bi_tangent.extend(1.0).to_array();

            let tex_coord_0 = [(corner_0.x + 1.) * 0.5, (corner_0.y + 1.) * 0.5];
            let tex_coord_1 = [(corner_1.x + 1.) * 0.5, (corner_1.y + 1.) * 0.5];
            let tex_coord_2 = [(corner_2.x + 1.) * 0.5, (corner_2.y + 1.) * 0.5];
            let tex_coord_3 = [(corner_3.x + 1.) * 0.5, (corner_3.y + 1.) * 0.5];

            //0,1,2,2,3,0
            container.push(Mesh {
                vertices: vec![
                    Vertex {
                        position: corner_0.to_array(),
                        tex_coord: tex_coord_0,
                        normal: normal_result,
                        tangent: tangent_result,
                        bi_tangent: bi_tangent_result,
                    },
                    Vertex {
                        position: corner_1.to_array(),
                        tex_coord: tex_coord_1,
                        normal: normal_result,
                        tangent: tangent_result,
                        bi_tangent: bi_tangent_result,
                    },
                    Vertex {
                        position: corner_2.to_array(),
                        tex_coord: tex_coord_2,
                        normal: normal_result,
                        tangent: tangent_result,
                        bi_tangent: bi_tangent_result,
                    },
                    Vertex {
                        position: corner_3.to_array(),
                        tex_coord: tex_coord_3,
                        normal: normal_result,
                        tangent: tangent_result,
                        bi_tangent: bi_tangent_result,
                    },
                ],
                material_id: 0,
                indices: indices.to_vec(),
            });
        }

        println!("{:?}", container);
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
        Cube::new(0.3);
    }
}
