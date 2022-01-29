use fabled_render::light::{DecayType, LightType};

pub fn load_light_handle(obj_handle: &fbxcel_dom::v7400::object::ObjectHandle) {
    if let fbxcel_dom::v7400::object::TypedObjectHandle::NodeAttribute(
        fbxcel_dom::v7400::object::nodeattribute::TypedNodeAttributeHandle::Light(light_handle),
    ) = obj_handle.get_typed()
    {
        let light_handle = crate::properties::LightProperties::new(&light_handle).unwrap();

        let light_type = light_handle.light_type_or_default().unwrap();

        let light_intensity = light_handle
            .light_intensity_or_default()
            .unwrap_or_default();

        let light_luminous_efficacy = fabled_render::light::Efficacy::INCANDESCENT;

        let [low_lumen, high_lumen, average_lumen] =
            fabled_render::light::watt_to_lumen(light_intensity, light_luminous_efficacy);

        let enabled_light = light_handle.cast_light_or_default().unwrap_or(false);

        let light_color_scalar = f64::from(u8::from(enabled_light));

        let light_color =
            light_handle.light_color_or_default().unwrap_or_default() * light_color_scalar;

        let enabled_shadow = light_handle.cast_shadow_or_default().unwrap_or(false);

        let shadow_color_scalar = f64::from(u8::from(enabled_shadow));

        let shadow_color =
            light_handle.shadow_color_or_default().unwrap_or_default() * shadow_color_scalar;

        let light_decay_type = light_handle
            .light_decay_type_or_default()
            .unwrap_or(DecayType::Linear);

        let light_decay_start = light_handle
            .light_decay_start_or_default()
            .unwrap_or(2000.0);

        match light_type {
            LightType::SpotLight(mut spot_light) => {
                spot_light.intensity = light_intensity;

                // get the inner, outer cone.
            }
            LightType::PointLight(mut point_light) => {
                point_light.intensity = light_intensity;
            }
            LightType::DirectionalLight(mut direction_light) => {
                // todo convert lumen to flux for direction light, but keep it
                // lumens for spot light and point light.
            }
        }
    }
}
