use std::fmt::Display;

use fabled_component::{Component, All};
use fabled_math::Quaternion;

#[derive(Copy, Clone)]
pub struct Rotation {
    pub value: Quaternion,
}

impl Default for Rotation {
    fn default() -> Self {
        Self {
            value : Quaternion::IDENTITY
        }
    }
}

impl Display for Rotation{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rotation({})", self.value)
    }
}

impl Component for Rotation{
    type Tracking = All;
}