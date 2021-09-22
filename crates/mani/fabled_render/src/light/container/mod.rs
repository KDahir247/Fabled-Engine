pub use appearance::*;
pub use attenuation::*;
pub use light::*;
pub use unit_type::*;

mod appearance;
mod attenuation;
mod light;
mod unit_type;

#[cfg(test)]
mod data_test {
    use crate::light::{Attenuation, LightAppearance, LightType, LightUnit, TemperatureUnit};

    #[test]
    fn data_size() {
        let appearance_size = std::mem::size_of::<LightAppearance>();
        assert_eq!(appearance_size & (appearance_size - 1), 0);

        let attenuation_size = std::mem::size_of::<Attenuation>();
        assert_eq!(attenuation_size & (attenuation_size - 1), 0);

        let light_size = std::mem::size_of::<LightType>();
        println!("{}", light_size);

        let unit_type_size = std::mem::size_of::<LightUnit>();
        println!("{}", unit_type_size);

        let temperature_size = std::mem::size_of::<TemperatureUnit>();
        assert_eq!(temperature_size & (temperature_size - 1), 0);
    }


    #[test]
    fn data_alignment() {
        let appearance_alignment = std::mem::align_of::<LightAppearance>();
        assert_eq!(appearance_alignment & (appearance_alignment - 1), 0);

        let attenuation_alignment = std::mem::align_of::<Attenuation>();
        assert_eq!(attenuation_alignment & (attenuation_alignment - 1), 0);

        let light_alignment = std::mem::align_of::<LightType>();
        assert_eq!(light_alignment & (light_alignment - 1), 0);

        let unit_type_alignment = std::mem::align_of::<LightUnit>();
        assert_eq!(unit_type_alignment & (unit_type_alignment - 1), 0);

        let temperature_alignment = std::mem::align_of::<TemperatureUnit>();
        assert_eq!(temperature_alignment & (temperature_alignment - 1), 0);
    }
}
