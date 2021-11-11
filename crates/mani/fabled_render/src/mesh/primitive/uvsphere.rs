use crate::mesh::{Mesh, Model, Vertex};

pub struct UvSphere {
    pub tessellation_stack: usize,
    pub tessellation_sector: usize,
    pub radius: f32,
}

impl Default for UvSphere {
    fn default() -> Self {
        Self::new(0.5, 32, 16)
    }
}

impl UvSphere {
    pub fn new(radius: f32, tessellation_stack: usize, tessellation_sector: usize) -> UvSphere {
        Self {
            radius,
            tessellation_stack,
            tessellation_sector,
        }
    }
}

impl From<UvSphere> for Model {
    fn from(uv_sphere: UvSphere) -> Self {
        // code adapted with modifications from https://behreajj.medium.com/making-a-capsule-mesh-via-script-in-five-3d-environments-c2214abf02db
        // provided by bevy. Thank you Bevy Community! \(ᵔᵕᵔ)/

        let vertex_size = uv_sphere.tessellation_stack * uv_sphere.tessellation_sector;
        let indices_size = (vertex_size << 1) * 3;

        let mut vertices: Vec<Vertex> = Vec::with_capacity(vertex_size);
        let mut indices: Vec<usize> = Vec::with_capacity(indices_size);

        let tessellation_sector = uv_sphere.tessellation_sector as f32;
        let tessellation_stack = uv_sphere.tessellation_stack as f32;

        let inv_length = 1.0 / uv_sphere.radius;
        let sector_step = std::f32::consts::TAU / tessellation_sector;
        let stack_step = std::f32::consts::PI / tessellation_stack;

        for i in 0..uv_sphere.tessellation_stack + 1 {
            let stack_angle = std::f32::consts::PI / 2.0 - i as f32 * stack_step;
            let xy = uv_sphere.radius * stack_angle.cos();
            let z = uv_sphere.radius * stack_angle.sin();

            for j in 0..uv_sphere.tessellation_sector + 1 {
                let sector_angle = j as f32 * sector_step;
                let x = xy * sector_angle.cos();
                let y = xy * sector_angle.sin();

                vertices.push(Vertex {
                    position: [x, y, z],
                    tex_coord: [
                        j as f32 / tessellation_sector,
                        i as f32 / tessellation_stack,
                    ],
                    normal: [x * inv_length, y * inv_length, z * inv_length],
                    tangent: [0.0; 4],
                    bi_tangent: [0.0; 4],
                });
            }
        }

        for i in 0..uv_sphere.tessellation_stack {
            let mut k1 = i * (uv_sphere.tessellation_sector + 1);
            let mut k2 = k1 + uv_sphere.tessellation_sector + 1;

            for _j in 0..uv_sphere.tessellation_sector {
                if i != 0 {
                    indices.push(k1);
                    indices.push(k2);
                    indices.push(k1 + 1);
                }

                if i != uv_sphere.tessellation_stack - 1 {
                    indices.push(k1 + 1);
                    indices.push(k2);
                    indices.push(k2 + 1);
                }

                k1 += 1;
                k2 += 1;
            }
        }

        let mesh = Mesh {
            vertices,
            material_id: 0,
            indices : indices.into(),
        };

        Model { meshes: vec![mesh] }
    }
}

#[cfg(test)]
mod test {
    use crate::mesh::primitive::uvsphere::UvSphere;
    use crate::mesh::Model;

    #[test]
    fn test() {
        let sphere = UvSphere::default();
        let sphere_model: Model = sphere.into();
        for vertex in &sphere_model.meshes[0].vertices {
            println!(
                "new Vector3({}f, {}f, {}f),",
                vertex.normal[0], vertex.normal[1], vertex.normal[2]
            );
        }
        println!("{:?}", sphere_model.meshes[0].indices);
    }
}
