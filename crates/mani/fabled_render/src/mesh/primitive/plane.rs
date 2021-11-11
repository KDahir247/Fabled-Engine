use crate::mesh::{Mesh, Model, RenderInstruction, Vertex};

// The normal vector of the front face plane
const NORMAL_FRONT: [f32; 3] = [0.0, 1.0, 0.0];

impl Default for RenderInstruction {
    fn default() -> Self {
        RenderInstruction::SingleSided
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    pub tessellation_width: usize,
    pub tessellation_height: usize,
    pub width: f32,
    pub height: f32,
    pub plane_instruction: RenderInstruction,
}

impl Default for Plane {
    fn default() -> Self {
        Self::new(1.0, 1.0, 10, 10, RenderInstruction::SingleSided)
    }
}

impl Plane {
    pub fn new(
        width: f32,
        height: f32,
        mut tessellation_width: usize,
        mut tessellation_height: usize,
        instruction: RenderInstruction,
    ) -> Plane {
        // Sanity check for the size of tesselation_width and tessellation_height.
        // Prevent increasing tessellation_height and tessellation_width intentional or
        // unintentional to the point where it causes repeating KiPageFault and
        // MmAccessFault resulting in cache thrashing or memory can't be
        // allocated error.
        //
        // Unity's plane mesh width and height tessellation caps at 10 width x 10
        // height.
        //
        // If the end user want more tesselation on the plane create another plane and
        // align it with the previous plane.

        tessellation_width = tessellation_width.min(10);
        tessellation_height = tessellation_height.min(10);

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
            tessellation_width * (tessellation_height << 1) * 3 * double_sided_remap as usize;

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

                // Normal are point in forward side of the plane even on double sided and
                // bi_tangent is assume the statement.
                vertices.push(Vertex {
                    position: [xf * scale_x - width * 0.5, 0.0, yf * scale_y - height * 0.5],
                    tex_coord: [inv_tessellation_width * xf, inv_tessellation_height * yf],
                    normal: NORMAL_FRONT,
                    tangent: [0.0; 4],
                    bi_tangent: [0.0; 4],
                });
            }
        }

        for y in 0..tessellation_height {
            let seg_0 = y * w_line;
            let seg_1 = (y + 1) * w_line;

            for x in 0..tessellation_width {
                let res_0 = seg_0 + x;
                let res_1 = seg_1 + x;

                indices.push(res_0);
                indices.push(res_1);
                indices.push(res_0 + 1);

                indices.push(res_1);
                indices.push(res_1 + 1);
                indices.push(res_0 + 1);
            }

            // Calculate back face indices if back face is enabled.
            // Two sided
            for x in 0..tessellation_width * plane_instruction as usize {
                let res_0 = seg_0 + x;
                let res_1 = seg_1 + x;


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
            indices : indices.into(),
        };

        Model { meshes: vec![mesh] }
    }
}

#[cfg(test)]
mod test {
    use crate::mesh::primitive::plane::Plane;
    use crate::mesh::Model;
    use crate::mesh::RenderInstruction;

    #[test]
    fn test() {
        let plane = Plane::new(3.0, 3.0, 10, 10, RenderInstruction::DoubleSided);
        let plane_model: Model = plane.into();
        for vertex in &plane_model.meshes[0].vertices {
            println!(
                "new Vector3({}f, {}f, {}f),",
                vertex.normal[0], vertex.normal[1], vertex.normal[2]
            );
        }
        println!("{:?}", plane_model.meshes[0].indices);
    }
}
