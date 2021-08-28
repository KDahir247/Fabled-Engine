#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Perspective {
    fovy: f32,
    aspect: f32,
    z_near: f32,
    z_far: f32,
}
