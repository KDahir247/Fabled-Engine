use crate::mesh::{Mesh, Model, Vertex};

/*
    Planning phase.
    R major radius (the distance from the center of the tube to the center of the torus)
    r minor radius (the radius of the tube)
    //tessellation segment,
    //tessellation tubes,
*/

pub struct Torus {
    /// The distance from the center of the tube to the center of the torus.
    major_radius: f32,
    /// The radius of the tube.
    minor_radius: f32,
    tessellation_segment: usize,
    tessellation_side: usize,
}

impl Default for Torus {
    fn default() -> Self {
        Self::new(0.7, 0.3, 32, 12)
    }
}

impl Torus {
    pub fn new(
        major_radius: f32,
        minor_radius: f32,
        tessellation_segment: usize,
        tessellation_side: usize,
    ) -> Torus {
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

        let total_vertices = tessellation_segment * tessellation_side;

        let total_indices = (total_vertices << 1) * 3;

        let mut vertex_buffer = Vec::with_capacity(total_vertices);
        let mut indices = vec![0; total_indices];

        let inv_segment = 1.0 / tessellation_segment as f32;
        let inv_side = 1.0 / tessellation_side as f32;

        for seg in 0..tessellation_segment {
            // Find next (or first) segment offset
            let next_segment = (seg + 1) % tessellation_segment;

            let current_side_offset = seg * tessellation_side;
            let next_side_offset = next_segment * tessellation_side;

            for side in 0..tessellation_side {
                // Find next (or first) vertex offset.
                let next_side = (side + 1) % tessellation_side;

                // Finding the 4 vertices that makes up the four corner of a quad.
                let i1 = current_side_offset + side;
                let i2 = current_side_offset + next_side;
                let i3 = next_side_offset + next_side;
                let i4 = next_side_offset + side;

                // Calculate the UV texture coordinates.
                let u_tex = seg as f32 * inv_segment;
                let v_tex = side as f32 * inv_side;

                // Calculate the Vertex position.
                let (phi_sin, phi_cos) = (u_tex * std::f32::consts::TAU).sin_cos();
                let (theta_sin, theta_cos) = (v_tex * std::f32::consts::TAU).sin_cos();

                let x_pos = (major_radius + minor_radius * theta_cos) * phi_cos;
                let y_pos = minor_radius * theta_sin;
                let z_pos = (major_radius + minor_radius * theta_cos) * phi_sin;

                // Calculate the tangent with respect to the outer circle.
                let tangents_outer = glam::Vec3A::new(-phi_sin, phi_cos, 0.0);

                // Calculate the tangent with respect to the inner circle.
                let tangents_inner =
                    glam::Vec3A::new(phi_cos * -theta_sin, phi_sin * -theta_sin, theta_cos);

                // Normal is the cross-product of the tangents.
                let normal = tangents_outer.cross(tangents_inner).normalize();

                vertex_buffer.push(Vertex {
                    position: [x_pos, y_pos, z_pos],
                    tex_coord: [u_tex, v_tex],
                    normal: normal.to_array(),
                    tangent: [0.0; 4],
                    bi_tangent: [0.0; 4],
                });

                // Store the indices
                indices[i1 * 6] = i1;
                indices[i1 * 6 + 1] = i2;
                indices[i1 * 6 + 2] = i3;

                indices[i1 * 6 + 3] = i3;
                indices[i1 * 6 + 4] = i4;
                indices[i1 * 6 + 5] = i1;
            }
        }

        let mesh = Mesh {
            vertices: vertex_buffer,
            material_id: 0,
            indices,
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
        let torus = Torus::default();
        let torus_model: Model = torus.into();

        for vertex in &torus_model.meshes[0].vertices {
            println!(
                "new Vector3({}f, {}f, {}f),",
                vertex.position[0], vertex.position[1], vertex.position[2]
            );
        }
        //println!("{:?}", torus_model.meshes[0].indices);
    }
}
