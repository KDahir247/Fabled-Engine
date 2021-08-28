// Used in Obj Wavefront file.

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum IllumModel {
    /// Color on and Ambient off
    Color = 0,
    /// Color on and Ambient on
    ColorAmbient = 1,
    /// Highlight on
    Highlight = 2,
    /// Reflection on and Ray trace on
    ReflectionRayTrace = 3,
    /// Transparency: Glass on, Reflection: Ray trace on
    TransparencyGlassReflectionRayTrace = 4,
    /// Reflection: Fresnel on and Ray trace on
    ReflectionFresnelRayTrace = 5,
    /// Transparency: Refraction on, Reflection: Fresnel off and Ray trace on
    TransparencyRefractionRayTrace = 6,
    /// Transparency: Refraction on, Reflection: Fresnel on and Ray trace on
    TransparencyRefractionReflectionFresnelRayTrace = 7,
    /// Reflection on and Ray trace off
    Reflection = 8,
    /// Transparency: Glass on, Reflection: Ray trace off
    TransparencyGlass = 9,
    /// Casts shadows onto invisible surfaces
    ShadowInvisible = 10,
}
