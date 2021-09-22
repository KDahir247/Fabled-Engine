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
        assert_eq!(point_light_size & (point_light_size - 1), 0);
    }


    #[test]
    fn data_alignment() {
        let directional_light_alignment = std::mem::align_of::<DirectionalLight>();
        assert_eq!(
            directional_light_alignment & (directional_light_alignment - 1),
            0
        );

        let spot_light_alignment = std::mem::align_of::<SpotLight>();
        assert_eq!(spot_light_alignment & (spot_light_alignment - 1), 0);

        let point_light_alignment = std::mem::align_of::<PointLight>();
        assert_eq!(point_light_alignment & (point_light_alignment - 1), 0);
    }
}
