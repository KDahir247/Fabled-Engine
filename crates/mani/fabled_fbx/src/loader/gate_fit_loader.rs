use anyhow::anyhow;
use fbxcel_dom::v7400::object::property::PropertyHandle;

#[derive(Default, Debug, Copy, Clone)]
pub(crate) struct CameraGateFitLoader;

impl<'a> fbxcel_dom::v7400::object::property::LoadProperty<'a> for CameraGateFitLoader {
    type Value = fabled_render::camera::GateFit;
    type Error = anyhow::Error;

    fn expecting(&self) -> String {
        "integer i32 as gate fit type".into()
    }

    fn load(self, node: &PropertyHandle<'a>) -> Result<Self::Value, Self::Error> {
        let i32_representation = node.load_value(
            fbxcel_dom::v7400::object::property::loaders::PrimitiveLoader::<i32>::new(),
        );

        let result = if let Ok(i32_repr) = i32_representation {
            let gate_fit = match i32_repr {
                1 => fabled_render::camera::GateFit::Vertical,
                2 => fabled_render::camera::GateFit::Horizontal,
                3 => fabled_render::camera::GateFit::Fill,
                4 => fabled_render::camera::GateFit::Overscan,
                5 => fabled_render::camera::GateFit::Stretch,
                _ => fabled_render::camera::GateFit::None,
            };
            
            Ok(gate_fit)
        } else {
            Err(anyhow!(
                "failed to parse camera gate fit as primitive type of i32"
            ))
        };

        result
    }
}
