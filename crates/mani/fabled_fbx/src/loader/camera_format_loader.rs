use anyhow::anyhow;
use fbxcel_dom::v7400::object::property::PropertyHandle;

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct CameraFormatLoader;

impl<'a> fbxcel_dom::v7400::object::property::LoadProperty<'a> for CameraFormatLoader {
    type Value = fabled_render::camera::CameraFormat;
    type Error = anyhow::Error;

    fn expecting(&self) -> String {
        "integer i32 as camera format type".into()
    }

    fn load(self, node: &PropertyHandle<'a>) -> Result<Self::Value, Self::Error> {
        let i32_representation = node.load_value(
            fbxcel_dom::v7400::object::property::loaders::PrimitiveLoader::<i32>::new(),
        );

        let result = if let Ok(i32_repr) = i32_representation {
            let camera_format = match i32_repr {
                1 => fabled_render::camera::CameraFormat::D1NTSC,
                2 => fabled_render::camera::CameraFormat::NTSC,
                3 => fabled_render::camera::CameraFormat::PAL,
                4 => fabled_render::camera::CameraFormat::D1PAL,
                5 => fabled_render::camera::CameraFormat::HD,
                6 => fabled_render::camera::CameraFormat::R640x480,
                7 => fabled_render::camera::CameraFormat::R320x200,
                8 => fabled_render::camera::CameraFormat::R320x240,
                9 => fabled_render::camera::CameraFormat::R128x128,
                10 => fabled_render::camera::CameraFormat::FullScreen,
                _ => fabled_render::camera::CameraFormat::CustomFormat,
            };

            Ok(camera_format)
        } else {
            Err(anyhow!(
                "failed to parse camera format type as primitive type of i32"
            ))
        };

        result
    }
}
