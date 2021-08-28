#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Orthographic {
    pub x_mag: f32,
    pub y_mag: f32,
    pub z_near: f32,
    pub z_far: f32,
}
