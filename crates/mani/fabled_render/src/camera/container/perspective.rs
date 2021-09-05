use crate::camera::{AspectRatio, ClippingPlane, Fov, ProjectionCoordinate, YAxis};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PerspectiveOrientation {
    Standard = 0,
    Reversed = -1,
}

impl Default for PerspectiveOrientation {
    fn default() -> Self {
        Self::Standard
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PerspectiveDistance {
    Standard,
    Infinite,
}

impl Default for PerspectiveDistance {
    fn default() -> Self {
        Self::Standard
    }
}

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

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct PerspectiveOption {
    pub direction: ProjectionCoordinate,
    pub y_axis: YAxis,
    pub orientation: PerspectiveOrientation,
    pub distance: PerspectiveDistance,
}

#[cfg(test)]
mod perspective_test {
    // no test needed for perspective, since it very primitive
}
