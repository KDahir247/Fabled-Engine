use fabled_math::Vector3;
use fabled_component::{Component, All};

use std::fmt::Display;

#[derive(Copy, Clone)]
pub struct Scale {
    pub value: Vector3,
}

impl Default for Scale{
    fn default() -> Self {
        Scale { value: Vector3::ONE }
    }
}

impl Display for Scale{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scale({})", self.value)
    }
}

impl Component for Scale{
    type Tracking = All;
}