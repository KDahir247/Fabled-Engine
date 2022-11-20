use fabled_component::{Component, Modification};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub struct ClippingPlane {
    pub near: f32,
    pub far: f32,
}

impl Default for ClippingPlane {
    fn default() -> ClippingPlane {
        ClippingPlane {
            near: 0.1,
            far: 1000.0,
        }
    }
}

impl ClippingPlane {
    pub fn new(far: f32, near: f32) -> ClippingPlane {
        let near = near.max(0.01);
        let far = far.max(near + 0.1);

        ClippingPlane { near, far }
    }
}

impl Component for ClippingPlane {
    type Tracking = Modification;
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
