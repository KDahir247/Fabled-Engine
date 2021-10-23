use crate::mesh::{Mesh, Model, Vertex};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(align(16))]
pub struct Torus {
    pub tessellation_segment: usize,
    pub tessellation_side: usize,

    /// The distance from the center of the tube to the center of the torus.
    pub major_radius: f32,
    /// The radius of the tube.
    pub minor_radius: f32,
}

impl Default for Torus {
    fn default() -> Self {
        Self::new(1.0, 0.5, 32, 24)
    }
}

impl Torus {
    pub fn new(
        mut major_radius: f32,
        mut minor_radius: f32,
        tessellation_segment: usize,
        tessellation_side: usize,
    ) -> Torus {
        // Sanity Check
        major_radius = major_radius.max(0.1);
        minor_radius = minor_radius.max(0.1);

        Self {
            major_radius,
            minor_radius,
            tessellation_segment,
            tessellation_side,
        }
    }
}

impl From<Torus> for Model {
    fn from(torus: Torus) -> Self {
        let Torus {
            major_radius,
            minor_radius,
            tessellation_segment,
            tessellation_side,
        } = torus;

        let total_faces = tessellation_segment * tessellation_side;
        let total_vertices = (tessellation_segment + 1) * (tessellation_side + 1);

        let total_indices = (total_faces << 1) * 3;
        let mut vertex_buffer = Vec::with_capacity(total_vertices);
        let mut indices = vec![0; total_indices];

        let inv_segment = 1.0 / tessellation_segment as f32;
        let inv_side = 1.0 / tessellation_side as f32;

        for seg in 0..=tessellation_segment {
            for side in 0..=tessellation_side {
                // Calculate the UV texture coordinates.
                let u_tex = seg as f32 * inv_segment;
                let v_tex = side as f32 * inv_side;

                // Calculate the Vertex position.
                let (phi_sin, phi_cos) = (u_tex * std::f32::consts::TAU).sin_cos();
                let (theta_sin, theta_cos) = (v_tex * std::f32::consts::TAU).sin_cos();

                let position = glam::Vec3A::new(
                    (major_radius + minor_radius * theta_cos) * phi_cos,
                    minor_radius * theta_sin,
                    (major_radius + minor_radius * theta_cos) * phi_sin,
                );

                // Calculate the Normal
                let normal = glam::Vec3A::new(phi_cos * theta_cos, theta_sin, phi_sin * theta_cos)
                    .normalize();

                vertex_buffer.push(Vertex {
                    position: position.to_array(),
                    tex_coord: [u_tex, v_tex],
                    normal: normal.to_array(),
                    tangent: [0.0; 4],
                    bi_tangent: [0.0; 4],
                });
            }
        }

        for seg in 0..tessellation_segment {
            let current_side_offset = seg * tessellation_side;
            let next_side_offset = (seg + 1) * tessellation_side;

            for side in 0..tessellation_side {
                // Finding the 4 vertices that makes up the four corner of a quad.
                let i1 = current_side_offset + side;
                let i2 = current_side_offset + side + 1;
                let i3 = next_side_offset + side + 1;
                let i4 = next_side_offset + side + 2;

                let face_indices = [i1 + seg, i2 + seg, i3 + seg, i2 + seg, i4 + seg, i3 + seg];

                {
                    // memcpy face_indices to 6 * i1 element slice of indices.
                    let offset = i1 * 6;
                    let (target_left, _) = indices[offset..].split_at_mut(6);
                    target_left.copy_from_slice(&face_indices);
                }
            }
        }

        let mesh = Mesh {
            vertices: vertex_buffer,
            material_id: 0,
            indices: indices.into(),
        };

        Model { meshes: vec![mesh] }
    }
}

#[cfg(test)]
mod test {
    use crate::mesh::primitive::torus::Torus;
    use crate::mesh::Model;

    #[test]
    fn test() {
        let torus = Torus {
            major_radius: 1.0,
            minor_radius: 0.5,
            tessellation_segment: 32,
            tessellation_side: 24,
        };
        let torus_model: Model = torus.into();

        for vertex in &torus_model.meshes[0].vertices {
            println!(
                "new Vector3({}f, {}f, {}f),",
                vertex.normal[0], vertex.normal[1], vertex.normal[2]
            );
        }
        println!("{:?}", torus_model.meshes[0].indices);
    }
}
