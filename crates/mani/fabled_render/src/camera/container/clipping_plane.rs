use fabled_math::Vector2;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub struct ClippingPlane {
    pub depth: Vector2,
}

impl Default for ClippingPlane {
    fn default() -> Self {
        Self {
            depth: Vector2::set(0.1, 1000.0),
        }
    }
}

impl ClippingPlane {
    pub fn new(far: f32, near: f32) -> Self {
        let near = near.max(0.01);
        let far = far.max(near + 0.1);

        Self {
            depth: Vector2::set(near, far),
        }
    }
}


impl Display for ClippingPlane {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ClippingPlane(near : {}, far : {})",
            self.depth.x(),
            self.depth.y()
        )
    }
}
