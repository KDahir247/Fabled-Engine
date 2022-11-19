use crate::camera::ClippingPlane;
use fabled_math::Vector4;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub struct Orthographic {
    // right left top bot
    pub orientation: Vector4,
    pub depth: ClippingPlane,
}

impl Default for Orthographic {
    fn default() -> Self {
        Self {
            orientation: Vector4::set(1.0, 0.0, 1.0, 0.0),
            depth: Default::default(),
        }
    }
}

impl Orthographic {
    pub fn new(orientation: Vector4, near: f32, far: f32) -> Self {
        Self {
            orientation,
            depth: ClippingPlane::new(far, near),
        }
    }
}
impl Display for Orthographic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Orthographic(\n\tx : {}, y : {}, width : {}, height : {}\n\t{})",
            self.orientation.x(),
            self.orientation.y(),
            self.orientation.z(),
            self.orientation.w(),
            self.depth
        )
    }
}
