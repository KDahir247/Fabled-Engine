use fabled_math::Matrix4x4;
use fabled_component::{Component, All};

use std::fmt::Display;

#[derive(Copy, Clone)]
pub struct LocalToWorld {
    pub value: Matrix4x4,
}

impl Default for LocalToWorld {
    fn default() -> Self {
        Self {
           value : Matrix4x4::IDENTITY
        }
    }
}

impl Display for LocalToWorld{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"LocalToWorld(\n{}\n)", self.value)
    }
}

impl Component for LocalToWorld{
    type Tracking = All;
}