#[allow(dead_code)]
const PI_DIV_180: f32 = 0.017_453_292; //excessive precision truncation. (0.01745329251994329576923690768489)]
                                       //blender has an normal offset of 0.0596445 of z
                                       //normal offset of 0.0644555 of x
                                       //normal offset of -0.0017136 of y
#[derive(Debug)]
pub struct Cone {
    //model: Model,
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
        let mut vertex_buffer = Vec::new();

        //We can't technically have a cone with less than three tessellation slice for the base. right?
        tessellation_slice = tessellation_slice.max(3);

        let tessellation: f32 = tessellation_slice as f32;
        let apex_position = glam::Vec3A::from_slice(&apex_position);
        let base_to_apex_dir = (apex_position - glam::Vec3A::ZERO).normalize(); // hard coded. Vec3A::Zero should be base position.

        let center = apex_position + (-base_to_apex_dir * height);
        let basis = glam::vec3a(1.0, 0.0, 0.0); //hard coded
        let forward_dir = basis.cross(base_to_apex_dir);

        let angle_inc = 360.0 / tessellation * PI_DIV_180;
        let slant_height = (radius * radius + height * height).sqrt();
        //let cone_angle = (radius / height).atan(); //slower than slant_height calculation

        let slope_sin = radius / slant_height; //cone_angle.sin();
        let slope_cos = height / slant_height; //cone_angle.cos();

        //Calculate the Cone
        let intro_vertex = center + (basis) * radius;
        vertex_buffer.push(intro_vertex);
        // Cone vertex
        for cone_index in 1..tessellation_slice {
            let (rad_sin, rad_cos) = (angle_inc * cone_index as f32).sin_cos();
            let vertex = center + (basis * rad_cos + forward_dir * rad_sin) * radius;

            let normal = glam::vec3a(slope_cos * rad_cos, slope_sin, slope_cos * rad_sin);

            vertex_buffer.push(apex_position);
            vertex_buffer.push(vertex);
            vertex_buffer.push(vertex);
        }

        vertex_buffer.push(apex_position);
        vertex_buffer.push(vertex_buffer[0]);

        //

        for base_index in 0..tessellation_slice {
            let (rad_sin, rad_cos) = (angle_inc * base_index as f32).sin_cos();
            let vertex = center + (basis * rad_cos + forward_dir * rad_sin) * radius;

            // calculate the normal
            // calculate the tangent
            // calculate the bi-normal

            vertex_buffer.push(vertex);
        }

        println!("{:?}", vertex_buffer);

        Cone {}
    }
}

#[cfg(test)]
mod test {
    use crate::mesh::primitive::cone::Cone;

    #[test]
    fn test() {
        let x = Cone::new(1.0, 6, 2., [0.0, 1.0, 0.0]);
    }
}
