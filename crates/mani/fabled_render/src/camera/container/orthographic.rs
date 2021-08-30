use crate::camera::{ProjectionCoordinate, YAxis};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Orthographic {
    pub right: f32,
    pub left: f32,
    pub top: f32,
    pub bottom: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Default for Orthographic {
    fn default() -> Self {
        Self {
            right: 1.0,
            left: 0.0,
            top: 1.0,
            bottom: 0.0,
            z_near: 0.1,
            z_far: 2000.0,
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct OrthographicOption {
    pub direction: ProjectionCoordinate,
    pub y_axis: YAxis,
}
