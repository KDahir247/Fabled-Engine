use crate::camera::{ProjectionCoordinate, YAxis};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Orthographic {
    pub x_mag: f32,
    pub y_mag: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl Default for Orthographic {
    fn default() -> Self {
        Self {
            x_mag: 1.0,
            y_mag: 1.0,
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
