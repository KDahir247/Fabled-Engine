pub struct SpotLight {
    pub intensity: f32,
    pub range: f32,
    pub inner_cone: f32,
    pub outer_cone: f32,
    pub color: [f32; 3],
    pub distance_m: f32,
}

impl Default for SpotLight {
    fn default() -> Self {
        unimplemented!()
    }
}

impl SpotLight {
    pub fn new(flux: f32) -> Self {
        unimplemented!()
    }
}
