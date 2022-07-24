use crate::light::{
    DirectionalLight, IntensityUnit, LightAppearance, LightType, PointLight, SpotLight,
    TemperatureUnit,
};
use gltf::khr_lights_punctual::{Kind, Light};

// todo this will return and entity identifier that contains both LightType and
// LightAppearance component.
impl From<Light<'_>> for LightType {
    fn from(light: Light<'_>) -> Self {
        let intensity = light.intensity();

        let range = light.range().unwrap_or_default();

        let light_type = match light.kind() {
            Kind::Directional => {
                let directional_light = DirectionalLight {
                    illuminance: intensity,
                };

                LightType::DirectionalLight(directional_light)
            }

            Kind::Point => {
                let point_light =
                    PointLight::new(intensity, IntensityUnit::Candela, 10.0, range, 10.0);

                LightType::PointLight(point_light)
            }

            Kind::Spot {
                inner_cone_angle,
                outer_cone_angle,
            } => {
                let spot_light = SpotLight::new(
                    intensity,
                    IntensityUnit::Candela,
                    range,
                    inner_cone_angle,
                    outer_cone_angle,
                    10.0,
                );

                LightType::SpotLight(spot_light)
            }
        };

        light_type
    }
}
