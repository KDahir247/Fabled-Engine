use crate::camera::ClippingPlane;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(align(16))]
pub struct Orthographic {
    pub clipping: ClippingPlane,
    pub right: f32,
    pub left: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Default for Orthographic {
    fn default() -> Self {
        Self {
            right: 1.0,
            left: 0.0,
            top: 1.0,
            bottom: 0.0,
            clipping: ClippingPlane::default(),
        }
    }
}

impl Orthographic {
    pub fn new(right: f32, left: f32, top: f32, bottom: f32, z_near: f32, z_far: f32) -> Self {
        Self {
            right,
            left,
            top,
            bottom,
            clipping: ClippingPlane::new(z_far, z_near),
        }
    }
}

#[cfg(test)]
mod orthographic_test {
    // no test needed for orthographic, since it very primitive
}
