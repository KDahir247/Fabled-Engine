use crate::mesh::{Mesh, Model, Vertex};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cone {
    pub apex_position: [f32; 3],
    pub tessellation_slice: usize,
    pub radius: f32,
    pub height: f32,
}

impl Default for Cone {
    fn default() -> Self {
        Self::new(5.0, 16, 2., [0., 2., 0.])
    }
}

impl Cone {
    pub fn new(
        radius: f32,
        mut tessellation_slice: usize,
        height: f32,
        apex_position: [f32; 3],
    ) -> Cone {
        // We can't technically have a cone with less than three tessellation slice for
        // the base. right?
        tessellation_slice = tessellation_slice.max(3);
        Self {
            radius,
            tessellation_slice,
            height,
            apex_position,
        }
    }
}

impl From<Cone> for Model {
    fn from(cone: Cone) -> Self {
        let Cone {
            radius,
            tessellation_slice,
            height,
            apex_position,
        } = cone;

        let mut indices = vec![0_usize; tessellation_slice * 6];
        let mut vertices: Vec<Vertex> = Vec::with_capacity(tessellation_slice + 2);

        let apex_position = glam::const_vec3a!(apex_position);

        let forward_dir = glam::Vec3A::X.cross(apex_position.normalize());
        let center = apex_position + (-apex_position.normalize() * height);

        let angle_inc = 2.0 * std::f32::consts::PI / tessellation_slice as f32;

        // Apex Vertex
        vertices.push(Vertex {
            position: apex_position.to_array(),
            tex_coord: [0.0, 1.0],
            normal: [0.0, 1.0, 0.0],
            tangent: [-1.0, 0.0, 0.0, 1.0],
            bi_tangent: [0.0, 0.0, 1.0, 1.0],
        });

        // Base Center Vertex
        vertices.push(Vertex {
            position: center.to_array(),
            tex_coord: [0.0, -1.0],
            normal: [0.0, -1.0, 0.0],
            tangent: [-1.0, 0.0, 0.0, 1.0],
            bi_tangent: [0.0, 0.0, -1.0, 1.0],
        });


        for side in 0..=tessellation_slice {
            let (rad_sin, rad_cos) = (angle_inc * side as f32).sin_cos();

            let vertex = center + (glam::Vec3A::X * rad_cos + forward_dir * rad_sin) * radius;

            let slant_height_vector = glam::Vec3A::Y - vertex;

            let vertex_direction = vertex.normalize();

            let tangent = glam::Vec3A::new(-vertex_direction.z, 0.0, vertex_direction.x);
            let normal = slant_height_vector.cross(tangent).normalize();

            vertices.push(Vertex {
                position: [vertex.x, vertex.y, vertex.z],
                tex_coord: [side as f32 / tessellation_slice as f32, 0.0],
                normal: [normal.x, normal.y, normal.z],
                tangent: [0.0; 4],
                bi_tangent: [0.0; 4],
            });
        }

        // indices
        for point in 2..tessellation_slice + 2 {
            let face_indices = [0, point + 1, point, 1, point, point + 1];

            let start_index = (point - 2) * 6;
            let end_index = (point - 2) * 6 + 6;
            indices[start_index..end_index].copy_from_slice(&face_indices);
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
    use crate::mesh::primitive::cone::Cone;
    use crate::mesh::Model;

    #[test]
    fn test() {
        let cone = Cone::new(1.0, 64, 2., [0.0, 1.0, 0.0]);
        let cone_model: Model = cone.into();
        for vertex in &cone_model.meshes[0].vertices {
            println!(
                "new Vector3({}f, {}f, {}f),",
                vertex.normal[0], vertex.normal[1], vertex.normal[2]
            );
        }
        println!("{:?}", cone_model.meshes[0].indices);
    }
}
