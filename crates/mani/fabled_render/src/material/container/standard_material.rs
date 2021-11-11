// This is the default implementation for basic non pbr material.

#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
pub struct StandardMaterial {
    pub ambient_color: [f32; 3],
    pub diffuse_color: [f32; 3],
    pub specular_color: [f32; 3],
    pub unknown_param: [f32; 3],

    // Shininess, Index of Refraction, Alpha,
    pub factor: [f32; 3],
}
