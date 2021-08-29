#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PerspectiveOrientation {
    Standard,
    Reversed,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PerspectiveDistance {
    Standard,
    Infinite,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Perspective {
    fovy: f32,
    /// Screen width / Screen height
    aspect: f32,
    z_near: f32,
    z_far: f32,
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
