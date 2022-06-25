use crate::camera::{AspectRatio, Fov};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Perspective {
    pub aspect: AspectRatio,
    pub fov: Fov,
}

impl Perspective {
    pub fn new(fov: Fov, aspect: AspectRatio) -> Self {
        Self { fov, aspect }
    }
}

#[cfg(test)]
mod perspective_test {
    // no test needed for perspective, since it very primitive
}
