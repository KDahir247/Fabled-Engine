use crate::camera::{AspectRatio, ClippingPlane, Fov};
use std::fmt::{Display, Formatter};

#[derive(Default, Copy, Clone, PartialEq)]
pub struct Perspective {
    pub aspect: AspectRatio,
    pub depth: ClippingPlane,
    pub fov: Fov,
}

impl Perspective {
    pub fn new(fov: Fov, depth: ClippingPlane, aspect: AspectRatio) -> Self {
        Self { fov, aspect, depth }
    }
}

impl Display for Perspective {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Perspective(\n\t{}\n\t{},\n\t{}\n)",
            self.fov, self.aspect, self.depth
        )
    }
}
