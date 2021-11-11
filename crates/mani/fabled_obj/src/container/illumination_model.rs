#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum IlluminationModel {
    None = 0,
    /// Color on and Ambient off
    Color = 1,
    /// Color on and Ambient on
    ColorAmbient = 2,
    /// Highlight on
    Highlight = 3,
    /// Reflection on and Ray trace on
    ReflectionRayTrace = 4,
    /// Transparency: Glass on, Reflection: Ray trace on
    TransparencyGlassReflectionRayTrace = 5,
    /// Reflection: Fresnel on and Ray trace on
    ReflectionFresnelRayTrace = 6,
    /// Transparency: Refraction on, Reflection: Fresnel off and Ray trace on
    TransparencyRefractionRayTrace = 7,
    /// Transparency: Refraction on, Reflection: Fresnel on and Ray trace on
    TransparencyRefractionReflectionFresnelRayTrace = 8,
    /// Reflection on and Ray trace off
    Reflection = 9,
    /// Transparency: Glass on, Reflection: Ray trace off
    TransparencyGlass = 10,
    /// Casts shadows onto invisible surfaces
    ShadowInvisible = 11,
}

impl From<Option<u8>> for IlluminationModel {
    fn from(value: Option<u8>) -> Self {
        match value.unwrap_or_default() {
            0 => Self::None,
            1 => Self::Color,
            2 => Self::ColorAmbient,
            3 => Self::Highlight,
            4 => Self::ReflectionRayTrace,
            5 => Self::TransparencyGlassReflectionRayTrace,
            6 => Self::ReflectionFresnelRayTrace,
            7 => Self::TransparencyRefractionRayTrace,
            8 => Self::TransparencyRefractionReflectionFresnelRayTrace,
            9 => Self::Reflection,
            10 => Self::TransparencyGlass,
            11 => Self::ShadowInvisible,
            _ => Self::None,
        }
    }
}


#[cfg(test)]
mod illumination_model_test {
    use crate::IlluminationModel;

    #[test]
    fn illumination_from() {
        let illum_val: Option<u8> = Some(11);

        let illumination: IlluminationModel = illum_val.into();
        assert_eq!(illumination, IlluminationModel::ShadowInvisible);
    }


    #[test]
    fn illumination_invalid() {
        let illum_val: Option<u8> = None;

        let illumination: IlluminationModel = illum_val.into();
        assert_eq!(illumination, IlluminationModel::None);

        let invalid_illum_val = Some(17);

        let illumination: IlluminationModel = invalid_illum_val.into();
        assert_eq!(illumination, IlluminationModel::None);
    }
}
