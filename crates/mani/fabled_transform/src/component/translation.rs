use fabled_math::Vector3;
use fabled_component::{Component, All};

use std::fmt::Display;

#[derive(Copy, Clone)]
pub struct Translation {
    pub value: Vector3,
}


impl Default for Translation {
    fn default() -> Self {
        Self {
            value : Vector3::ZERO
        }
    }
}

impl Display for Translation{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Translation({})", self.value)
    }
}

impl Component for Translation{
    type Tracking = All;
}
