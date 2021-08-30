use crate::camera::{ProjectionCoordinate, YAxis};

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
    pub fovy: f32,
    /// Screen width / Screen height
    pub aspect: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Default for Perspective {
    fn default() -> Self {
        Self {
            fovy: 60.0,
            aspect: 1920.0 / 1080.0,
            z_near: 0.1,
            z_far: 2000.0,
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
