use crate::camera::CameraFormat;
use fabled_component::{Component, Modification};
use fabled_math::Vector4;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub struct ViewPort {
    pub rect: Vector4,
}

impl Default for ViewPort {
    fn default() -> ViewPort {
        ViewPort {
            rect: Vector4::set(0.0, 0.0, 1.0, 1.0),
        }
    }
}

impl ViewPort {
    pub fn from_preset(format: CameraFormat) -> ViewPort {
        let resolution = format.resolution_pixel;

        ViewPort::new(Vector4::set(0.0, 0.0, resolution.x(), resolution.y()))
    }

    pub fn new(rect: Vector4) -> ViewPort {
        ViewPort { rect }
    }
}

impl Component for ViewPort {
    type Tracking = Modification;
}

impl Display for ViewPort {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Viewport(\n\tx : {}, y : {}\n\twidth : {}, height : {}\n)",
            self.rect.x(),
            self.rect.y(),
            self.rect.z(),
            self.rect.w(),
        )
    }
}
