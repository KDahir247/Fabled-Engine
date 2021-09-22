// This is the default implementation for basic non pbr material.

#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
pub struct StandardMaterial {
    ambient_color: [f32; 3],
    emissive_color: [f32; 3],
    diffuse_color: [f32; 3],
    specular_color: [f32; 3],
    shininess: f32,
    index_of_refraction: f32,
    alpha_threshold: f32,
    unknown_param: f32,
}
