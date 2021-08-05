use crate::mesh::Model;

#[allow(dead_code)]
const PI_DIV_180: f32 = 0.017_453_292; //excessive precision truncation. (0.01745329251994329576923690768489)]

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

        //Center vertex
        println!("{}", center);
        //normal is pointing downwards.
        for i in 0..tessellation_slice {
            let (rad_sin, rad_cos) = (angle_inc * i as f32).sin_cos();

            let tex_coord = glam::vec2(i as f32 / tessellation, 1. - (i as f32 / tessellation));
            let vertex = center + (basis * rad_cos + forward_dir * rad_sin) * radius;
            let normal = glam::vec3a(slope_cos * rad_cos, slope_sin, slope_cos * rad_sin);

            println!("vertex {}", vertex);
            println!("normal {}", normal);
            println!("tex_coord {} \n", tex_coord);
        }
        // Apex vertex
        println!("{}", apex_position);
        //for the last element we know that will be the apex  of the cone so we can multiple by height.
        Cone {}
    }
}

#[cfg(test)]
mod test {
    use crate::mesh::primitive::cone::Cone;

    #[test]
    fn test() {
        let x = Cone::new(1.0, 33, 2., [0.0, 2.0, 0.0]);
    }
}
