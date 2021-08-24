pub struct Material {}

//testing struct
#[allow(dead_code)]
pub struct PrototypeMaterial {
    ambient_color: [f32; 4],
    emissive_color: [f32; 4],
    diffuse_color: [f32; 4],
    specular_color: [f32; 4],
    //Reflective value
    reflectance: [f32; 4],

    opacity: f32,
    specular_power: f32,
    index_of_refraction: f32,
    has_ambient_texture: bool,
    has_emissive_texture: bool,
    has_diffuse_texture: bool,
    has_specular_texture: bool,
    has_specular_power_texture: bool,

    has_normal_texture: bool,
    has_bump_texture: bool,
    bump_intensity: f32,

    specular_scale: f32,
    alpha_threshold: f32,
    __padding__: [f32; 2],
}
