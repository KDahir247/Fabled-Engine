#[derive(Debug, Default, Copy, Clone)]
pub struct MaterialData {
    // ambient RGB + ambient coefficient
    pub ambient_color: [f32; 3],

    // diffuse RGB + diffuse coefficient
    pub diffuse_color: [f32; 3],

    // specular RGB + specular coefficient
    pub specular_color: [f32; 3],

    // emissive RGB + emissive factor
    pub emissive_color: [f32; 3],

    pub shininess: f32,

    // bump XYZ + bump factor
    pub bump_vector: [f32; 3],

    pub normal_map: [f32; 3],

    // transparent RGB + transparent factor
    pub transparent_color: [f32; 3],

    // displacement RGB + displacement factor
    pub displacement_color: [f32; 3],

    // vector displacement RGB + vector displacement factor
    pub vector_displacement_color: [f32; 3],

    // reflection + reflection factor
    pub reflection: [f32; 3],
}
