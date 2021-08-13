use crate::mesh::util::min_ss;
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
   4) the maximum number of tessellation is 10x10.
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
#[repr(align(16))]
pub struct Plane {
    pub width: f32,  // todo these are size so it can the data normalized.
    pub height: f32, // todo these are size so it can the data normalized.
    pub tessellation_width: u8,
    pub tessellation_height: u8,
    pub plane_instruction: PlaneInstruction,
}

impl Default for Plane {
    fn default() -> Self {
        Self::new(1.0, 1.0, 10, 10, PlaneInstruction::SingleSided)
    }
}

impl Plane {
    pub fn new(
        width: f32,
        height: f32,
        mut tessellation_width: u8,
        mut tessellation_height: u8,
        instruction: PlaneInstruction,
    ) -> Plane {
        /*
            Sanity check for the size of tesselation_width and tessellation_height.
            Prevent increasing tessellation_height and tessellation_width intentional or unintentional
            to the point where it causes repeating KiPageFault and MmAccessFault resulting in
            cache thrashing or memory can't be allocated error.

            Unity's plane mesh width and height tessellation caps at 10 width x 10 height.

            If the end user want more tesselation on the plane create another plane and align it with the
            previous plane.
        */

        tessellation_width = min_ss(tessellation_width as f32, 10.0) as u8;
        tessellation_height = min_ss(tessellation_height as f32, 10.0) as u8;

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
        let double_sided_remap = plane_instruction as u8 + 1;
        let num_triangles =
            (tessellation_width * tessellation_height * 6 * double_sided_remap) as usize;
        let num_vertices = (w_line * h_line) as usize;

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
            let seg_0 = y * w_line;
            let seg_1 = (y + 1) * w_line;

            for x in 0..tessellation_width {
                let res_0 = (seg_0 + x) as usize;
                let res_1 = (seg_1 + x) as usize;

                indices.push(res_0);
                indices.push(res_1);
                indices.push(res_0 + 1);

                indices.push(res_1);
                indices.push(res_1 + 1);
                indices.push(res_0 + 1);
            }

            //Calculate back face indices if back face is enabled.
            //Two sided
            for x in 0..tessellation_width * plane_instruction as u8 {
                let res_0 = (seg_0 + x) as usize;
                let res_1 = (seg_1 + x) as usize;

                indices.push(res_0);
                indices.push(res_0 + 1);
                indices.push(res_1);

                indices.push(res_1);
                indices.push(res_0 + 1);
                indices.push(res_1 + 1);
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
        let plane = Plane::new(1.0, 1.0, 5, 5, PlaneInstruction::SingleSided);
        let plane_model: Model = plane.into();
        for vertices in &plane_model.meshes[0].vertices {
            println!("{:?}", vertices.position);
        }
        println!("{:?}", plane_model.meshes[0].indices);
    }
}
