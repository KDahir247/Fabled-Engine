use anyhow::anyhow;
use fbxcel_dom::v7400::object::property::PropertyHandle;

#[derive(Default, Debug, Copy, Clone)]
pub(crate) struct AspectRatioLoader;

impl<'a> fbxcel_dom::v7400::object::property::LoadProperty<'a> for AspectRatioLoader {
    type Value = fabled_render::camera::AspectRatioMode;
    type Error = anyhow::Error;

    fn expecting(&self) -> String {
        "integer i32 as aspect ratio type".into()
    }

    fn load(self, node: &PropertyHandle<'a>) -> Result<Self::Value, Self::Error> {
        let i32_representation = node.load_value(
            fbxcel_dom::v7400::object::property::loaders::PrimitiveLoader::<i32>::new(),
        );

        let result = if let Ok(i32_representation) = i32_representation {
            let repr = match i32_representation {
                1 => fabled_render::camera::AspectRatioMode::FixedRatio,
                2 => fabled_render::camera::AspectRatioMode::FixedResolution,
                3 => fabled_render::camera::AspectRatioMode::FixedWidth,
                4 => fabled_render::camera::AspectRatioMode::FixedHeight,
                _ => fabled_render::camera::AspectRatioMode::WindowSize,
            };

            Ok(repr)
        } else {
            Err(anyhow!(
                "failed to parse aspect ration type as primitive type of i32"
            ))
        };

        result
    }
}
