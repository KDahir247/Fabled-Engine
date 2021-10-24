// This is the default implementation for basic pbr material.

#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
pub struct PBRStandardMaterial {
    albedo: [f32; 3],
    metallic: f32,
    roughness: f32,
    index_of_refraction: f32,
    ambient_occlusion: f32,
    emmisive: f32,
}
