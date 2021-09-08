use crate::camera::{AspectRatio, ClippingPlane, Fov};


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Perspective {
    pub fov: Fov,
    pub aspect: AspectRatio,
    pub clipping: ClippingPlane,
}

impl Default for Perspective {
    fn default() -> Self {
        Self {
            fov: Fov::default(),
            aspect: AspectRatio::default(),
            clipping: ClippingPlane::default(),
        }
    }
}

impl Perspective {
    pub fn new(fov: Fov, aspect: AspectRatio, z_far: f32, z_near: f32) -> Self {
        Self {
            fov,
            aspect,
            clipping: ClippingPlane::new(z_far, z_near),
        }
    }
}

#[cfg(test)]
mod perspective_test {
    // no test needed for perspective, since it very primitive
}
