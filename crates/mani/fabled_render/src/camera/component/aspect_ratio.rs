use crate::camera::CameraFormat;
use fabled_component::{Component, Modification};
use std::fmt::{Display, Formatter};

// Optional add component if camera is Perspective.

#[derive(Copy, Clone, PartialEq)]
pub struct AspectRatio {
    pub horizontal: f32,
    pub vertical: f32,
}

impl Default for AspectRatio {
    fn default() -> AspectRatio {
        AspectRatio {
            horizontal: 16.0,
            vertical: 9.0,
        }
    }
}

// todo do this tomorrow implementation for mode is not complete..
impl AspectRatio {
    pub fn new(horizontal: f32, vertical: f32) -> AspectRatio {
        let horizontal = horizontal.max(1.0);
        let vertical = vertical.max(1.0);


        AspectRatio {
            horizontal,
            vertical,
        }
    }

    pub const fn from_preset(format: CameraFormat) -> AspectRatio {
        format.aspect_ratio
    }

    pub fn get_aspect(&self) -> f32 {
        self.horizontal / self.vertical
    }
}


impl Component for AspectRatio {
    type Tracking = Modification;
}

impl Display for AspectRatio {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AspectRatio(horizontal : {}, vertical : {})",
            self.horizontal, self.vertical
        )
    }
}
