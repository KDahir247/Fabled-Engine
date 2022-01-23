use anyhow::anyhow;
use fbxcel_dom::v7400::object::property::PropertyHandle;

#[derive(Default, Debug, Copy, Clone)]
pub(crate) struct CameraProjectionTypeLoader;

impl<'a> fbxcel_dom::v7400::object::property::LoadProperty<'a> for CameraProjectionTypeLoader {
    type Value = fabled_render::camera::Projection;
    type Error = anyhow::Error;

    fn expecting(&self) -> String {
        "integer i32 as camera projection type".into()
    }

    fn load(self, node: &PropertyHandle<'a>) -> Result<Self::Value, Self::Error> {
        let i32_representation = node.load_value(
            fbxcel_dom::v7400::object::property::loaders::PrimitiveLoader::<i32>::new(),
        );

        let result = if let Ok(i32_repr) = i32_representation {
            let projection = match i32_repr {
                1 => fabled_render::camera::Projection::Orthographic(
                    fabled_render::camera::Orthographic::default(),
                ),
                _ => fabled_render::camera::Projection::Perspective(
                    fabled_render::camera::Perspective::default(),
                ),
            };
            Ok(projection)
        } else {
            Err(anyhow!(
                "failed to parse camera projection as primitive type of i32"
            ))
        };

        result
    }
}
