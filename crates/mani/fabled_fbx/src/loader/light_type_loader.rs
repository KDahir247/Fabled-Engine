use anyhow::anyhow;
use fbxcel_dom::v7400::object::property::PropertyHandle;

#[derive(Default, Debug, Copy, Clone)]
pub(crate) struct LightTypeLoader;

impl<'a> fbxcel_dom::v7400::object::property::LoadProperty<'a> for LightTypeLoader {
    type Value = fabled_render::light::LightType;
    type Error = anyhow::Error;

    fn expecting(&self) -> String {
        "integer i32 as light type".into()
    }

    fn load(self, node: &PropertyHandle<'a>) -> Result<Self::Value, Self::Error> {
        let i32_representation = node.load_value(
            fbxcel_dom::v7400::object::property::loaders::PrimitiveLoader::<i32>::new(),
        );

        let result = if let Ok(i32_repr) = i32_representation {
            let light_type = match i32_repr {
                0 => fabled_render::light::LightType::PointLight(
                    fabled_render::light::PointLight::default(),
                ),
                2 => fabled_render::light::LightType::SpotLight(
                    fabled_render::light::SpotLight::default(),
                ),
                _ => fabled_render::light::LightType::DirectionalLight(
                    fabled_render::light::DirectionalLight::default(),
                ),
            };

            Ok(light_type)
        } else {
            Err(anyhow!(
                "failed to parse light type as primitive typ of i32"
            ))
        };

        result
    }
}
