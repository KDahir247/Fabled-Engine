// vertical_position get the vertical position relative to angle_rad
// depth_offset get the z position relative to angle_rad

use fabled_math::Vector3;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub struct Oblique {
    // angle_rad, vertical_position, depth_offset.
    pub value: Vector3,
}

impl Default for Oblique {
    fn default() -> Self {
        Self {
            value: Vector3::set(std::f32::consts::FRAC_PI_4, 0.05, 10.0),
        }
    }
}

impl Oblique {
    pub fn new(angle_degree: f32, vertical_position: f32, depth_offset: f32) -> Self {
        let angle = angle_degree.clamp(-180.0, 180.0);
        let vertical = vertical_position.clamp(-1.0, 1.0);

        Self {
            value: Vector3::set(angle.to_radians(), vertical, depth_offset),
        }
    }
}

impl Display for Oblique {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Oblique(\n\tangle radian : {}\n\tvertical position : {}\n\tdepth offset : {})",
            self.value.x(),
            self.value.y(),
            self.value.z(),
        )
    }
}
