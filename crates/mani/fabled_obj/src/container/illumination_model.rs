use std::convert::TryFrom;
// Used in Obj Wavefront file.

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
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
        // todo if statement to check if value is some or less than or equal to 11 if so
        // transmute otherwise error
        let z: IlluminationModel = unsafe { std::mem::transmute(value.unwrap()) };
        z
    }
}


#[cfg(test)]
mod illumination_model_test {
    use crate::IlluminationModel;

    #[test]
    fn illumination_from() {
        let var_a: Option<u8> = Some(11);

        let illumination: IlluminationModel = var_a.into();

        println!("{:?}", illumination);
    }
}
