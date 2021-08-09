use crate::mesh::{Mesh, Model, Vertex};

#[allow(dead_code)]
const PI_DIV_180: f32 = 0.017_453_292;

#[repr(C, align(16))]
#[derive(Debug)]
struct ConeData {
    pub position: [glam::Vec3A; 2],
    pub normal: [glam::Vec3A; 2],
    pub tangent: [glam::Vec4; 2],
    pub bi_tangent: [glam::Vec4; 2],
    pub uv: [glam::Vec2; 2],
}

#[allow(dead_code)]
const CONE_DATA: ConeData = ConeData {
    position: [
        glam::const_vec3a!([0.0, 1.0, 0.0]), // Apex position
        glam::const_vec3a!([0.0, 0.0, 0.0]), // Base-Center position
    ],
    normal: [
        glam::const_vec3a!([0.0, 1.0, 0.0]),  // Apex normal
        glam::const_vec3a!([0.0, -1.0, 0.0]), // Base-Center normal
    ],
    tangent: [
        glam::const_vec4!([-1.0, 0.0, 0.0, 1.0]), // Apex tangent
        glam::const_vec4!([-1.0, 0.0, 0.0, 1.0]), // Base-Center tangent
    ],
    bi_tangent: [
        glam::const_vec4!([0.0, 0.0, 1.0, 1.0]),  // Apex bi tangent
        glam::const_vec4!([0.0, 0.0, -1.0, 1.0]), // Base-Center bi tangent
    ],
    uv: [
        glam::const_vec2!([0.0, 1.0]),  // Apex uv
        glam::const_vec2!([0.0, -1.0]), // Base-Center uv
    ],
};

#[derive(Debug)]
pub struct Cone {
    model: Model,
}

impl Default for Cone {
    fn default() -> Self {
        Self::new(5.0, 1, 2., [0., 2., 0.])
    }
}

impl Cone {
    pub fn new(
        radius: f32,
        mut tessellation_slice: usize,
        height: f32,
        apex_position: [f32; 3],
    ) -> Cone {
        let mut indices = Vec::with_capacity(tessellation_slice * 6);
        let mut vertex_buffer: Vec<Vertex> = Vec::with_capacity(tessellation_slice + 2);

        //We can't technically have a cone with less than three tessellation slice for the base. right?
        tessellation_slice = tessellation_slice.max(3);

        let tessellation: f32 = tessellation_slice as f32;
        let apex_position = glam::Vec3A::from_slice(&apex_position);

        let base_to_apex_dir = (apex_position - glam::Vec3A::ZERO).normalize(); // hard coded. Vec3A::Zero should be base position.
        let forward_dir = glam::Vec3A::X.cross(base_to_apex_dir).normalize();

        let center = apex_position + (-base_to_apex_dir * height);

        let angle_inc = 360.0 / tessellation * PI_DIV_180;
        let slant_height = (radius * radius + height * height).sqrt();

        let slope_sin = radius / slant_height;
        let slope_cos = height / slant_height;

        // Apex Vertex
        vertex_buffer.push(Vertex {
            position: apex_position.to_array(),
            tex_coord: [0.0, 1.0],
            normal: [0.0, 1.0, 0.0],
            tangent: [-1.0, 0.0, 0.0, 1.0],
            bi_tangent: [0.0, 0.0, 1.0, 1.0],
        });

        // Base Center Vertex
        vertex_buffer.push(Vertex {
            position: center.to_array(),
            tex_coord: [0.0, -1.0],
            normal: [0.0, -1.0, 0.0],
            tangent: [-1.0, 0.0, 0.0, 1.0],
            bi_tangent: [0.0, 0.0, -1.0, 1.0],
        });

        // Cone vertex
        for side in 0..=tessellation_slice {
            let side = side as f32;
            let (rad_sin, rad_cos) = (angle_inc * side).sin_cos();

            let vertex = center + (glam::Vec3A::X * rad_cos + forward_dir * rad_sin) * radius;

            let normal =
                glam::vec3a(slope_cos * rad_cos, slope_sin, slope_cos * rad_sin).normalize();

            let tangent = vertex.normalize().cross(glam::Vec3A::Y);

            let bi_tangent = normal.cross(tangent);

            vertex_buffer.push(Vertex {
                position: vertex.to_array(),
                tex_coord: [side / tessellation, 0.0],
                normal: normal.to_array(),
                tangent: tangent.extend(1.0).to_array(),
                bi_tangent: bi_tangent.extend(1.0).to_array(),
            });
        }

        //indices
        const TOP: usize = 0;
        const BOTTOM: usize = 1;

        for point in 2..tessellation_slice + 2 {
            let left = point + 1;
            let right = point;

            indices.push(TOP);
            indices.push(left);
            indices.push(right);

            indices.push(BOTTOM);
            indices.push(right);
            indices.push(left);
        }

        let mesh = Mesh {
            vertices: vertex_buffer,
            material_id: 0,
            indices,
        };

        Cone {
            model: Model { meshes: vec![mesh] },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::mesh::primitive::cone::Cone;

    #[test]
    fn test() {
        Cone::new(5.0, 12, 2., [0.0, 1.0, 0.0]);
    }
}
