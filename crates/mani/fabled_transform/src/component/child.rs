use fabled_component::{Component, All};

use std::fmt::Display;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]

pub struct Child {
    pub value: u64,
}

impl Display for Child{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Child({})", self.value)
    }
}

impl Component for Child{
    type Tracking = All;
}