use std::fmt::Display;

use fabled_component::{Component, All};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Parent {
    pub value: u64,
}

impl Display for Parent{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parent({})", self.value)
    }
}

impl Component for Parent{
    type Tracking = All;
}