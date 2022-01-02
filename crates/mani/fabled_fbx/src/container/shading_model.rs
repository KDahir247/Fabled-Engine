#[derive(Copy, Clone, Debug)]
pub enum ShaderModel {
    Standard(StandardData),
    Full(FullData),
}


#[derive(Debug, Copy, Clone)]
pub struct StandardData {
    // ambient RGB + ambient coefficient
    pub ambient_color_coe: [f32; 3],

    // diffuse RGB + diffuse coefficient
    pub diffuse_color_coe: [f32; 3],

    // specular RGB + specular coefficient
    pub specular_color_coe: [f32; 3],

    // emissive RGB + emissive factor
    pub emissive_color_factor: [f32; 3],

    pub shininess: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct FullData {
    pub standard_param: StandardData,

    // bump XYZ + bump factor
    pub bump_vector_coe: [f32; 3],

    pub normal_map: [f32; 3],

    // transparent RGB + transparent factor
    pub transparent_color_coe: [f32; 3],

    // displacement RGB + displacement factor
    pub displacement_color_coe: [f32; 3],

    // vector displacement RGB + vector displacement factor
    pub vector_displacement_color_coe: [f32; 3],

    // reflection + reflection factor
    pub reflection_coe: [f32; 3],
}
