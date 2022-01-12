use anyhow::anyhow;
use fbxcel_dom::v7400::object::property::PropertyHandle;

#[derive(Default, Debug, Copy, Clone)]
pub(crate) struct DecayLightLoader;

impl<'a> fbxcel_dom::v7400::object::property::LoadProperty<'a> for DecayLightLoader {
    type Value = fabled_render::light::DecayType;
    type Error = anyhow::Error;

    fn expecting(&self) -> String {
        "integer i32 as light decay type".into()
    }

    fn load(self, node: &PropertyHandle<'a>) -> Result<Self::Value, Self::Error> {
        let i32_representation = node.load_value(
            fbxcel_dom::v7400::object::property::loaders::PrimitiveLoader::<i32>::new(),
        );

        let result = if let Ok(i32_repr) = i32_representation {
            let decay_type = match i32_repr {
                0 => fabled_render::light::DecayType::None,
                1 => fabled_render::light::DecayType::Linear,
                2 => fabled_render::light::DecayType::Quadratic,
                3 => fabled_render::light::DecayType::Cubic,
                _ => fabled_render::light::DecayType::None,
            };


            Ok(decay_type)
        } else {
            Err(anyhow!(
                "failed to parse decay type as primitive type of i32"
            ))
        };

        result
    }
}
