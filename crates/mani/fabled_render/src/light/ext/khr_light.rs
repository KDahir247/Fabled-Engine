use crate::light::{
    DirectionalLight, LightAppearance, LightType, LightUnit, PointLight, SpotLight,
};
use gltf::khr_lights_punctual::{Kind, Light};

impl From<gltf::khr_lights_punctual::Light<'_>> for LightType {
    fn from(light: Light<'_>) -> Self {
        // point and spot lights use luminous intensity in candela (lm/sr) while
        // directional lights use illuminance in lux (lm/m^2).
        let intensity = light.intensity();
        // RGB normalized value (0..1) for light's color in linear space.
        let color = light.color();
        // distance cutoff at which the light’s intensity may be considered to have
        // reached zero. Supported only for point and spot lights.
        let range = light.range();

        match light.kind() {
            Kind::Directional => {
                let directional_light = DirectionalLight {
                    illuminance: intensity,
                    appearance: LightAppearance {
                        color,
                        ..Default::default()
                    },
                };

                LightType::DirectionalLight(directional_light)
            }
            Kind::Point => {
                let light_appearance = LightAppearance {
                    color,
                    ..Default::default()
                };

                // since we can use new for point and spot light, since it requires the
                // luminance flux. to get to calculate the luminance intensity,
                // but khr_lights_punctual already returns luminance intensity.
                let mut point_light = PointLight::new(
                    intensity,
                    LightUnit::Candela,
                    10.0,
                    0.0,
                    light_appearance,
                    10.0,
                );

                if let Some(point_range) = range {
                    point_light.range = point_range;
                }

                LightType::PointLight(point_light)
            }
            Kind::Spot {
                inner_cone_angle,
                outer_cone_angle,
            } => {
                let light_appearance = LightAppearance {
                    color,
                    ..Default::default()
                };
                let mut spot_light = SpotLight::new(
                    intensity,
                    LightUnit::Candela,
                    0.0,
                    inner_cone_angle,
                    outer_cone_angle,
                    light_appearance,
                    10.0,
                );

                if let Some(spot_range) = range {
                    spot_light.range = spot_range;
                }

                LightType::SpotLight(spot_light)
            }
        }
    }
}
