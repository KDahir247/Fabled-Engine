use anyhow::anyhow;
use fbxcel_dom::v7400::object::property::PropertyHandle;

#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct ApertureFormatLoader;

impl<'a> fbxcel_dom::v7400::object::property::LoadProperty<'a> for ApertureFormatLoader {
    type Value = fabled_render::camera::Aperture;
    type Error = anyhow::Error;

    fn expecting(&self) -> String {
        "integer i32 as camera aperture type".into()
    }

    fn load(self, node: &PropertyHandle<'a>) -> Result<Self::Value, Self::Error> {
        let i32_representation = node.load_value(
            fbxcel_dom::v7400::object::property::loaders::PrimitiveLoader::<i32>::new(),
        );

        let result = if let Ok(i32_repr) = i32_representation {
            let aperture = match i32_repr {
                1 => fabled_render::camera::Aperture::R16MM_THEATRICAL,
                2 => fabled_render::camera::Aperture::SUPER_16MM,
                3 => fabled_render::camera::Aperture::R35MM_ACADEMY,
                4 => fabled_render::camera::Aperture::R35MM_TECHNISCOPE,
                5 => fabled_render::camera::Aperture::R35MM_2PERF,
                6 => fabled_render::camera::Aperture::R35MM_185_PROJECTION,
                7 => fabled_render::camera::Aperture::R35MM_ANAMORPHIC,
                8 => fabled_render::camera::Aperture::R70MM,
                9 => fabled_render::camera::Aperture::VISTA_VISION,
                10 => fabled_render::camera::Aperture::DYNA_VISION,
                11 => fabled_render::camera::Aperture::IMAX,
                _ => fabled_render::camera::Aperture::default(),
            };

            Ok(aperture)
        } else {
            Err(anyhow!(
                "failed to parse camera aperture as a primitive type of i32"
            ))
        };

        result
    }
}


#[derive(Default, Debug, Clone, Copy)]
pub(crate) struct ApertureModeLoader;

impl<'a> fbxcel_dom::v7400::object::property::LoadProperty<'a> for ApertureModeLoader {
    type Value = fabled_render::camera::ApertureMode;
    type Error = anyhow::Error;

    fn expecting(&self) -> String {
        "integer i32 as camera aperture mode".into()
    }

    fn load(self, node: &PropertyHandle<'a>) -> Result<Self::Value, Self::Error> {
        let i32_representation = node.load_value(
            fbxcel_dom::v7400::object::property::loaders::PrimitiveLoader::<i32>::new(),
        );

        let result = if let Ok(i32_repr) = i32_representation {
            let aperture_mode = match i32_repr {
                0 => fabled_render::camera::ApertureMode::HorizontalAndVertical,
                1 => fabled_render::camera::ApertureMode::Horizontal,
                3 => fabled_render::camera::ApertureMode::FocalLength,
                _ => fabled_render::camera::ApertureMode::Vertical,
            };

            Ok(aperture_mode)
        } else {
            Err(anyhow!(
                "failed to parse camera aperture mode as a primitive type of i32"
            ))
        };

        result
    }
}
