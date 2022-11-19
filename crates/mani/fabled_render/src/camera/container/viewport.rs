use crate::camera::ClippingPlane;
use fabled_math::Vector4;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub struct ViewPort {
    pub rect: Vector4,
    pub depth: ClippingPlane,
}

impl Default for ViewPort {
    fn default() -> Self {
        Self {
            rect: Vector4::set(0.0, 0.0, 1.0, 1.0),
            depth: Default::default(),
        }
    }
}

impl ViewPort {
    pub fn new(rect: Vector4, min_depth: f32, max_depth: f32) -> ViewPort {
        let near_depth = min_depth.max(0.01);
        let max_depth = max_depth.max(min_depth + 0.1);

        Self {
            rect,
            depth: ClippingPlane::new(max_depth, near_depth),
        }
    }
}

impl Display for ViewPort {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Viewport(\n\tx : {}, y : {}\n\twidth : {}, height : {}\n)\n{}",
            self.rect.x(),
            self.rect.y(),
            self.rect.z(),
            self.rect.w(),
            self.depth,
        )
    }
}
