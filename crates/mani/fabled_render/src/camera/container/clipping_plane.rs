#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ClippingPlane {
    pub far: f32,
    pub near: f32,
}

impl Default for ClippingPlane {
    fn default() -> Self {
        Self {
            far: 1000.,
            near: 0.1,
        }
    }
}

impl ClippingPlane {
    pub fn new(far: f32, near: f32) -> Self {
        let near = near.max(0.01);

        Self { far, near }
    }
}
