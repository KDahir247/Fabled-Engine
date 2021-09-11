use crate::light::{DirectionalLight, LightType, PointLight, SpotLight};
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
                    color,
                };

                LightType::DirectionalLight(directional_light)
            }
            Kind::Point => {
                // todo maybe convert intensity to illuminance in candela

                // since we can use new for point and spot light, since it requires the
                // luminance flux. to get to calculate the luminance intensity,
                // but khr_lights_punctual already returns luminance intensity.
                let mut point_light = PointLight {
                    intensity,
                    color,
                    ..Default::default()
                };

                if let Some(point_range) = range {
                    point_light.range = point_range;
                }
                LightType::PointLight(point_light)
            }
            Kind::Spot {
                inner_cone_angle,
                outer_cone_angle,
            } => {
                let mut spot_light = SpotLight {
                    intensity,
                    inner_cone: inner_cone_angle,
                    outer_cone: outer_cone_angle,
                    color,
                    ..Default::default()
                };

                if let Some(spot_range) = range {
                    spot_light.range = spot_range;
                }

                LightType::SpotLight(spot_light)
            }
        }
    }
}
