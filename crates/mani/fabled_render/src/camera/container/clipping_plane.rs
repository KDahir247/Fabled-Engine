pub struct ClippingPlane {
    pub far: f32,
    pub near: f32,
}

impl Default for ClippingPlane {
    fn default() -> Self {
        Self {
            far: 0.1,
            near: 1000.0,
        }
    }
}

impl ClippingPlane {
    pub fn new(far: f32, near: f32) -> Self {
        Self { far, near }
    }
}
