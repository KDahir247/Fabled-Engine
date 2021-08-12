use crate::mesh::{Mesh, Model, Vertex};

const TANGENT: [f32; 4] = [-1.0, 0.0, 0.0, 1.0];

// The normal vector of the front face plane
const NORMAL_FRONT: [f32; 3] = [0.0, 1.0, 0.0];

const BI_TANGENT: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
/*
    Planning phase.
   Assumptions for the 3D Plane model.
   1) the Orientation is Horizontal, since we want it flat on the grid plane,
   2) the Pivot point is on the center of the plane.
   3) the normal vector is always the front plane and not the back plane even if double sided.
*/
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaneInstruction {
    SingleSided = 0,
    DoubleSided = 1,
}

impl Default for PlaneInstruction {
    fn default() -> Self {
        PlaneInstruction::SingleSided
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Plane {
    pub width: f32,
    pub height: f32,
    pub tessellation_width: usize,
    pub tessellation_height: usize,
    pub plane_instruction: PlaneInstruction,
}

impl Default for Plane {
    fn default() -> Self {
        Self::new(1.0, 1.0, 1, 1, PlaneInstruction::default())
    }
}

impl Plane {
    pub fn new(
        width: f32,
        height: f32,
        tessellation_width: usize,
        tessellation_height: usize,
        instruction: PlaneInstruction,
    ) -> Plane {
        Self {
            width,
            height,
            tessellation_width,
            tessellation_height,
            plane_instruction: instruction,
        }
    }
}

impl From<Plane> for Model {
    fn from(plane: Plane) -> Self {
        let Plane {
            width,
            height,
            tessellation_width,
            tessellation_height,
            plane_instruction,
        } = plane;

        let w_line = tessellation_width + 1;
        let h_line = tessellation_height + 1;

        // remap plane_instruction to SingleSided = 1, DoubleSided = 2,
        let double_sided_remap = plane_instruction as usize + 1;
        let num_triangles = tessellation_width * tessellation_height * 6 * double_sided_remap;
        let num_vertices = w_line * h_line;

        let mut vertices: Vec<Vertex> = Vec::with_capacity(num_vertices);
        let mut indices: Vec<usize> = Vec::with_capacity(num_triangles);

        let inv_tessellation_width = 1.0 / tessellation_width as f32;
        let inv_tessellation_height = 1.0 / tessellation_height as f32;
        let scale_x = width * inv_tessellation_width;
        let scale_y = height * inv_tessellation_height;

        for y in 0..h_line {
            let yf = y as f32;

            for x in 0..w_line {
                let xf = x as f32;

                // Normal are point in forward side of the plane even on double sided and bi_tangent is assume the statement.
                vertices.push(Vertex {
                    position: [xf * scale_x - width * 0.5, 0.0, yf * scale_y - height * 0.5],
                    tex_coord: [inv_tessellation_width * xf, inv_tessellation_height * yf],
                    normal: NORMAL_FRONT,
                    tangent: TANGENT,
                    bi_tangent: BI_TANGENT,
                });
            }
        }

        for y in 0..tessellation_height {
            for x in 0..tessellation_width {
                indices.push((y * w_line) + x);
                indices.push(((y + 1) * w_line) + x);
                indices.push((y * w_line) + x + 1);

                indices.push(((y + 1) * w_line) + x);
                indices.push(((y + 1) * w_line) + x + 1);
                indices.push((y * w_line) + x + 1);
            }

            //Calculate back face indices if back face is enabled.
            //Two sided
            let trip_count = tessellation_width * plane_instruction as usize;
            for x in 0..trip_count {
                indices.push((y * w_line) + x);
                indices.push((y * w_line) + x + 1);
                indices.push(((y + 1) * w_line) + x);

                indices.push(((y + 1) * w_line) + x);
                indices.push((y * w_line) + x + 1);
                indices.push(((y + 1) * w_line) + x + 1);
            }
        }

        let mesh = Mesh {
            vertices,
            material_id: 0,
            indices,
        };
        Model { meshes: vec![mesh] }
    }
}

#[cfg(test)]
mod test {
    use crate::mesh::primitive::plane::Plane;
    use crate::mesh::{Model, PlaneInstruction};

    #[test]
    fn test() {
        let plane = Plane::new(
            10000.0,
            10000.0,
            10000,
            10000,
            PlaneInstruction::SingleSided,
        );
        let plane_model: Model = plane.into();
        /*for vertices in &plane_model.meshes[0].vertices {
            println!("{:?}", vertices.position);
        }
        println!("{:?}", plane_model.meshes[0].indices);*/
    }
}
