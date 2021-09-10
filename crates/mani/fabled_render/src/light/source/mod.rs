mod directional_light;
mod point_light;
mod spot_light;


pub use directional_light::*;
pub use point_light::*;
pub use spot_light::*;


#[cfg(test)]
mod data_test {
    use crate::light::{DirectionalLight, PointLight, SpotLight};

    #[test]
    fn data_size() {
        let directional_light_size = std::mem::size_of::<DirectionalLight>();
        println!("{}", directional_light_size);

        let spot_light_size = std::mem::size_of::<SpotLight>();
        println!("{}", spot_light_size);

        let point_light_size = std::mem::size_of::<PointLight>();
        println!("{}", point_light_size);
    }


    #[test]
    fn data_alignment() {
        let directional_light_alignment = std::mem::align_of::<DirectionalLight>();
        println!("{}", directional_light_alignment);

        let spot_light_alignment = std::mem::align_of::<SpotLight>();
        println!("{}", spot_light_alignment);

        let point_light_alignment = std::mem::align_of::<PointLight>();
        println!("{}", point_light_alignment);
    }
}
