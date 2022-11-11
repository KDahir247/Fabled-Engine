use fabled_component::{Component, All};

use std::fmt::Display;

#[derive(Copy, Clone, Default)]
pub struct Frozen {}

impl Display for Frozen{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Frozen()")
    }
}

impl Component for Frozen{
    type Tracking = All;
}